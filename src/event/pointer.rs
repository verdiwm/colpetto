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
