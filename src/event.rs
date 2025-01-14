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
    ($main:ident, $raw:ident, $get:expr, $set:expr, $($event:ident,)+) => {
        paste::paste! {
            #[derive(Debug)]
            pub enum [<$main Event>] {
                $(
                    $event([<$main $event Event>]),
                )+
            }

            impl crate::event::sealed::EventSealed for [<$main Event>] {}

            impl crate::event::AsRawEvent for [<$main Event>] {
                fn as_raw_event(&self) -> *mut crate::sys::libinput_event {
                    match self {
                        $(
                            Self::$event(e) => e.as_raw_event(),
                        )+
                    }
                }
            }

            $(
                pub struct [<$main $event Event>] {
                    raw: *mut $raw,
                }


                impl std::fmt::Debug for [<$main $event Event>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f,stringify!($event))
                    }
                }

                impl crate::event::sealed::EventSealed for [<$main $event Event>] {}

                impl crate::event::AsRawEvent for [<$main $event Event>] {
                    fn as_raw_event(&self) -> *mut crate::sys::libinput_event {
                        unsafe { $get(self.raw) }
                    }
                }

                impl crate::event::FromRawEvent for [<$main $event Event>] {
                    unsafe fn from_raw_event(event: *mut crate::sys::libinput_event) -> Self {
                        Self {
                            raw: $set(event),
                        }
                    }
                }

                impl Drop for [<$main $event Event>] {
                    fn drop(&mut self) {
                        unsafe {
                            crate::sys::libinput_event_destroy(crate::event::AsRawEvent::as_raw_event(self));
                        }
                    }
                }

            )+
        }
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
    Unknown(Unknown),
}

impl sealed::EventSealed for Event {}

impl AsRawEvent for Event {
    fn as_raw_event(&self) -> *mut sys::libinput_event {
        match self {
            Event::Device(e) => e.as_raw_event(),
            Event::Gesture(e) => e.as_raw_event(),
            Event::Keyboard(e) => e.as_raw_event(),
            Event::Pointer(e) => e.as_raw_event(),
            Event::Switch(e) => e.as_raw_event(),
            Event::TabletPad(e) => e.as_raw_event(),
            Event::TabletTool(e) => e.as_raw_event(),
            Event::Touch(e) => e.as_raw_event(),
            Event::Unknown(e) => e.as_raw_event(),
        }
    }
}

#[derive(Debug)]
pub struct Unknown {
    #[allow(unused)]
    raw: *mut sys::libinput_event,
}

impl sealed::EventSealed for Unknown {}

impl AsRawEvent for Unknown {
    fn as_raw_event(&self) -> *mut sys::libinput_event {
        self.raw
    }
}

impl Drop for Unknown {
    fn drop(&mut self) {
        unsafe {
            sys::libinput_event_destroy(self.raw);
        }
    }
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
    pub unsafe fn from_raw(
        event: *mut sys::libinput_event,
        event_type: sys::libinput_event_type::Type,
    ) -> Self {
        use sys::libinput_event_type::*;

        match event_type {
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

            _ => Event::Unknown(Unknown { raw: event }),
        }
    }
}

pub trait AsRawEvent: sealed::EventSealed {
    // fn as_raw(&self) -> *mut T;
    fn as_raw_event(&self) -> *mut sys::libinput_event;

    fn device(&self) -> Device {
        unsafe { Device::from_raw(sys::libinput_event_get_device(self.as_raw_event())) }
    }
}

pub trait FromRawEvent: sealed::EventSealed {
    // unsafe fn from_raw(raw: *mut T) -> Self;
    unsafe fn from_raw_event(event: *mut sys::libinput_event) -> Self;
}

mod sealed {

    pub trait EventSealed {}
}
