use crate::sys::{
    libinput_event_get_switch_event, libinput_event_switch, libinput_event_switch_get_base_event,
};

use super::define_events;

#[derive(Debug)]
pub enum SwitchEvent {
    Toggle(SwitchToggleEvent),
}

define_events!(
    libinput_event_switch,
    libinput_event_switch_get_base_event,
    libinput_event_get_switch_event,
    SwitchToggleEvent,
);
