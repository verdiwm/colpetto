use crate::sys::{
    self, libinput_event_get_keyboard_event, libinput_event_keyboard,
    libinput_event_keyboard_get_base_event,
};

super::define_events!(
    Keyboard,
    libinput_event_keyboard,
    libinput_event_keyboard_get_base_event,
    libinput_event_get_keyboard_event,
    Key,
);

impl KeyboardKeyEvent {
    pub fn key(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_key(self.raw) }
    }

    pub fn key_state(&self) -> KeyState {
        KeyState::from_raw(unsafe { sys::libinput_event_keyboard_get_key_state(self.raw) })
    }

    pub fn seat_key_count(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_seat_key_count(self.raw) }
    }

    pub fn time(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_time(self.raw) }
    }

    pub fn time_usec(&self) -> u64 {
        unsafe { sys::libinput_event_keyboard_get_time_usec(self.raw) }
    }
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
