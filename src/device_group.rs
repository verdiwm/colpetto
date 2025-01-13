use crate::sys;

pub struct DeviceGroup {
    #[allow(unused)]
    raw: *mut sys::libinput_device_group,
}

impl DeviceGroup {
    pub unsafe fn from_raw(raw: *mut sys::libinput_device_group) -> Self {
        Self {
            raw: sys::libinput_device_group_ref(raw),
        }
    }
}

impl Drop for DeviceGroup {
    fn drop(&mut self) {
        unsafe { sys::libinput_device_group_unref(self.raw) };
    }
}
