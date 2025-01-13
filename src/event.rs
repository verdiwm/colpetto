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
            pub struct $event {
                raw: *mut $raw,
            }

            impl std::fmt::Debug for $event {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!($event)).finish()
                }
            }

            impl crate::event::AsRawEvent<$raw> for $event {
                fn as_raw(&self) -> *mut $raw {
                    self.raw
                }

                fn as_raw_event(&self) -> *mut crate::sys::libinput_event {
                    unsafe { $get(self.raw) }
                }
            }

            impl crate::event::FromRawEvent<$raw> for $event {
                unsafe fn from_raw(raw: *mut $raw) -> Self {
                    Self {
                        raw,
                    }
                }

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

macro_rules! map_raw {
    ($outer:ident($inner:ident), $event:expr) => {
        paste::paste! {
            crate::Event::$outer(crate::event::[<$outer Event>]::$inner(unsafe {
                crate::event::[<$outer $inner Event>]::from_raw_event($event)
            }))
        }
    };
}

impl Event {
    pub(crate) fn from_raw(
        event: *mut sys::libinput_event,

        event_type: sys::libinput_event_type::Type,
    ) -> Self {
        use sys::libinput_event_type::*;

        let event = match event_type {
            LIBINPUT_EVENT_DEVICE_ADDED => map_raw!(Device(Added), event),
            LIBINPUT_EVENT_DEVICE_REMOVED => map_raw!(Device(Removed), event),

            LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN => map_raw!(Gesture(SwipeBegin), event),
            LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE => map_raw!(Gesture(SwipeUpdate), event),
            LIBINPUT_EVENT_GESTURE_SWIPE_END => map_raw!(Gesture(SwipeEnd), event),
            LIBINPUT_EVENT_GESTURE_PINCH_BEGIN => map_raw!(Gesture(PinchBegin), event),
            LIBINPUT_EVENT_GESTURE_PINCH_UPDATE => map_raw!(Gesture(PinchUpdate), event),
            LIBINPUT_EVENT_GESTURE_PINCH_END => map_raw!(Gesture(PinchEnd), event),
            LIBINPUT_EVENT_GESTURE_HOLD_BEGIN => map_raw!(Gesture(HoldBegin), event),
            LIBINPUT_EVENT_GESTURE_HOLD_END => map_raw!(Gesture(HoldEnd), event),

            LIBINPUT_EVENT_KEYBOARD_KEY => map_raw!(Keyboard(Key), event),

            LIBINPUT_EVENT_POINTER_MOTION => map_raw!(Pointer(Motion), event),
            LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE => map_raw!(Pointer(MotionAbsolute), event),
            LIBINPUT_EVENT_POINTER_BUTTON => map_raw!(Pointer(Button), event),
            LIBINPUT_EVENT_POINTER_AXIS => map_raw!(Pointer(Axis), event),
            LIBINPUT_EVENT_POINTER_SCROLL_WHEEL => map_raw!(Pointer(ScrollWheel), event),
            LIBINPUT_EVENT_POINTER_SCROLL_FINGER => map_raw!(Pointer(ScrollFinger), event),
            LIBINPUT_EVENT_POINTER_SCROLL_CONTINUOUS => map_raw!(Pointer(ScrollContinuous), event),

            LIBINPUT_EVENT_SWITCH_TOGGLE => map_raw!(Switch(Toggle), event),

            LIBINPUT_EVENT_TABLET_PAD_BUTTON => map_raw!(TabletPad(Button), event),
            LIBINPUT_EVENT_TABLET_PAD_RING => map_raw!(TabletPad(Ring), event),
            LIBINPUT_EVENT_TABLET_PAD_STRIP => map_raw!(TabletPad(Strip), event),
            LIBINPUT_EVENT_TABLET_PAD_KEY => map_raw!(TabletPad(Key), event),
            LIBINPUT_EVENT_TABLET_PAD_DIAL => map_raw!(TabletPad(Dial), event),

            LIBINPUT_EVENT_TABLET_TOOL_AXIS => map_raw!(TabletTool(Axis), event),
            LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY => map_raw!(TabletTool(Proximity), event),
            LIBINPUT_EVENT_TABLET_TOOL_TIP => map_raw!(TabletTool(Tip), event),
            LIBINPUT_EVENT_TABLET_TOOL_BUTTON => map_raw!(TabletTool(Button), event),

            LIBINPUT_EVENT_TOUCH_DOWN => map_raw!(Touch(Down), event),
            LIBINPUT_EVENT_TOUCH_UP => map_raw!(Touch(Up), event),
            LIBINPUT_EVENT_TOUCH_MOTION => map_raw!(Touch(Motion), event),
            LIBINPUT_EVENT_TOUCH_CANCEL => map_raw!(Touch(Cancel), event),
            LIBINPUT_EVENT_TOUCH_FRAME => map_raw!(Touch(Frame), event),

            _ => Event::Unknown,
        };

        event
    }
}

pub trait AsRawEvent<T>: sealed::EventSealed {
    fn as_raw(&self) -> *mut T;
    fn as_raw_event(&self) -> *mut sys::libinput_event;

    fn device(&self) -> Device {
        unsafe { Device::from_raw(sys::libinput_event_get_device(self.as_raw_event())) }
    }
}

pub trait FromRawEvent<T>: sealed::EventSealed {
    unsafe fn from_raw(raw: *mut T) -> Self;
    unsafe fn from_raw_event(event: *mut sys::libinput_event) -> Self;
}

mod sealed {

    pub trait EventSealed {}

    macro_rules! seal {
        ($($event:ident,)+) => {
            $(
                impl EventSealed for super::$event {}
            )+
        };
    }

    seal! {
        DeviceAddedEvent,
        DeviceRemovedEvent,
        GestureSwipeBeginEvent,
        GestureSwipeUpdateEvent,
        GestureSwipeEndEvent,
        GesturePinchBeginEvent,
        GesturePinchUpdateEvent,
        GesturePinchEndEvent,
        GestureHoldBeginEvent,
        GestureHoldEndEvent,
        KeyboardKeyEvent,
        PointerMotionEvent,
        PointerMotionAbsoluteEvent,
        PointerButtonEvent,
        PointerAxisEvent,
        PointerScrollWheelEvent,
        PointerScrollFingerEvent,
        PointerScrollContinuousEvent,
        SwitchToggleEvent,
        TabletPadButtonEvent,
        TabletPadRingEvent,
        TabletPadStripEvent,
        TabletPadKeyEvent,
        TabletPadDialEvent,
        TabletToolAxisEvent,
        TabletToolProximityEvent,
        TabletToolTipEvent,
        TabletToolButtonEvent,
        TouchUpEvent,
        TouchDownEvent,
        TouchFrameEvent,
        TouchCancelEvent,
        TouchMotionEvent,
    }
}
