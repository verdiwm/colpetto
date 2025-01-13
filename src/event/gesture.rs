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
