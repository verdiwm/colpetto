use std::{
    os::fd::RawFd,
    pin::Pin,
    task::{self, Poll},
};

use futures_core::{ready, Stream};
use tokio::io::unix::AsyncFd;

use crate::{Error, Event, Libinput, Result};

/// An asynchronous stream of libinput events integrated with the tokio runtime.
///
/// `EventStream` provides an asynchronous interface to libinput's event system, allowing
/// you to receive input device events using tokio's async/await syntax. It implements
/// the [`Stream`] trait, making it compatible with tokio's async ecosystem.
///
/// Events include device additions/removals, as well as input events like key presses,
/// pointer movements, touch events, etc. The stream will yield these events as they
/// occur from the underlying libinput context.
///
/// # Initial Events
///
/// When first polling the stream, it will immediately yield any pending device creation
/// events before waiting for new events. This ensures you receive information about
/// all currently connected devices.
///
/// # Example usage
///
/// ```
/// use tokio_stream::StreamExt;
///
/// /* libinput initialization is omited for brevity */
///
/// let mut stream = libinput.event_stream()?;
///
/// while let Some(event) = stream.try_next().await? {
///     println!(
///         "Got \"{}\" event from \"{}\"",
///         event.event_type(),
///         event.device().name().to_string_lossy()
///     )
/// }
/// ```
///
/// # Implementation Note
///
/// The stream internally manages an [`AsyncFd`] wrapper around the libinput file descriptor,
/// ensuring efficient integration with tokio's event loop. It will only wake up when new
/// events are available to be read from the libinput context.
#[derive(Debug)]
pub struct EventStream {
    libinput: Libinput,
    fd: AsyncFd<RawFd>,
    is_first: bool,
}

impl EventStream {
    pub(crate) fn new(libinput: Libinput, fd: RawFd) -> Result<Self> {
        Ok(Self {
            libinput,
            fd: AsyncFd::new(fd)?,
            is_first: true,
        })
    }
}

impl Stream for EventStream {
    type Item = Result<Event, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // The first time we poll there is already device created events available
            if self.is_first {
                self.libinput.dispatch()?;

                if let Some(event) = self.libinput.get_event() {
                    return Poll::Ready(Some(Ok(event)));
                } else {
                    self.is_first = false;
                    continue;
                }
            }

            let mut guard = ready!(self.fd.poll_read_ready(cx))?;
            self.libinput.dispatch()?;

            if let Some(event) = self.libinput.get_event() {
                return Poll::Ready(Some(Ok(event)));
            } else {
                guard.clear_ready();
                continue;
            }
        }
    }
}
