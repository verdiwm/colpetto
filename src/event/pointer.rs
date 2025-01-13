use crate::sys::{
    self, libinput_event_get_pointer_event, libinput_event_pointer,
    libinput_event_pointer_get_base_event,
};

super::define_events!(
    Pointer,
    libinput_event_pointer,
    libinput_event_pointer_get_base_event,
    libinput_event_get_pointer_event,
    Motion,
    MotionAbsolute,
    Button,
    Axis,
    ScrollWheel,
    ScrollFinger,
    ScrollContinuous,
);

impl PointerMotionAbsoluteEvent {
    pub fn absolute_x(&self) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_x(self.raw) }
    }

    pub fn absolute_x_transformed(&self, width: u32) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_x_transformed(self.raw, width) }
    }

    pub fn absolute_y(&self) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_y(self.raw) }
    }

    pub fn absolute_y_transformed(&self, width: u32) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_y_transformed(self.raw, width) }
    }
}
