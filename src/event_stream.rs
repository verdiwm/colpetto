use std::{
    os::fd::RawFd,
    pin::Pin,
    task::{self, Poll},
};

use futures_core::{ready, Stream};
use tokio::io::unix::AsyncFd;

use crate::{Error, Event, Libinput, Result};

pub struct EventStream {
    libinput: Libinput,
    fd: AsyncFd<i32>,
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
