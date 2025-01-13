use crate::sys;

pub struct Seat {
    #[allow(unused)]
    raw: *mut sys::libinput_seat,
}

impl Seat {
    pub unsafe fn from_raw(raw: *mut sys::libinput_seat) -> Self {
        Self {
            raw: sys::libinput_seat_ref(raw),
        }
    }
}

impl Drop for Seat {
    fn drop(&mut self) {
        unsafe { sys::libinput_seat_unref(self.raw) };
    }
}
