use crate::sys;

#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    Device(DeviceEvent),
    Keyboard(KeyboardEvent),
    Pointer,
    Touch,
    TabletPad,
    TabletTool,
    Gesture,
    Switch,
    Unknown,
}

#[derive(Debug)]
pub enum DeviceEvent {
    Added,
    Removed,
}

#[derive(Debug)]
pub enum KeyboardEvent {
    Key,
}

#[derive(Debug)]
pub enum PointerEvent {}

impl Event {
    pub(crate) fn from_event_type(event_type: sys::libinput_event_type) -> Option<Self> {
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
                Event::Keyboard(KeyboardEvent::Key)
            }
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_BUTTON => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_AXIS => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_SCROLL_WHEEL => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_SCROLL_FINGER => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_POINTER_SCROLL_CONTINUOUS => Event::Pointer,
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_DOWN => Event::Touch,
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_UP => Event::Touch,
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_MOTION => Event::Touch,
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_CANCEL => Event::Touch,
            sys::libinput_event_type_LIBINPUT_EVENT_TOUCH_FRAME => Event::Touch,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_AXIS => Event::TabletTool,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY => Event::TabletTool,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_TIP => Event::TabletTool,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_BUTTON => Event::TabletTool,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_BUTTON => Event::TabletPad,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_RING => Event::TabletPad,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_STRIP => Event::TabletPad,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_KEY => Event::TabletPad,
            sys::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_DIAL => Event::TabletPad,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_END => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_BEGIN => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_UPDATE => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_END => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_HOLD_BEGIN => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_GESTURE_HOLD_END => Event::Gesture,
            sys::libinput_event_type_LIBINPUT_EVENT_SWITCH_TOGGLE => Event::Switch,
            _ => Event::Unknown,
        };

        Some(event)
    }
}
