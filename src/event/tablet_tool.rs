use crate::sys::{
    libinput_event_get_tablet_tool_event, libinput_event_tablet_tool,
    libinput_event_tablet_tool_get_base_event,
};

super::define_events!(
    TabletTool,
    libinput_event_tablet_tool,
    libinput_event_tablet_tool_get_base_event,
    libinput_event_get_tablet_tool_event,
    Axis,
    Proximity,
    Tip,
    Button,
);
