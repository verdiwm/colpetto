//! Key events are generated when a key changes its logical state, usually by being pressed or released.

use crate::sys::{
    self, libinput_event_get_keyboard_event, libinput_event_keyboard,
    libinput_event_keyboard_get_base_event,
};

super::define_events!(
    /// A keyboard event representing a key press/release.
    Keyboard,
    libinput_event_keyboard,
    libinput_event_keyboard_get_base_event,
    libinput_event_get_keyboard_event,
    /// Signals that a device has been removed. No more events from the associated device will be in the queue or be queued after this event.
    Key,
);

impl KeyboardKeyEvent {
    /// The keycode that triggered this key event
    pub fn key(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_key(self.raw) }
    }

    /// The state change of the key
    pub fn key_state(&self) -> KeyState {
        KeyState::from_raw(unsafe { sys::libinput_event_keyboard_get_key_state(self.raw) })
    }

    /// The seat wide pressed key count for the key of this event
    pub fn seat_key_count(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_seat_key_count(self.raw) }
    }

    /// The event time for this event
    ///
    /// # Note
    ///
    /// Timestamps may not always increase. See the libinput documentation for more details.
    pub fn time(&self) -> u32 {
        unsafe { sys::libinput_event_keyboard_get_time(self.raw) }
    }

    /// The event time for this event in microseconds
    ///
    /// # Note
    ///
    /// Timestamps may not always increase. See the libinput documentation for more details.
    pub fn time_usec(&self) -> u64 {
        unsafe { sys::libinput_event_keyboard_get_time_usec(self.raw) }
    }
}

/// Logical state of a key. Note that the logical state may not represent the physical state of the key.
pub enum KeyState {
    /// Logical released state
    Released,
    /// Logical pressed state
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
    /// Builds a new keystate from a raw libinput one
    pub const fn from_raw(raw: sys::libinput_key_state) -> Self {
        match raw {
            sys::libinput_key_state::LIBINPUT_KEY_STATE_RELEASED => Self::Released,
            sys::libinput_key_state::LIBINPUT_KEY_STATE_PRESSED => Self::Pressed,
            _ => panic!("libinput returned an invalid keystate"), // FIXME: I dont think panicking is a good idea. Maybe we could return an option?
        }
    }
}
