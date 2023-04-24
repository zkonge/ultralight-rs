use std::{mem::ManuallyDrop, ops::Deref, sync::RwLock};

use ultralight_sys::*;

use crate::string::UString;

macro_rules! lock_fail {
    () => {
        println!("NOTICE: logger lock poisoned, some crash occured in logger");
        return;
    };
}

static USER_LOGGER: RwLock<Option<UserLogger>> = RwLock::new(None);
static LOGGER: ULLogger = ULLogger {
    log_message: Some(logger_callback),
};

unsafe extern "C" fn logger_callback(level: ULLogLevel, message: ULString) {
    let guard = match USER_LOGGER.read() {
        Ok(guard) => guard,
        Err(_) => {
            lock_fail!();
        }
    };

    let logger = match *guard {
        Some(logger) => logger,
        None => return,
    };

    let level = match level {
        0 => LogLevel::Error,
        1 => LogLevel::Warning,
        2 => LogLevel::Info,
        _ => unreachable!(),
    };

    let message = ManuallyDrop::new(unsafe { UString::from_raw(message) });

    logger(level, message.deref());
}

pub type UserLogger = fn(LogLevel, &str);

// SAFETY: ultralight can only run on systems, that c_int = i32.
#[derive(Debug)]
#[repr(i32)]
pub enum LogLevel {
    Error = ULLogLevel_kLogLevel_Error as i32,
    Warning = ULLogLevel_kLogLevel_Warning as i32,
    Info = ULLogLevel_kLogLevel_Info as i32,
}

pub fn set_platform_logger(logger: UserLogger) {
    match USER_LOGGER.write() {
        Ok(mut guard) => guard.insert(logger),
        Err(_) => {
            lock_fail!();
        }
    };

    unsafe {
        ulPlatformSetLogger(LOGGER);
    }
}
