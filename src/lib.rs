#[allow(nonstandard_style)]
pub mod sys;

use std::{
    ffi::{c_char, c_int, c_void, CStr},
    io,
    pin::Pin,
    sync::atomic::{AtomicPtr, Ordering},
    task::{self, Poll},
};

use devil::Udev;
use futures_core::{ready, Stream};
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
    user_data: *mut c_void,
) -> c_int {
    let handler = user_data as *mut Handler;
    let handler = unsafe { &mut *handler };

    (handler.open)(CStr::from_ptr(path), flags)
}

unsafe extern "C" fn close_restricted(fd: c_int, user_data: *mut c_void) {
    let handler = user_data as *mut Handler;
    let handler = unsafe { &mut *handler };

    (handler.close)(fd)
}

const INTERFACE: sys::libinput_interface = sys::libinput_interface {
    open_restricted: Some(open_restricted),
    close_restricted: Some(close_restricted),
};

pub struct Libinput {
    raw: AtomicPtr<sys::libinput>,
}

struct Handler {
    open: Box<dyn FnMut(&CStr, c_int) -> c_int>,
    close: Box<dyn FnMut(c_int)>,
}

impl Libinput {
    pub fn new<O, C>(open: O, close: C) -> Result<Self, Error>
    where
        O: FnMut(&CStr, c_int) -> c_int + 'static,
        C: FnMut(c_int) + 'static,
    {
        let udev = Udev::new()?;

        let user_data = Box::new(Handler {
            open: Box::new(open),
            close: Box::new(close),
        });

        let libinput = unsafe {
            sys::libinput_udev_create_context(
                &INTERFACE,
                Box::into_raw(user_data) as *mut _ as _, // FIXME: this is leaking memory
                udev.as_raw().cast(),
            )
        };

        if libinput.is_null() {
            return Err(Error::Context);
        }

        Ok(Self {
            raw: AtomicPtr::new(libinput),
        })
    }

    pub fn as_raw(&self) -> *mut sys::libinput {
        self.raw.load(Ordering::SeqCst)
    }

    pub fn get_fd(&self) -> i32 {
        unsafe { sys::libinput_get_fd(self.as_raw()) }
    }

    pub fn event_stream(&self) -> Result<EventStream, Error> {
        Ok(EventStream {
            libinput: self.clone(),
            fd: AsyncFd::new(self.get_fd())?,
            is_first: true,
        })
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
        let user_data = unsafe { sys::libinput_get_user_data(self.as_raw()) };

        unsafe {
            if sys::libinput_unref(self.as_raw()).is_null() {
                drop(Box::<Handler>::from_raw(user_data.cast()));
            }
        }
    }
}

impl Clone for Libinput {
    fn clone(&self) -> Self {
        Self {
            raw: AtomicPtr::new(unsafe { sys::libinput_ref(self.as_raw()) }),
        }
    }
}

pub struct EventStream {
    libinput: Libinput,
    fd: AsyncFd<i32>,
    is_first: bool,
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
