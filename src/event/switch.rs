//! Events that come from switch devices.
//!
//! Switch devices represent hardware switches like laptop lid switches,
//! tablet mode switches, and other binary state hardware.
//!
//! These switches can trigger events when the physical state of the device changes,
//! for example when a laptop lid is closed.

use crate::sys::{
    self, libinput_event_get_switch_event, libinput_event_switch,
    libinput_event_switch_get_base_event,
};

super::define_events!(
    /// A switch event representing a changed state in a switch.
    ///
    /// Switch events are generated when a physical switch on a device changes state,
    /// such as a laptop lid being opened or closed, or a laptop being converted
    /// to tablet mode.
    Switch,
    libinput_event_switch,
    libinput_event_switch_get_base_event,
    libinput_event_get_switch_event,
    /// Signals that the switch has been toggled between states.
    ///
    /// This event is sent when a switch changes its physical state.
    /// The current state can be retrieved with `switch_state()`.
    Toggle,
);

impl SwitchToggleEvent {
    /// Returns the current state of the switch.
    ///
    /// This function returns whether the switch is currently in the on or off position.
    pub fn switch_state(&self) -> SwitchState {
        SwitchState::from_raw(unsafe { sys::libinput_event_switch_get_switch_state(self.raw) })
    }
}

/// Represents the physical state of a switch device.
///
/// A switch can be in one of two states: on or off, reflecting the physical
/// position of the switch.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SwitchState {
    /// The switch is in the off position.
    Off,
    /// The switch is in the on position.
    On,
}

impl std::fmt::Display for SwitchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::On => write!(f, "on"),
        }
    }
}

impl SwitchState {
    /// Builds a new switch state from a raw libinput switch state.
    ///
    /// # Panics
    ///
    /// This function will panic if libinput returns an invalid switch state.
    pub const fn from_raw(raw: sys::libinput_switch_state) -> Self {
        match raw {
            sys::libinput_switch_state::LIBINPUT_SWITCH_STATE_OFF => Self::Off,
            sys::libinput_switch_state::LIBINPUT_SWITCH_STATE_ON => Self::On,
            _ => panic!("libinput returned an invalid switch state"), // FIXME: I dont think panicking is a good idea. Maybe we could return an option?
        }
    }
}
