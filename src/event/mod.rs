use crate::sys;

#[derive(Debug)]
pub enum Event {
    Device(DeviceEvent),
    Keyboard(KeyboardEvent),
    Pointer(PointerEvent),
    Touch(TouchEvent),
    TabletTool(TabletToolEvent),
    TabletPad(TabletPadEvent),
    Gesture(GestureEvent),
    Switch(SwitchEvent),
    Unknown,
}

#[derive(Debug)]
pub enum DeviceEvent {
    Added,
    Removed,
}

#[derive(Debug)]
pub enum KeyboardEvent {
    Key(KeyboardEventKey),
}

#[derive(Debug)]
pub struct KeyboardEventKey {
    raw: *mut sys::libinput_event_keyboard,
}

#[derive(Debug)]
pub enum PointerEvent {
    Motion,
    MotionAbsolute,
    Button,
    Axis,
    ScrollWheel,
    ScrollFinger,
    ScrollContinuous,
}

#[derive(Debug)]
pub enum TouchEvent {
    Down,
    Up,
    Motion,
    Cancel,
    Frame,
}

#[derive(Debug)]
pub enum TabletToolEvent {
    Axis,
    Proximity,
    Tip,
    Button,
}

#[derive(Debug)]
pub enum TabletPadEvent {
    Button,
    Ring,
    Strip,
    Key,
    Dial,
}

#[derive(Debug)]
pub enum GestureEvent {
    SwipeBegin,
    SwipeUpdate,
    SwipeEnd,
    PinchBegin,
    PinchUpdate,
    PinchEnd,
    HoldBegin,
    HoldEnd,
}

#[derive(Debug)]
pub enum SwitchEvent {
    Toggle,
}

impl Event {
    pub(crate) fn get_event(raw: *mut sys::libinput) -> Option<Self> {
        let event = unsafe { sys::libinput_get_event(raw) };

        if event.is_null() {
            return None;
        }

        let event_type = unsafe { sys::libinput_event_get_type(event) };

        let event = match event_type {
            sys::libinput_event_type_LIBINPUT_EVENT_NONE => {
                return None;
            }
            sys::libinput_event_type_LIBINPUT_EVENT_DEVICE_ADDED => {
                Event::Device(DeviceEvent::Added)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_DEVICE_REMOVED => {
                Event::Device(DeviceEvent::Removed)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_KEYBOARD_KEY => {
                let keyboard_event = unsafe { sys::libinput_event_get_keyboard_event(event) };

                Event::Keyboard(KeyboardEvent::Key(KeyboardEventKey {
                    raw: keyboard_event,
                }))
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION => {
                Event::Pointer(PointerEvent::Motion)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE => {
                Event::Pointer(PointerEvent::MotionAbsolute)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_BUTTON => {
                Event::Pointer(PointerEvent::Button)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_AXIS => {
                Event::Pointer(PointerEvent::Axis)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_SCROLL_WHEEL => {
                Event::Pointer(PointerEvent::ScrollWheel)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_SCROLL_FINGER => {
                Event::Pointer(PointerEvent::ScrollFinger)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_SCROLL_CONTINUOUS => {
                Event::Pointer(PointerEvent::ScrollContinuous)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_DOWN => Event::Touch(TouchEvent::Down),
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_UP => Event::Touch(TouchEvent::Up),
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_MOTION => {
                Event::Touch(TouchEvent::Motion)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_CANCEL => {
                Event::Touch(TouchEvent::Cancel)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_FRAME => Event::Touch(TouchEvent::Frame),
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_AXIS => {
                Event::TabletTool(TabletToolEvent::Axis)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY => {
                Event::TabletTool(TabletToolEvent::Proximity)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_TIP => {
                Event::TabletTool(TabletToolEvent::Tip)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_BUTTON => {
                Event::TabletTool(TabletToolEvent::Button)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_BUTTON => {
                Event::TabletPad(TabletPadEvent::Button)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_RING => {
                Event::TabletPad(TabletPadEvent::Ring)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_STRIP => {
                Event::TabletPad(TabletPadEvent::Strip)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_KEY => {
                Event::TabletPad(TabletPadEvent::Key)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_DIAL => {
                Event::TabletPad(TabletPadEvent::Dial)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN => {
                Event::Gesture(GestureEvent::SwipeBegin)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE => {
                Event::Gesture(GestureEvent::SwipeUpdate)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_END => {
                Event::Gesture(GestureEvent::SwipeEnd)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_BEGIN => {
                Event::Gesture(GestureEvent::PinchBegin)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_UPDATE => {
                Event::Gesture(GestureEvent::PinchUpdate)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_END => {
                Event::Gesture(GestureEvent::PinchEnd)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_HOLD_BEGIN => {
                Event::Gesture(GestureEvent::HoldBegin)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_HOLD_END => {
                Event::Gesture(GestureEvent::HoldEnd)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_SWITCH_TOGGLE => {
                Event::Switch(SwitchEvent::Toggle)
            }
            _ => Event::Unknown,
        };
        // unsafe { sys::libinput_event_destroy(raw) };
        Some(event)
    }
}
