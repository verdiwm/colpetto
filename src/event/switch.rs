use crate::sys::{
    libinput_event_get_switch_event, libinput_event_switch, libinput_event_switch_get_base_event,
};

super::define_events!(
    Switch,
    libinput_event_switch,
    libinput_event_switch_get_base_event,
    libinput_event_get_switch_event,
    Toggle,
);
