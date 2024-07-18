#[allow(nonstandard_style)]
pub mod sys;

use std::{
    ffi::{c_char, c_int, c_void, CStr},
    io,
    pin::Pin,
    ptr::{null_mut, NonNull},
    task::{self, Poll},
};

use devil::Udev;
use futures_util::{ready, Stream};
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{open, Mode, OFlags},
};
use tokio::io::unix::AsyncFd;

mod event;
pub use event::Event;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create libinput context")]
    Context,
    #[error("Failed to assign seat")]
    Seat,
    #[error("{0}")]
    IoError(#[from] io::Error),
}

unsafe extern "C" fn open_restricted(
    path: *const c_char,
    flags: c_int,
    _user_data: *mut c_void,
) -> c_int {
    match open(
        CStr::from_ptr(path),
        OFlags::from_bits_retain(flags as u32),
        Mode::empty(),
    ) {
        Ok(fd) => fd.into_raw_fd(),
        Err(errno) => errno.raw_os_error(),
    }
}

unsafe extern "C" fn close_restricted(fd: c_int, _user_data: *mut c_void) {
    drop(OwnedFd::from_raw_fd(fd))
}

const INTERFACE: sys::libinput_interface = sys::libinput_interface {
    open_restricted: Some(open_restricted),
    close_restricted: Some(close_restricted),
};

pub struct Libinput {
    raw: NonNull<sys::libinput>,
    fd: AsyncFd<i32>,
    is_first: bool,
}

impl Libinput {
    pub fn new() -> Result<Self, Error> {
        let udev = Udev::new()?;

        let libinput = unsafe {
            sys::libinput_udev_create_context(&INTERFACE, null_mut(), udev.as_raw().cast())
        };

        if libinput.is_null() {
            return Err(Error::Context);
        }

        Ok(Self {
            raw: unsafe { NonNull::new_unchecked(libinput) },
            fd: AsyncFd::new(unsafe { sys::libinput_get_fd(libinput) })?,
            is_first: true,
        })
    }

    pub const fn as_raw(&self) -> *mut sys::libinput {
        self.raw.as_ptr()
    }

    pub fn assign_seat(&self, seat: &CStr) -> Result<(), Error> {
        unsafe {
            let res = sys::libinput_udev_assign_seat(self.as_raw(), seat.as_ptr());

            if res != 0 {
                return Err(Error::Seat);
            }
        }

        Ok(())
    }

    pub fn dispatch(&self) -> Result<(), Error> {
        unsafe {
            match sys::libinput_dispatch(self.as_raw()) {
                0 => Ok(()),
                e => Err(Error::IoError(io::Error::from_raw_os_error(-e))),
            }
        }
    }

    pub fn get_event(&self) -> Option<Event> {
        let raw = unsafe { sys::libinput_get_event(self.as_raw()) };

        if raw.is_null() {
            return None;
        }

        let event = Event::from_event_type(unsafe { sys::libinput_event_get_type(raw) });

        unsafe { sys::libinput_event_destroy(raw) };

        event
    }
}

impl Drop for Libinput {
    fn drop(&mut self) {
        unsafe {
            sys::libinput_unref(self.as_raw());
        }
    }
}

impl Stream for Libinput {
    type Item = Result<Event, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // The first time we poll there is already device created events available
            if self.is_first {
                self.dispatch()?;

                if let Some(event) = self.get_event() {
                    return Poll::Ready(Some(Ok(event)));
                } else {
                    self.is_first = false;
                    continue;
                }
            }

            let mut guard = ready!(self.fd.poll_read_ready(cx))?;
            self.dispatch()?;

            if let Some(event) = self.get_event() {
                return Poll::Ready(Some(Ok(event)));
            } else {
                guard.clear_ready();
                continue;
            }
        }
    }
}
