use std::ffi::CStr;

use crate::sys;

pub struct Device {
    raw: *mut sys::libinput_device,
}

impl Device {
    pub unsafe fn from_raw(raw: *mut sys::libinput_device) -> Self {
        Self { raw }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::libinput_device_get_name(self.raw)) }
    }

    pub fn udev_device(&self) -> devil::Device {
        unsafe { devil::Device::from_raw(sys::libinput_device_get_udev_device(self.raw).cast()) }
    }
}
