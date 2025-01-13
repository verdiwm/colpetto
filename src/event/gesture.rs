use crate::sys::{
    libinput_event_gesture, libinput_event_gesture_get_base_event, libinput_event_get_gesture_event,
};

use super::define_events;

#[derive(Debug)]
pub enum GestureEvent {
    SwipeBegin(GestureSwipeBeginEvent),
    SwipeUpdate(GestureSwipeUpdateEvent),
    SwipeEnd(GestureSwipeEndEvent),
    PinchBegin(GesturePinchBeginEvent),
    PinchUpdate(GesturePinchUpdateEvent),
    PinchEnd(GesturePinchEndEvent),
    HoldBegin(GestureHoldBeginEvent),
    HoldEnd(GestureHoldEndEvent),
}

define_events!(
    libinput_event_gesture,
    libinput_event_gesture_get_base_event,
    libinput_event_get_gesture_event,
    GestureSwipeBeginEvent,
    GestureSwipeUpdateEvent,
    GestureSwipeEndEvent,
    GesturePinchBeginEvent,
    GesturePinchUpdateEvent,
    GesturePinchEndEvent,
    GestureHoldBeginEvent,
    GestureHoldEndEvent,
);
