//! Events that come from switch devices.

use crate::sys::{
    libinput_event_get_switch_event, libinput_event_switch, libinput_event_switch_get_base_event,
};

super::define_events!(
    /// A switch event representing a changed state in a switch.
    Switch,
    libinput_event_switch,
    libinput_event_switch_get_base_event,
    libinput_event_get_switch_event,
    /// Signals that the switch has been toggled
    Toggle,
);
