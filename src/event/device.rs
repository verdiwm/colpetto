use crate::sys;

#[derive(Debug)]
pub enum DeviceEvent {
    Added(DeviceNotifyEvent),
    Removed(DeviceNotifyEvent),
}

#[derive(Debug)]
pub struct DeviceNotifyEvent {
    raw: *mut sys::libinput_event_device_notify,
}

impl super::RawEvent for DeviceNotifyEvent {
    fn as_raw_event(&self) -> *mut sys::libinput_event {
        unsafe { sys::libinput_event_device_notify_get_base_event(self.raw) }
    }

    unsafe fn from_raw_event(event: *mut sys::libinput_event) -> Self {
        Self {
            raw: sys::libinput_event_get_device_notify_event(event),
        }
    }
}
