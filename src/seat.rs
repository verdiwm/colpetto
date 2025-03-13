use crate::{macros, sys};

/// The base handle for accessing libinput seats
pub struct Seat {
    #[allow(unused)]
    raw: *mut sys::libinput_seat,
}

impl Seat {
    /// Builds a new seat from a raw libinput one
    ///
    /// # Safety
    ///
    /// The caller must ensure it's passing a valid pointer
    pub unsafe fn from_raw(raw: *mut sys::libinput_seat) -> Self {
        unsafe {
            Self {
                raw: sys::libinput_seat_ref(raw),
            }
        }
    }
}

impl Drop for Seat {
    fn drop(&mut self) {
        unsafe { sys::libinput_seat_unref(self.raw) };
    }
}

impl Clone for Seat {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::libinput_seat_ref(self.raw) },
        }
    }
}

macros::impl_debug!(Seat);
