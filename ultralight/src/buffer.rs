use std::{ffi::c_void, mem, ops::Deref, ptr, rc::Rc, slice};

use ultralight_sys::*;

// we store the data length as a userdata address
unsafe extern "C" fn destroy_borrowed_buffer(user_data: *mut c_void, data: *mut c_void) {
    let data_ptr = ptr::slice_from_raw_parts(data as *const u8, user_data as usize);
    drop(Rc::from_raw(data_ptr))
}

#[derive(Debug)]
pub struct Buffer(ULBuffer);

impl Buffer {
    pub fn new_owned<T: AsRef<[u8]>>(data: T) -> Self {
        let data = data.as_ref();
        let buffer = unsafe { ulCreateBufferFromCopy(data.as_ptr().cast(), data.len()) };
        Self(buffer)
    }

    pub fn new_borrowed(data: Rc<[u8]>) -> Self {
        let data_len = data.len(); // fat ptr len is unstable, so get it from slice
        let data = Rc::into_raw(data);
        let buffer = unsafe {
            ulCreateBuffer(
                data as *mut _,
                data_len,
                // we store the data length as a userdata address
                data_len as _,
                Some(destroy_borrowed_buffer),
            )
        };
        Self(buffer)
    }

    pub fn owns_data(&self) -> bool {
        let Self(buffer) = self;
        unsafe { ulBufferOwnsData(*buffer) }
    }

    pub fn into_raw(self) -> ULBuffer {
        let Self(buffer) = self;
        mem::forget(self);
        buffer
    }

    /// # Safety
    ///
    /// Input must be a valid ULBuffer
    pub unsafe fn from_raw(buffer: ULBuffer) -> Self {
        Self(buffer)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let Self(buffer) = self;
        unsafe { ulDestroyBuffer(*buffer) }
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let Self(buffer) = self;
        let data = unsafe { ulBufferGetData(*buffer) };
        let len = unsafe { ulBufferGetSize(*buffer) };
        unsafe { slice::from_raw_parts(data.cast(), len) }
    }
}
