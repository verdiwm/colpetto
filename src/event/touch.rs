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
