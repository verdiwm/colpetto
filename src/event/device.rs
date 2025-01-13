use crate::sys::{
    libinput_event_device_notify, libinput_event_device_notify_get_base_event,
    libinput_event_get_device_notify_event,
};

use super::define_events;

#[derive(Debug)]
pub enum DeviceEvent {
    Added(DeviceAddedEvent),
    Removed(DeviceRemovedEvent),
}

define_events!(
    libinput_event_device_notify,
    libinput_event_device_notify_get_base_event,
    libinput_event_get_device_notify_event,
    DeviceAddedEvent,
    DeviceRemovedEvent,
);
