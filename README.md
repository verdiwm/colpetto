# Colpetto

Colpetto is an async wrapper for libinput. It leverages the tokio runtime to
offer a robust stream-based implementation for handling libinput events.

Integrating with the async ecosystem allows for efficient polling of events,
minimizing CPU consumption and maximizes performance in applications that
require real-time event handling.

You can find a simple example usage example [here](examples/print_keys.rs)

Currently, Colpetto utilizes udev as the backend for device discovery and
management. Future updates will include support for additional custom backends,
allowing for greater flexibility.

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
