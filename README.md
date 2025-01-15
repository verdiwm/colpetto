# Colpetto

Colpetto provides async Rust bindings for libinput, enabling seamless handling
of input device events on Linux systems. By integrating with tokio, Colpetto
offers a modern stream-based API that naturally fits into async applications
while ensuring efficient system resource usage and low latency in real time
applications.

## Key Features

Colpetto transforms libinput's traditional callback-based interface into an
ergonomic Rust API:

- Stream-based event handling through tokio
- Safe management of libinput resources and contexts
- Efficient event polling with minimal CPU overhead
- Comprehensive type safety around libinput's event types
- Support for custom loggers with basic ones provided

## Examples

Here's a simple example that uses rustix to open devices:

```rust
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
```

You can find more examples in the [examples directory](examples/), including:

- [Simple](examples/simple.rs): The example above, demostrates basic event
  handling
- [Print Keys](examples/print_keys.rs): A more complex example that showcases
  custom loggers and basic keyboard event handling

<!-- - [Device Management](examples/devices.rs): Shows device detection and
  configuration
- [Multi-seat Setup](examples/seats.rs): Illustrates handling multiple seat
  configurations -->

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
