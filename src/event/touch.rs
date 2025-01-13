use crate::sys::{
    libinput_event_get_touch_event, libinput_event_touch, libinput_event_touch_get_base_event,
};

use super::define_events;

#[derive(Debug)]
pub enum TouchEvent {
    Down(TouchDownEvent),
    Up(TouchUpEvent),
    Motion(TouchMotionEvent),
    Cancel(TouchCancelEvent),
    Frame(TouchFrameEvent),
}

define_events!(
    libinput_event_touch,
    libinput_event_touch_get_base_event,
    libinput_event_get_touch_event,
    TouchDownEvent,
    TouchUpEvent,
    TouchMotionEvent,
    TouchCancelEvent,
    TouchFrameEvent,
);
