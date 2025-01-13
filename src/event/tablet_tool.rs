use crate::sys::{
    libinput_event_get_tablet_tool_event, libinput_event_tablet_tool,
    libinput_event_tablet_tool_get_base_event,
};

use super::define_events;

#[derive(Debug)]
pub enum TabletToolEvent {
    Axis(TabletToolAxisEvent),
    Proximity(TabletToolProximityEvent),
    Tip(TabletToolTipEvent),
    Button(TabletToolButtonEvent),
}

define_events!(
    libinput_event_tablet_tool,
    libinput_event_tablet_tool_get_base_event,
    libinput_event_get_tablet_tool_event,
    TabletToolAxisEvent,
    TabletToolProximityEvent,
    TabletToolTipEvent,
    TabletToolButtonEvent,
);
