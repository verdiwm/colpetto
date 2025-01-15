use std::ffi::CStr;

use crate::{sys, DeviceGroup, Seat};

/// A base handle for accessing libinput devices.
pub struct Device {
    raw: *mut sys::libinput_device,
}

/// Capabilities on a device. A device may have one or more capabilities at a time, capabilities remain static for the lifetime of the device.
#[repr(u32)]
#[non_exhaustive]
pub enum DeviceCapability {
    Gesture = sys::LIBINPUT_DEVICE_CAP_GESTURE,
    Keyboard = sys::LIBINPUT_DEVICE_CAP_KEYBOARD,
    Pointer = sys::LIBINPUT_DEVICE_CAP_POINTER,
    Switch = sys::LIBINPUT_DEVICE_CAP_SWITCH,
    TabletPad = sys::LIBINPUT_DEVICE_CAP_TABLET_PAD,
    TabletTool = sys::LIBINPUT_DEVICE_CAP_TABLET_TOOL,
    Touch = sys::LIBINPUT_DEVICE_CAP_TOUCH,
}

impl Device {
    /// Builds a new device from a raw libinput one
    ///
    /// # Safety
    ///
    /// The caller must ensure it's passing a valid pointer
    pub unsafe fn from_raw(raw: *mut sys::libinput_device) -> Self {
        Self {
            raw: unsafe { sys::libinput_device_ref(raw) },
        }
    }

    pub fn device_group(&self) -> DeviceGroup {
        unsafe { DeviceGroup::from_raw(sys::libinput_device_get_device_group(self.raw)) }
    }

    pub fn bustype_id(&self) -> u32 {
        unsafe { sys::libinput_device_get_id_bustype(self.raw) }
    }

    pub fn product_id(&self) -> u32 {
        unsafe { sys::libinput_device_get_id_product(self.raw) }
    }

    pub fn vendor_id(&self) -> u32 {
        unsafe { sys::libinput_device_get_id_vendor(self.raw) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::libinput_device_get_name(self.raw)) }
    }

    pub fn output_name(&self) -> Option<&CStr> {
        let name = unsafe { sys::libinput_device_get_output_name(self.raw) };

        if name.is_null() {
            return None;
        }

        Some(unsafe { CStr::from_ptr(name) })
    }

    pub fn seat(&self) -> Seat {
        unsafe { Seat::from_raw(sys::libinput_device_get_seat(self.raw)) }
    }

    pub fn udev_device(&self) -> Option<devil::Device> {
        let device = unsafe { sys::libinput_device_get_udev_device(self.raw) };

        if device.is_null() {
            return None;
        }

        Some(unsafe { devil::Device::from_raw(device.cast()) })
    }

    /// Check if the given device has the specified capability
    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        unsafe { sys::libinput_device_has_capability(self.raw, capability as u32) != 0 }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { sys::libinput_device_unref(self.raw) };
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Self {
            raw: unsafe { sys::libinput_device_ref(self.raw) },
        }
    }
}
