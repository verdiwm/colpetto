use std::{
    ffi::{c_int, CStr},
    os::fd::RawFd,
};

use colpetto::{event::AsRawEvent, Libinput, Result};
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{open, Mode, OFlags},
    io::Errno,
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
    let mut libinput = Libinput::new(open_restricted, close_restricted)?;

    libinput.udev_assign_seat(c"seat0")?;

    let mut stream = libinput.event_stream()?;
    while let Some(event) = stream.try_next().await? {
        println!(
            "Got \"{}\" event from \"{}\"",
            event.event_type(),
            event.device().name().to_string_lossy()
        )
    }

    Ok(())
}
