use crate::event as libinput_event;

#[derive(Debug)]
pub struct Event {
    pub name: &'static str,
    pub event_type: EventType,
    pub device_name: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EventType {
    Keyboard(KeyboardEvent),
    Unknown,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum KeyboardEvent {
    Key {
        key: u32,
        state: libinput_event::KeyState,
        time: u64,
    },
}

impl From<&crate::Event> for EventType {
    fn from(value: &crate::Event) -> Self {
        match value {
            crate::Event::Keyboard(libinput_event::KeyboardEvent::Key(event)) => {
                EventType::Keyboard(KeyboardEvent::Key {
                    key: event.key(),
                    state: event.key_state(),
                    time: event.time_usec(),
                })
            }
            _ => EventType::Unknown,
        }
    }
}
