//! Helper module to facilitate integration with tokio

#![allow(missing_docs)]

use std::{
    ffi::{c_int, CString},
    future::Future,
    os::fd::RawFd,
    sync::mpsc,
};

use tokio::{sync::mpsc as tokio_mpsc, task::LocalSet};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use tracing::{debug, info};

use crate::event::{AsRawEvent, KeyState, KeyboardEvent};
use crate::{Error, Libinput, Result};

#[derive(Debug, Clone)]
pub struct Handle {
    signal_sender: tokio_mpsc::UnboundedSender<LibinputSignal>,
}

impl Handle {
    pub fn new<O, OFut, C, CFut>(
        open: O,
        close: C,
        seat_name: CString,
    ) -> Result<(Self, UnboundedReceiverStream<Result<Event, crate::Error>>)>
    where
        O: Fn(CString) -> OFut + Send + 'static,
        OFut: Future<Output = RawFd> + Send + 'static,
        C: Fn(RawFd) -> CFut + Send + 'static,
        CFut: Future + Send + 'static,
    {
        let (ask_sx, respond_rx) = {
            let (ask_sx, ask_rx) = tokio_mpsc::unbounded_channel::<CString>();
            let (respond_sx, respond_rx) = mpsc::channel::<c_int>();

            let mut ask_rx = UnboundedReceiverStream::new(ask_rx);

            tokio::spawn(async move {
                while let Some(path) = ask_rx.next().await {
                    let _ = respond_sx.send(open(path).await);
                }
            });

            (ask_sx, respond_rx)
        };

        let close_sx = {
            let (close_sx, close_rx) = tokio_mpsc::unbounded_channel::<c_int>();

            let mut close_rx = UnboundedReceiverStream::new(close_rx);

            tokio::spawn(async move {
                while let Some(fd) = close_rx.next().await {
                    close(fd).await;
                }
            });

            close_sx
        };

        let (rx, signal_sender) = spawn_libinput_task(seat_name, ask_sx, close_sx, respond_rx)?;

        let stream = UnboundedReceiverStream::new(rx);

        Ok((Self { signal_sender }, stream))
    }

    pub fn shutdown(&self) -> Result<()> {
        self.signal_sender.send(LibinputSignal::Shutdown).unwrap(); // FIXME: handle errors

        Ok(())
    }

    pub fn suspend(&self) -> Result<()> {
        self.signal_sender.send(LibinputSignal::Suspend).unwrap(); // FIXME: handle errors

        Ok(())
    }

    pub fn resume(&self) -> Result<()> {
        self.signal_sender.send(LibinputSignal::Resume).unwrap(); // FIXME: handle errors

        Ok(())
    }
}

#[derive(Debug)]
pub struct Event {
    pub name: &'static str,
    pub event_type: EventType,
    pub device_name: String,
}

#[derive(Debug)]
pub enum EventType {
    Keyboard { key: u32, state: KeyState },
    Unknown,
}

impl From<&crate::Event> for EventType {
    fn from(value: &crate::Event) -> Self {
        match value {
            crate::Event::Keyboard(KeyboardEvent::Key(event)) => EventType::Keyboard {
                key: event.key(),
                state: event.key_state(),
            },
            _ => EventType::Unknown,
        }
    }
}

enum LibinputSignal {
    Shutdown,
    Suspend,
    Resume,
}

fn spawn_libinput_task(
    seat_name: CString,
    ask_sx: tokio_mpsc::UnboundedSender<CString>,
    close_sx: tokio_mpsc::UnboundedSender<i32>,
    respond_rx: mpsc::Receiver<i32>,
) -> Result<(
    tokio_mpsc::UnboundedReceiver<Result<Event>>,
    tokio_mpsc::UnboundedSender<LibinputSignal>,
)> {
    let (event_sx, event_rx) = tokio_mpsc::unbounded_channel();
    let (signal_sx, mut signal_rx) = tokio_mpsc::unbounded_channel();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    std::thread::spawn(move || {
        let local = LocalSet::new();

        local.spawn_local(async move {
            info!("Creating libinput object");

            let mut libinput = Libinput::new(
                move |path, _| {
                    debug!("Opening fd at path {}", path.to_string_lossy());
                    ask_sx.send(path.to_owned()).unwrap();
                    let res = respond_rx.recv().unwrap();

                    Ok(res)
                },
                move |fd| {
                    debug!("Closing fd: {fd}");
                    let _ = close_sx.send(fd); // Libinput doesn't care about closing errors
                },
            )?;

            libinput.udev_assign_seat(&seat_name)?;

            let mut stream = libinput.event_stream()?;

            loop {
                tokio::select! {
                    Some(signal) = signal_rx.recv() => {
                        match signal {
                            LibinputSignal::Shutdown => break,
                            LibinputSignal::Suspend => libinput.suspend(),
                            LibinputSignal::Resume => libinput.resume().expect("Failed to resume libinput context"), // FIXME: error handling
                        }
                    }
                    Some(res) = stream.next() => {
                        if event_sx
                            .send(res.map(|ref event| Event {
                                name: event.event_type(),
                                event_type: event.into(),
                                device_name: event.device().name().to_string_lossy().to_string(),
                            }))
                            .is_err()
                        {
                            break;
                        }
                    }
                    else => break,
                }
            }

            Ok::<_, Error>(())
        });

        rt.block_on(local);
    });

    Ok((event_rx, signal_sx))
}
