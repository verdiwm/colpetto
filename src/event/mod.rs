use crate::{sys, Device};

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
    Added(DeviceNotifyEvent),
    Removed(DeviceNotifyEvent),
}

#[derive(Debug)]
pub struct DeviceNotifyEvent {
    raw: *mut sys::libinput_event_device_notify,
}

#[derive(Debug)]
pub enum KeyboardEvent {
    Key(KeyboardEventKey),
}

#[derive(Debug)]
pub struct KeyboardEventKey {
    raw: *mut sys::libinput_event_keyboard,
}

pub enum KeyState {
    Released,
    Pressed,
}

impl std::fmt::Display for KeyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyState::Released => write!(f, "released"),
            KeyState::Pressed => write!(f, "pressed"),
        }
    }
}

impl KeyState {
    pub const fn from_raw(raw: sys::libinput_key_state) -> Self {
        match raw {
            sys::libinput_key_state::LIBINPUT_KEY_STATE_RELEASED => Self::Released,
            sys::libinput_key_state::LIBINPUT_KEY_STATE_PRESSED => Self::Pressed,
            _ => panic!("libinput returned an invalid keystate"), // FIXME: I dont think panicking is a good idea. Maybe we could return an option?
        }
    }
}

impl KeyboardEventKey {
    pub fn key(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_key(self.raw) }
    }

    pub fn key_state(&self) -> KeyState {
        KeyState::from_raw(unsafe { sys::libinput_event_keyboard_get_key_state(self.raw) })
    }

    pub fn time(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_time(self.raw) }
    }

    pub fn time_usec(&self) -> u64 {
        unsafe { sys::libinput_event_keyboard_get_time_usec(self.raw) }
    }
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
    pub(crate) fn from_raw(
        raw: *mut sys::libinput_event,
        event_type: sys::libinput_event_type,
    ) -> Self {
        let event = match event_type {
            sys::libinput_event_type::LIBINPUT_EVENT_DEVICE_ADDED => {
                Event::Device(DeviceEvent::Added(DeviceNotifyEvent {
                    raw: unsafe { sys::libinput_event_get_device_notify_event(raw) },
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_DEVICE_REMOVED => {
                Event::Device(DeviceEvent::Removed(DeviceNotifyEvent {
                    raw: unsafe { sys::libinput_event_get_device_notify_event(raw) },
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_KEYBOARD_KEY => {
                Event::Keyboard(KeyboardEvent::Key(KeyboardEventKey {
                    raw: unsafe { sys::libinput_event_get_keyboard_event(raw) },
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION => {
                Event::Pointer(PointerEvent::Motion)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE => {
                Event::Pointer(PointerEvent::MotionAbsolute)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_BUTTON => {
                Event::Pointer(PointerEvent::Button)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_AXIS => {
                Event::Pointer(PointerEvent::Axis)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_SCROLL_WHEEL => {
                Event::Pointer(PointerEvent::ScrollWheel)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_SCROLL_FINGER => {
                Event::Pointer(PointerEvent::ScrollFinger)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_POINTER_SCROLL_CONTINUOUS => {
                Event::Pointer(PointerEvent::ScrollContinuous)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_DOWN => Event::Touch(TouchEvent::Down),
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_UP => Event::Touch(TouchEvent::Up),
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_MOTION => {
                Event::Touch(TouchEvent::Motion)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_CANCEL => {
                Event::Touch(TouchEvent::Cancel)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_FRAME => Event::Touch(TouchEvent::Frame),
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_AXIS => {
                Event::TabletTool(TabletToolEvent::Axis)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY => {
                Event::TabletTool(TabletToolEvent::Proximity)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_TIP => {
                Event::TabletTool(TabletToolEvent::Tip)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_BUTTON => {
                Event::TabletTool(TabletToolEvent::Button)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_BUTTON => {
                Event::TabletPad(TabletPadEvent::Button)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_RING => {
                Event::TabletPad(TabletPadEvent::Ring)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_STRIP => {
                Event::TabletPad(TabletPadEvent::Strip)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_KEY => {
                Event::TabletPad(TabletPadEvent::Key)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_DIAL => {
                Event::TabletPad(TabletPadEvent::Dial)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN => {
                Event::Gesture(GestureEvent::SwipeBegin)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE => {
                Event::Gesture(GestureEvent::SwipeUpdate)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END => {
                Event::Gesture(GestureEvent::SwipeEnd)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN => {
                Event::Gesture(GestureEvent::PinchBegin)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE => {
                Event::Gesture(GestureEvent::PinchUpdate)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END => {
                Event::Gesture(GestureEvent::PinchEnd)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_HOLD_BEGIN => {
                Event::Gesture(GestureEvent::HoldBegin)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_GESTURE_HOLD_END => {
                Event::Gesture(GestureEvent::HoldEnd)
            }
            sys::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE => {
                Event::Switch(SwitchEvent::Toggle)
            }
            _ => Event::Unknown,
        };

        event
    }
}

pub trait AsEvent: sealed::EventSealed {
    fn as_raw_event(&self) -> *mut sys::libinput_event;

    fn device(&self) -> Device {
        unsafe { Device::from_raw(sys::libinput_event_get_device(self.as_raw_event())) }
    }
}

impl AsEvent for KeyboardEventKey {
    fn as_raw_event(&self) -> *mut sys::libinput_event {
        unsafe { sys::libinput_event_keyboard_get_base_event(self.raw) }
    }
}

impl AsEvent for DeviceNotifyEvent {
    fn as_raw_event(&self) -> *mut sys::libinput_event {
        unsafe { sys::libinput_event_device_notify_get_base_event(self.raw) }
    }
}

mod sealed {

    pub trait EventSealed {}

    impl EventSealed for super::KeyboardEventKey {}
    impl EventSealed for super::DeviceNotifyEvent {}
}
