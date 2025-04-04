# Colpetto

Colpetto is a modern Rust library that provides asynchronous bindings for
libinput, enabling efficient handling of input device events on Linux systems.
Built on tokio, it offers a stream-based API that seamlessly integrates with
async applications while maintaining low latency and efficient resource usage
for real-time operations.

## Key Features

Colpetto transforms libinput's traditional callback-based interface into an
ergonomic Rust API with:

- Stream-based event handling powered by tokio
- Low-overhead event polling optimized for performance
- Flexible logging system with built-in basic loggers

## Examples

Here's a basic example demonstrating event handling using rustix:

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

For more examples, check out the [examples directory](examples/):

- [Simple](examples/simple.rs): Basic event handling demonstration
- [Print Keys](examples/print_keys.rs): Advanced example with custom logging and
  keyboard event handling
- [Task](examples/task.rs): Showcases how to work around libinput's thread
  safety limitations in tokio tasks

<!-- - [Device Management](examples/devices.rs): Device detection and configuration example
- [Multi-seat Setup](examples/seats.rs): Multiple seat configuration handling -->

## Supported libinput Versions

Colpetto requires libinput 1.22 or higher (current version in Debian stable).

Contributions to support older versions are welcome provided they:

- Don't compromise the existing async functionality
- Maintain API consistency
- Include appropriate version-specific tests if applicable

If you're using an older version of libinput, you may need to upgrade your
system packages or consider using input-rs which supports earlier versions.

## Comparison with input-rs

While [input-rs](https://github.com/Smithay/input.rs) is an established
alternative, Colpetto takes a different approach in several key areas:

### Interface Design

Colpetto uses direct function passing for device management, while input-rs
employs a trait-based approach:

```rust
// Colpetto: Function-based approach with closure support
let mut libinput = Libinput::new(
    |path, flags| {  // Open function
        open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
            .map(IntoRawFd::into_raw_fd)
            .map_err(Errno::raw_os_error)
    },
    |fd| drop(unsafe { OwnedFd::from_raw_fd(fd) })  // Close function
)?;

// input-rs: Trait-based approach
struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
            .map_err(Errno::raw_os_error)
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
}
let mut input = Libinput::new_with_udev(Interface);
```

### Version Support

While input-rs supports a wider range of libinput versions (1.9+), Colpetto
focuses on modern versions (1.22+) to provide comprehensive bindings for newer
libinput features and async capabilities.

Colpetto also requires a modern rust version since it depends on edition 2024
which was introduced on rust 1.85

### Some other differences

- Native tokio integration for async/await support with stream-based event
  handling via `event_stream()`
- Safe handling of non-UTF8 strings using `CStr` instead of implicit panics
- Comprehensive event type safety through more exhaustive enum matching
- More robust context lifetime management

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
