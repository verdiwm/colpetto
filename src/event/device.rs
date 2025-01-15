//! Device events are generated when a device is added/removed

use crate::sys::{
    libinput_event_device_notify, libinput_event_device_notify_get_base_event,
    libinput_event_get_device_notify_event,
};

super::define_events!(
    /// An event notifying the caller of a device being added or removed.
    Device,
    libinput_event_device_notify,
    libinput_event_device_notify_get_base_event,
    libinput_event_get_device_notify_event,
    /// Signals that a device has been added to the context.
    /// The device will not be read until the next time the user calls libinput_dispatch() and data is available.
    /// This allows setting up initial device configuration before any events are created.
    Added,
    /// Signals that a device has been removed. No more events from the associated device will be in the queue or be queued after this event.
    Removed,
);
