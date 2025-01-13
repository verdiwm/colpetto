use crate::sys::{
    libinput_event_gesture, libinput_event_gesture_get_base_event, libinput_event_get_gesture_event,
};

super::define_events!(
    Gesture,
    libinput_event_gesture,
    libinput_event_gesture_get_base_event,
    libinput_event_get_gesture_event,
    SwipeBegin,
    SwipeUpdate,
    SwipeEnd,
    PinchBegin,
    PinchUpdate,
    PinchEnd,
    HoldBegin,
    HoldEnd,
);
