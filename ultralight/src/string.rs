use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
    slice::from_raw_parts,
};

use ultralight_sys::*;

use crate::AsULRawPtr;

pub struct UString(ULString);

impl UString {
    pub fn into_raw(self) -> ULString {
        let s = self.0;
        std::mem::forget(self);
        s
    }

    pub unsafe fn from_raw(s: ULString) -> Self {
        Self(s)
    }
}

impl AsULRawPtr<ULString> for UString {
    fn as_raw_ptr(&self) -> ULString {
        self.0
    }
}

impl<T: AsRef<str>> From<T> for UString {
    fn from(value: T) -> Self {
        let s = value.as_ref();
        Self(unsafe { ulCreateStringUTF8(s.as_ptr().cast(), s.len()) })
    }
}

impl Drop for UString {
    fn drop(&mut self) {
        unsafe { ulDestroyString(self.0) }
    }
}

impl Deref for UString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let s = self.0;
        unsafe {
            let ptr = ulStringGetData(s).cast();
            let len = ulStringGetLength(s);
            std::str::from_utf8_unchecked(from_raw_parts(ptr, len))
        }
    }
}

impl Debug for UString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl Display for UString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.deref(), f)
    }
}
