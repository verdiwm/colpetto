use crate::sys::{
    libinput_event_get_tablet_pad_event, libinput_event_tablet_pad,
    libinput_event_tablet_pad_get_base_event,
};

use super::define_events;

#[derive(Debug)]
pub enum TabletPadEvent {
    Button(TabletPadButtonEvent),
    Ring(TabletPadRingEvent),
    Strip(TabletPadStripEvent),
    Key(TabletPadKeyEvent),
    Dial(TabletPadDialEvent),
}

define_events!(
    libinput_event_tablet_pad,
    libinput_event_tablet_pad_get_base_event,
    libinput_event_get_tablet_pad_event,
    TabletPadButtonEvent,
    TabletPadRingEvent,
    TabletPadStripEvent,
    TabletPadKeyEvent,
    TabletPadDialEvent,
);
