use std::{mem::ManuallyDrop, ops::Deref, path::Path, sync::RwLock};

use ultralight_sys::*;

use crate::{buffer::Buffer, string::UString};

macro_rules! lock_fail {
    () => {
        panic!("NOTICE: file system lock poisoned, some crash occured in file system");
    };
}

macro_rules! unreachable_fail {
    () => {
        unreachable!("NOTICE: USER_FILE_SYSTEM must be provided, this is a internal error");
    };
}

static USER_FILE_SYSTEM: RwLock<Option<Box<dyn FileSystem>>> = RwLock::new(None);
static FILE_SYSTEM: ULFileSystem = ULFileSystem {
    file_exists: Some(file_exists_callback),
    get_file_mime_type: Some(get_file_mime_type_callback),
    get_file_charset: Some(get_file_charset_callback),
    open_file: Some(open_file_callback),
};

unsafe extern "C" fn file_exists_callback(path: ULString) -> bool {
    let guard = match USER_FILE_SYSTEM.read() {
        Ok(guard) => guard,
        Err(_) => {
            lock_fail!();
        }
    };

    let file_system = match guard.as_ref() {
        Some(file_system) => file_system,
        None => {
            unreachable_fail!();
        }
    };

    let path = ManuallyDrop::new(UString::from_raw(path));
    let path = Path::new(path.deref().deref());

    file_system.file_exists(path)
}

unsafe extern "C" fn get_file_mime_type_callback(path: ULString) -> ULString {
    let guard = match USER_FILE_SYSTEM.read() {
        Ok(guard) => guard,
        Err(_) => {
            lock_fail!();
        }
    };

    let file_system = match guard.as_ref() {
        Some(file_system) => file_system,
        None => {
            unreachable_fail!();
        }
    };

    let path = ManuallyDrop::new(UString::from_raw(path));
    let path = Path::new(path.deref().deref());

    UString::from(file_system.get_file_mime_type(path)).into_raw()
}

unsafe extern "C" fn get_file_charset_callback(path: ULString) -> ULString {
    let guard = match USER_FILE_SYSTEM.read() {
        Ok(guard) => guard,
        Err(_) => {
            lock_fail!();
        }
    };

    let file_system = match guard.as_ref() {
        Some(file_system) => file_system,
        None => {
            unreachable_fail!();
        }
    };

    let path = ManuallyDrop::new(UString::from_raw(path));
    let path = Path::new(path.deref().deref());

    UString::from(file_system.get_file_charset(path)).into_raw()
}

unsafe extern "C" fn open_file_callback(path: ULString) -> ULBuffer {
    let guard = match USER_FILE_SYSTEM.read() {
        Ok(guard) => guard,
        Err(_) => {
            lock_fail!();
        }
    };

    let file_system = match guard.as_ref() {
        Some(file_system) => file_system,
        None => {
            unreachable_fail!();
        }
    };

    let path = ManuallyDrop::new(UString::from_raw(path));
    let path = Path::new(path.deref().deref());

    file_system.open_file(path).into_raw()
}

pub trait FileSystem: Send + Sync {
    /// The callback invoked when the FileSystem wants to check if a file path exists,
    /// return [`true`] if it exists.
    fn file_exists(&self, path: &Path) -> bool;

    /// Get the mime-type of the file (eg "text/html").
    ///
    /// This is usually determined by analyzing the file extension.
    ///
    /// If a mime-type cannot be determined, you should return "application/unknown" for this value.
    ///
    /// The library will consume the result and call ulDestroyString() after this call returns.
    fn get_file_mime_type(&self, path: &Path) -> String;

    ///  Get the charset / encoding of the file (eg "utf-8").
    ///
    /// This is only important for text-based files and is usually determined by analyzing the
    /// contents of the file.
    ///
    /// If a charset cannot be determined, it's usually safe to return "utf-8" for this value.
    ///
    /// The library will consume the result and call ulDestroyString() after this call returns.
    fn get_file_charset(&self, path: &Path) -> String;

    /// Open file for reading and map it to a Buffer.
    ///
    /// To minimize copies, you should map the requested file into memory and use ulCreateBuffer()
    /// to wrap the data pointer (unmapping should be performed in the destruction callback).
    ///
    /// If the file was unable to be opened, you should return NULL for this value.
    fn open_file(&self, path: &Path) -> Buffer;
}

pub fn set_platform_file_system(fs: Box<dyn FileSystem>) {
    match USER_FILE_SYSTEM.write() {
        Ok(mut guard) => guard.insert(fs),
        Err(_) => {
            lock_fail!();
        }
    };

    unsafe {
        ulPlatformSetFileSystem(FILE_SYSTEM);
    }
}
