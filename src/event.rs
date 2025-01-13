use crate::{sys, Device};

mod device;
mod gesture;
mod keyboard;
mod pointer;
mod switch;
mod table_pad;
mod tablet_tool;
mod touch;

pub use device::*;
pub use gesture::*;
pub use keyboard::*;
pub use pointer::*;
pub use switch::*;
pub use table_pad::*;
pub use tablet_tool::*;
pub use touch::*;

macro_rules! define_events {
    ($raw:ident, $get:expr, $set:expr, $($event:ident,)+) => {
        $(
            #[derive(Debug)]
            pub struct $event {
                raw: *mut $raw,
            }

            impl crate::event::AsRawEvent for $event {
                fn as_raw_event(&self) -> *mut crate::sys::libinput_event {
                    unsafe { $get(self.raw) }
                }
            }

            impl crate::event::FromRawEvent for $event {
                unsafe fn from_raw_event(event: *mut crate::sys::libinput_event) -> Self {
                    Self {
                        raw: $set(event),
                    }
                }
            }
        )+
    };
}

pub(crate) use define_events;

#[derive(Debug)]
pub enum Event {
    Device(DeviceEvent),
    Gesture(GestureEvent),
    Keyboard(KeyboardEvent),
    Pointer(PointerEvent),
    Switch(SwitchEvent),
    TabletPad(TabletPadEvent),
    TabletTool(TabletToolEvent),
    Touch(TouchEvent),
    Unknown,
}

impl Event {
    pub(crate) fn from_raw(
        event: *mut sys::libinput_event,
        event_type: sys::libinput_event_type,
    ) -> Self {
        let event = match event_type {
            sys::libinput_event_type::LIBINPUT_EVENT_DEVICE_ADDED => {
                Event::Device(DeviceEvent::Added(unsafe {
                    DeviceNotifyEvent::from_raw_event(event)
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_DEVICE_REMOVED => {
                Event::Device(DeviceEvent::Removed(unsafe {
                    DeviceNotifyEvent::from_raw_event(event)
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_KEYBOARD_KEY => {
                Event::Keyboard(KeyboardEvent::Key(unsafe {
                    KeyboardKeyEvent::from_raw_event(event)
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
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_DOWN => {
                Event::Touch(TouchEvent::Down(unsafe {
                    TouchDownEvent::from_raw_event(event)
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_UP => {
                Event::Touch(TouchEvent::Up(unsafe {
                    TouchUpEvent::from_raw_event(event)
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_MOTION => {
                Event::Touch(TouchEvent::Motion(unsafe {
                    TouchMotionEvent::from_raw_event(event)
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_CANCEL => {
                Event::Touch(TouchEvent::Cancel(unsafe {
                    TouchCancelEvent::from_raw_event(event)
                }))
            }
            sys::libinput_event_type::LIBINPUT_EVENT_TOUCH_FRAME => {
                Event::Touch(TouchEvent::Frame(unsafe {
                    TouchFrameEvent::from_raw_event(event)
                }))
            }
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

pub trait AsRawEvent: sealed::EventSealed {
    fn as_raw_event(&self) -> *mut sys::libinput_event;

    fn device(&self) -> Device {
        unsafe { Device::from_raw(sys::libinput_event_get_device(self.as_raw_event())) }
    }
}

pub trait FromRawEvent: sealed::EventSealed {
    unsafe fn from_raw_event(event: *mut sys::libinput_event) -> Self;
}

mod sealed {

    pub trait EventSealed {}

    impl EventSealed for super::KeyboardKeyEvent {}
    impl EventSealed for super::DeviceNotifyEvent {}
    impl EventSealed for super::TouchUpEvent {}
    impl EventSealed for super::TouchDownEvent {}
    impl EventSealed for super::TouchFrameEvent {}
    impl EventSealed for super::TouchCancelEvent {}
    impl EventSealed for super::TouchMotionEvent {}
}
