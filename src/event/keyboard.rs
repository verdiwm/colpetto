use crate::sys;

#[derive(Debug)]
pub enum KeyboardEvent {
    Key(KeyboardKeyEvent),
}

#[derive(Debug)]
pub struct KeyboardKeyEvent {
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

impl KeyboardKeyEvent {
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

impl super::AsRawEvent for KeyboardKeyEvent {
    fn as_raw_event(&self) -> *mut sys::libinput_event {
        unsafe { sys::libinput_event_keyboard_get_base_event(self.raw) }
    }
}

impl super::FromRawEvent for KeyboardKeyEvent {
    unsafe fn from_raw_event(event: *mut sys::libinput_event) -> Self {
        Self {
            raw: sys::libinput_event_get_keyboard_event(event),
        }
    }
}
