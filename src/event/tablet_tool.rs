//! Events that come from tools on tablet devices.

use crate::sys::{
    libinput_event_get_tablet_tool_event, libinput_event_tablet_tool,
    libinput_event_tablet_tool_get_base_event,
};

// FIXME: better docs
super::define_events!(
    /// Tablet tool event representing an axis update, button press, or tool update
    TabletTool,
    libinput_event_tablet_tool,
    libinput_event_tablet_tool_get_base_event,
    libinput_event_get_tablet_tool_event,
    /// One or more axes have changed state on a device
    Axis,
    /// Signals that a tool has come in or out of proximity of a device
    Proximity,
    /// Signals that a tool has come in contact with the surface of a device
    Tip,
    /// Signals that a tool has changed a logical button state on a device
    Button,
);
