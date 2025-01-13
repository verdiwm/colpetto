use crate::sys::{
    libinput_event_get_pointer_event, libinput_event_pointer, libinput_event_pointer_get_base_event,
};

super::define_events!(
    Pointer,
    libinput_event_pointer,
    libinput_event_pointer_get_base_event,
    libinput_event_get_pointer_event,
    Motion,
    MotionAbsolute,
    Button,
    Axis,
    ScrollWheel,
    ScrollFinger,
    ScrollContinuous,
);
