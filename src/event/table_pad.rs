//! Events that come from the pad of tablet devices.

use crate::sys::{
    libinput_event_get_tablet_pad_event, libinput_event_tablet_pad,
    libinput_event_tablet_pad_get_base_event,
};

// FIXME: better docs
super::define_events!(
    /// Tablet pad event representing a button press, or ring/strip update on the tablet pad itself.
    TabletPad,
    libinput_event_tablet_pad,
    libinput_event_tablet_pad_get_base_event,
    libinput_event_get_tablet_pad_event,
    /// A button pressed on a device
    Button,
    /// A status change on a tablet ring
    Ring,
    /// A status change on a strip on a device
    Strip,
    /// A key pressed on a device
    Key,
    /// A status change on a tablet dial
    Dial,
);
