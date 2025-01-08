use std::ffi::CString;

use anyhow::Result as AnyResult;
use colpetto::Libinput;
use futures_util::TryStreamExt;
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{open, Mode, OFlags},
    io::Errno,
};

#[tokio::main]
async fn main() -> AnyResult<()> {
    tracing_subscriber::fmt::init();

    let libinput = Libinput::with_logger(
        |path, flags| {
            open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
                .map(IntoRawFd::into_raw_fd)
                .map_err(Errno::raw_os_error)
        },
        |fd| drop(unsafe { OwnedFd::from_raw_fd(fd) }),
        Some(colpetto::tracing_logger),
    )?;
    libinput.assign_seat(CString::new("seat0").unwrap().as_c_str())?;

    let mut stream = libinput.event_stream()?;

    while let Some(event) = stream.try_next().await? {
        dbg!(event);
    }

    Ok(())
}
