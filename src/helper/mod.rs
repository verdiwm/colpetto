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

use crate::event::AsRawEvent;
use crate::{Error, Libinput, Result};

pub mod event;
use event::Event;

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
        OFut: Future<Output = RawFd> + Send,
        C: Fn(RawFd) -> CFut + Send + 'static,
        CFut: Future + Send,
    {
        let (open_request_sx, open_response_rx, close_sx) = {
            let (open_request_sx, open_request_rx) = tokio_mpsc::unbounded_channel::<CString>();
            let (open_response_sx, open_response_rx) = mpsc::channel::<c_int>();

            let (close_sx, close_rx) = tokio_mpsc::unbounded_channel::<c_int>();

            let mut close_rx = UnboundedReceiverStream::new(close_rx);
            let mut open_request_rx = UnboundedReceiverStream::new(open_request_rx);

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        Some(path) = open_request_rx.next() => {
                            if open_response_sx.send(open(path).await).is_err() {
                                break;
                            }
                        }
                        Some(fd) = close_rx.next() => {
                            close(fd).await;
                        }
                        else => break
                    }
                }
            });

            (open_request_sx, open_response_rx, close_sx)
        };

        let (rx, signal_sender) =
            spawn_libinput_task(seat_name, open_request_sx, close_sx, open_response_rx)?;

        let stream = UnboundedReceiverStream::new(rx);

        Ok((Self { signal_sender }, stream))
    }

    pub fn shutdown(&self) {
        let _ = self.signal_sender.send(LibinputSignal::Shutdown);
    }

    pub fn suspend(&self) -> Result<()> {
        let _ = self.signal_sender.send(LibinputSignal::Suspend);

        Ok(())
    }

    pub fn resume(&self) -> Result<()> {
        if self.signal_sender.send(LibinputSignal::Resume).is_err() {
            return Err(Error::Resume);
        }

        Ok(())
    }
}

enum LibinputSignal {
    Shutdown,
    Suspend,
    Resume,
}

fn spawn_libinput_task(
    seat_name: CString,
    open_request_sx: tokio_mpsc::UnboundedSender<CString>,
    close_sx: tokio_mpsc::UnboundedSender<i32>,
    open_response_rx: mpsc::Receiver<i32>,
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
            let mut libinput = Libinput::new(
                move |path, _| {
                    open_request_sx.send(path.to_owned()).map_err(|_| -1)?;
                    open_response_rx.recv().map_err(|_| -1)
                },
                move |fd| {
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
                            LibinputSignal::Resume => {
                                if let Err(err) = libinput.resume() {
                                    if event_sx.send(Err(err)).is_err() {
                                        break
                                    }
                                }
                            }
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
