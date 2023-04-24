use std::{cell::Cell, ffi::c_void, marker::PhantomData};

use ultralight_sys::*;

use super::PixelGuard;
use crate::{surface::Surface, AsULRawPtr};

pub struct GenericSurface<'a> {
    surface: ULSurface,
    lifetime: PhantomData<Cell<&'a ()>>,
}

impl<'a> GenericSurface<'a> {
    /// # Safety
    ///
    /// `surface` must be a valid pointer to a `ULSurface` instance.
    pub unsafe fn from_raw(surface: ULSurface) -> Self {
        Self {
            surface,
            lifetime: PhantomData,
        }
    }

    pub fn into_raw(self) -> ULSurface {
        self.surface
    }
}

impl AsULRawPtr<ULSurface> for GenericSurface<'_> {
    fn as_raw_ptr(&self) -> ULSurface {
        self.surface
    }
}

impl Surface for GenericSurface<'_> {
    fn width(&self) -> u32 {
        unsafe { ulSurfaceGetWidth(self.surface) }
    }

    fn height(&self) -> u32 {
        unsafe { ulSurfaceGetHeight(self.surface) }
    }

    fn row_bytes(&self) -> u32 {
        unsafe { ulSurfaceGetRowBytes(self.surface) }
    }

    fn size(&self) -> usize {
        unsafe { ulSurfaceGetSize(self.surface) }
    }

    fn pixels(&mut self) -> PixelGuard<Self> {
        PixelGuard::new(self)
    }

    unsafe fn lock_pixels(&self) -> *mut c_void {
        ulSurfaceLockPixels(self.surface)
    }

    unsafe fn unlock_pixels(&self) {
        ulSurfaceUnlockPixels(self.surface)
    }
}
