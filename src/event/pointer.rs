use crate::sys::{
    libinput_event_get_pointer_event, libinput_event_pointer, libinput_event_pointer_get_base_event,
};

use super::define_events;

#[derive(Debug)]
pub enum PointerEvent {
    Motion(PointerMotionEvent),
    MotionAbsolute(PointerMotionAbsoluteEvent),
    Button(PointerButtonEvent),
    Axis(PointerAxisEvent),
    ScrollWheel(PointerScrollWheelEvent),
    ScrollFinger(PointerScrollFingerEvent),
    ScrollContinuous(PointerScrollContinuousEvent),
}

define_events!(
    libinput_event_pointer,
    libinput_event_pointer_get_base_event,
    libinput_event_get_pointer_event,
    PointerMotionEvent,
    PointerMotionAbsoluteEvent,
    PointerButtonEvent,
    PointerAxisEvent,
    PointerScrollWheelEvent,
    PointerScrollFingerEvent,
    PointerScrollContinuousEvent,
);
