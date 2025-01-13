use std::ffi::CStr;

use crate::sys;

pub struct Device {
    raw: *mut sys::libinput_device,
}

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

pub struct DeviceGroup {
    #[allow(unused)]
    raw: *mut sys::libinput_device_group,
}

impl Device {
    pub unsafe fn from_raw(raw: *mut sys::libinput_device) -> Self {
        Self { raw }
    }

    pub fn device_group(&self) -> DeviceGroup {
        DeviceGroup {
            raw: unsafe { sys::libinput_device_get_device_group(self.raw) },
        }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::libinput_device_get_name(self.raw)) }
    }

    pub fn udev_device(&self) -> devil::Device {
        unsafe { devil::Device::from_raw(sys::libinput_device_get_udev_device(self.raw).cast()) }
    }

    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        unsafe { sys::libinput_device_has_capability(self.raw, capability as u32) != 0 }
    }
}
