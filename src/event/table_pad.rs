use crate::sys::{
    libinput_event_get_tablet_pad_event, libinput_event_tablet_pad,
    libinput_event_tablet_pad_get_base_event,
};

super::define_events!(
    TabletPad,
    libinput_event_tablet_pad,
    libinput_event_tablet_pad_get_base_event,
    libinput_event_get_tablet_pad_event,
    Button,
    Ring,
    Strip,
    Key,
    Dial,
);
