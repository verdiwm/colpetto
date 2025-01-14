use colpetto::{event::AsRawEvent, Libinput, Result};
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{open, Mode, OFlags},
    io::Errno,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut libinput = Libinput::new(
        |path, flags| {
            open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
                .map(IntoRawFd::into_raw_fd)
                .map_err(Errno::raw_os_error)
        },
        |fd| drop(unsafe { OwnedFd::from_raw_fd(fd) }),
    )?;
    libinput.assign_seat(c"seat0")?;

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
