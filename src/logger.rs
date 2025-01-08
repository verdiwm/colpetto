use std::ffi::{c_char, CStr};
use tracing::{debug, error, info};

use crate::sys;

pub type Logger = Option<unsafe extern "C" fn(sys::libinput_log_priority, *const c_char)>;

extern "C" {
    fn set_log_callback(callback: Logger);
    fn get_log_handler() -> sys::libinput_log_handler;
}

pub unsafe extern "C" fn tracing_logger(
    priority: sys::libinput_log_priority,
    message: *const c_char,
) {
    let message = CStr::from_ptr(message).to_string_lossy();

    match priority {
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_INFO => info!("{message}"),
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_DEBUG => debug!("{message}"),
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_ERROR => error!("{message}"),
        _ => println!("{message}"),
    }
}

pub(crate) fn setup_logger(libinput: *mut sys::libinput, logger: Logger) {
    if logger.is_none() {
        return;
    }

    unsafe {
        set_log_callback(logger);
    }

    unsafe {
        sys::libinput_log_set_priority(
            libinput,
            sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_DEBUG,
        );
    }

    unsafe {
        sys::libinput_log_set_handler(libinput, get_log_handler());
    }
}
