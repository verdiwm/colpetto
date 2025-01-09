use crate::sys;

pub struct Device {
    raw: *mut sys::libinput_device,
}

impl Device {
    pub unsafe fn from_raw(raw: *mut sys::libinput_device) -> Self {
        Self { raw }
    }
}
