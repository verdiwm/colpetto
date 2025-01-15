#![allow(missing_docs)] // FIXME: touch events dont seem to be properly documented by libinput

//! Events from absolute touch devices.

use crate::sys::{
    libinput_event_get_touch_event, libinput_event_touch, libinput_event_touch_get_base_event,
};

super::define_events!(
    Touch,
    libinput_event_touch,
    libinput_event_touch_get_base_event,
    libinput_event_get_touch_event,
    Down,
    Up,
    Motion,
    Cancel,
    Frame,
);
