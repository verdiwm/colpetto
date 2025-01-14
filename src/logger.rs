use std::ffi::c_char;

use crate::sys;

/// The function type to pass as a logger to libinput
pub type Logger = Option<unsafe extern "C" fn(sys::libinput_log_priority, *const c_char)>;

/// Provides a simple logger that redirects libinput events as tracing events
#[cfg(feature = "tracing")]
pub unsafe extern "C" fn tracing_logger(
    priority: sys::libinput_log_priority,
    message: *const c_char,
) {
    use std::ffi::CStr;
    use tracing::{debug, error, info, trace};

    let message = CStr::from_ptr(message).to_string_lossy();

    match priority {
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_INFO => info!("{message}"),
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_DEBUG => debug!("{message}"),
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_ERROR => error!("{message}"),
        _ => trace!("{message}"),
    }
}

/// Provides a simple logger that redirects libinput events as log events
#[cfg(feature = "log")]
pub unsafe extern "C" fn log_logger(priority: sys::libinput_log_priority, message: *const c_char) {
    use log::{debug, error, info, trace};
    use std::ffi::CStr;

    let message = CStr::from_ptr(message).to_string_lossy();

    match priority {
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_INFO => info!("{message}"),
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_DEBUG => debug!("{message}"),
        sys::libinput_log_priority::LIBINPUT_LOG_PRIORITY_ERROR => error!("{message}"),
        _ => trace!("{message}"),
    }
}

extern "C" {
    fn set_log_callback(callback: Logger);
    fn get_log_handler() -> sys::libinput_log_handler;
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
