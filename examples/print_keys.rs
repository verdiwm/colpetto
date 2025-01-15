use colpetto::{
    event::{AsRawEvent, DeviceEvent, KeyboardEvent},
    Event, Libinput, Result,
};
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{open, Mode, OFlags},
    io::Errno,
};
use std::{
    ffi::{c_int, CStr},
    os::fd::RawFd,
    path::Path,
};
use tokio_stream::StreamExt;

fn open_restricted(path: &CStr, flags: c_int) -> Result<RawFd, c_int> {
    open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
        .map(IntoRawFd::into_raw_fd)
        .map_err(Errno::raw_os_error)
}

fn close_restricted(fd: RawFd) {
    drop(unsafe { OwnedFd::from_raw_fd(fd) });
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut libinput = Libinput::with_logger(
        open_restricted,
        close_restricted,
        Some(colpetto::tracing_logger),
    )?;
    libinput.udev_assign_seat(c"seat0")?;

    let mut stream = libinput.event_stream()?;

    while let Some(event) = stream.try_next().await? {
        match event {
            Event::Keyboard(KeyboardEvent::Key(event_key)) => {
                let device = event_key.device();
                let device_name = device.name().to_string_lossy();
                let udev_device = device.udev_device().unwrap();
                let device_path = udev_device.devnode().unwrap_or(Path::new("Unknown"));
                let state = event_key.key_state();
                let key = event_key.key();

                println!(
                    "Key \"{key}\" {state} on \"{device_name}\" at node \"{device_path}\"",
                    device_path = device_path.display()
                );
            }
            Event::Device(DeviceEvent::Added(event)) => {
                let device = event.device();
                let name = device.name().to_string_lossy();

                println!("Found new device: \"{name}\"");
            }
            Event::Device(DeviceEvent::Removed(event)) => {
                let device = event.device();
                let name = device.name().to_string_lossy();

                println!("Lost device: \"{name}\"");
            }
            _ => (),
        }
    }

    Ok(())
}
