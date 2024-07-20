use std::ffi::CString;

use anyhow::Result as AnyResult;
use colpetto::Libinput;
use futures_util::TryStreamExt;
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{open, Mode, OFlags},
};

#[tokio::main]
async fn main() -> AnyResult<()> {
    let libinput = Libinput::new(
        |path, flags| match open(path, OFlags::from_bits_retain(flags as u32), Mode::empty()) {
            Ok(fd) => fd.into_raw_fd(),
            Err(errno) => errno.raw_os_error(),
        },
        |fd| drop(unsafe { OwnedFd::from_raw_fd(fd) }),
    )?;
    libinput.assign_seat(CString::new("seat0").unwrap().as_c_str())?;

    let mut stream = libinput.event_stream()?;

    while let Some(event) = stream.try_next().await? {
        dbg!(event);
    }

    Ok(())
}
