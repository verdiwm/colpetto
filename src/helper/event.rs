use crate::event as libinput_event;

#[derive(Debug)]
pub struct Event {
    pub name: &'static str,
    pub event_type: EventType,
    pub device_name: String,
}

#[derive(Debug)]
pub enum EventType {
    Keyboard {
        key: u32,
        state: libinput_event::KeyState,
    },
    Unknown,
}

impl From<&crate::Event> for EventType {
    fn from(value: &crate::Event) -> Self {
        match value {
            crate::Event::Keyboard(libinput_event::KeyboardEvent::Key(event)) => {
                EventType::Keyboard {
                    key: event.key(),
                    state: event.key_state(),
                }
            }
            _ => EventType::Unknown,
        }
    }
}
