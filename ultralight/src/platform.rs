use std::path::Path;

use ultralight_sys::*;

use crate::{string::UString, AsULRawPtr};

pub fn enable_platform_font_loader() {
    unsafe {
        ulEnablePlatformFontLoader();
    }
}

pub fn enable_default_logger(path: &Path) {
    let logger_path = UString::from(path.to_string_lossy());
    unsafe { ulEnableDefaultLogger(logger_path.as_raw_ptr()) };
}

/// NOTICE: the file system must provide the ICU data file, or it will make program exit.
pub fn enable_platform_file_system(path: &Path) {
    let base_dir = UString::from(path.to_string_lossy());
    unsafe { ulEnablePlatformFileSystem(base_dir.as_raw_ptr()) }
}
