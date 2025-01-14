use crate::sys::{
    self, libinput_event_get_pointer_event, libinput_event_pointer,
    libinput_event_pointer_get_base_event,
};

super::define_events!(
    /// A pointer event representing relative or absolute pointer movement, a button press/release or scroll axis events.
    Pointer,
    libinput_event_pointer,
    libinput_event_pointer_get_base_event,
    libinput_event_get_pointer_event,
    /// Signals that a device has been removed. No more events from the associated device will be in the queue or be queued after this event.
    Motion,
    /// Signals that a device has been removed. No more events from the associated device will be in the queue or be queued after this event.
    MotionAbsolute,
    /// Signals that a device has been removed. No more events from the associated device will be in the queue or be queued after this event.
    Button,
    Axis,
    ScrollWheel,
    ScrollFinger,
    ScrollContinuous,
);

impl PointerMotionAbsoluteEvent {
    /// Tthe current absolute x coordinate of the pointer event, in mm from the top left corner of the device.
    /// To get the corresponding output screen coordinate, use [`absolute_x_transformed`](Self::absolute_x_transformed).
    pub fn absolute_x(&self) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_x(self.raw) }
    }

    /// The current absolute x coordinate of the pointer event, transformed to screen coordinates.
    pub fn absolute_x_transformed(&self, width: u32) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_x_transformed(self.raw, width) }
    }

    /// The current absolute y coordinate of the pointer event, in mm from the top left corner of the device.
    /// To get the corresponding output screen coordinate, use [`absolute_y_transformed`](Self::absolute_y_transformed).
    pub fn absolute_y(&self) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_y(self.raw) }
    }

    /// The current absolute y coordinate of the pointer event, transformed to screen coordinates.
    pub fn absolute_y_transformed(&self, width: u32) -> f64 {
        unsafe { sys::libinput_event_pointer_get_absolute_y_transformed(self.raw, width) }
    }
}
