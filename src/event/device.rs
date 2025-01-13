use crate::sys::{
    libinput_event_device_notify, libinput_event_device_notify_get_base_event,
    libinput_event_get_device_notify_event,
};

super::define_events!(
    Device,
    libinput_event_device_notify,
    libinput_event_device_notify_get_base_event,
    libinput_event_get_device_notify_event,
    Added,
    Removed,
);
