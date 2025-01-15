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

    /// The descriptive device name as advertised by the kernel and/or the hardware itself.
    /// To get the sysname for this device, use [`sysname`](Self::sysname).
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::libinput_device_get_name(self.raw)) }
    }

    /// Get the system name of the device.
    /// To get the descriptive device name, use [`name`](Self::name).
    pub fn sysname(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::libinput_device_get_sysname(self.raw)) }
    }

    /// A device may be mapped to a single output, or all available outputs.
    /// If a device is mapped to a single output only, a relative device may not move beyond the boundaries of this output.
    /// An absolute device has its input coordinates mapped to the extents of this output.
    ///
    /// # Note
    ///
    /// Use of this function is discouraged.
    /// Its return value is not precisely defined and may not be understood by the caller or may be insufficient to map the device.
    /// Instead, the system configuration could set a udev property the caller understands and interprets correctly.
    /// The caller could then obtain device with [`udev_device`](Self::udev_device) and query it for this property.
    /// For more complex cases, the caller must implement monitor-to-device association heuristics.
    pub fn output_name(&self) -> Option<&CStr> {
        let name = unsafe { sys::libinput_device_get_output_name(self.raw) };

        if name.is_null() {
            return None;
        }

        Some(unsafe { CStr::from_ptr(name) })
    }

    /// Get the seat associated with this input device.
    /// A seat can be uniquely identified by the physical and logical seat name.
    /// There will ever be only one seat instance with a given physical and logical seat name pair at any given time,
    /// but if no external reference is kept, it may be destroyed if no device belonging to it is left.
    pub fn seat(&self) -> Seat {
        unsafe { Seat::from_raw(sys::libinput_device_get_seat(self.raw)) }
    }

    /// Return a udev handle to the device that is this libinput device, if any
    ///
    /// Some devices may not have a udev device, or the udev device may be unobtainable.
    /// Calling this function multiple times for the same device may not return the same udev handle each time.
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
