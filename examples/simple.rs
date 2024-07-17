use std::ptr::{null, null_mut};

use colpetto::sys::{self, libinput_interface};

fn main() {
    unsafe {
        let interface = libinput_interface {
            open_restricted: todo!(),
            close_restricted: todo!(),
        };


        let context = sys::libinput_udev_create_context(&interface, null_mut(), udev);
    }
}
