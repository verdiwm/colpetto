use crate::{macros, sys};

/// A base handle for accessing libinput device groups.
pub struct DeviceGroup {
    #[allow(unused)]
    raw: *mut sys::libinput_device_group,
}

impl DeviceGroup {
    /// Builds a new device group from a raw libinput one
    /// # Safety
    ///
    /// The caller must ensure it's passing a valid pointer
    pub unsafe fn from_raw(raw: *mut sys::libinput_device_group) -> Self {
        unsafe {
            Self {
                raw: sys::libinput_device_group_ref(raw),
            }
        }
    }
}

impl Drop for DeviceGroup {
    fn drop(&mut self) {
        unsafe { sys::libinput_device_group_unref(self.raw) };
    }
}

impl Clone for DeviceGroup {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::libinput_device_group_ref(self.raw) },
        }
    }
}

macros::impl_debug!(DeviceGroup);
