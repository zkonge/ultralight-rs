use std::{
    ffi::{c_void, CString},
    path::Path,
};

use ultralight_sys::*;

use crate::{
    surface::{generic_surface::GenericSurface, PixelGuard, Surface},
    AsULRawPtr,
};

pub struct BitmapSurface<'a> {
    bitmap: ULBitmap,
    _base_surface: GenericSurface<'a>,
}

impl<'a> BitmapSurface<'a> {
    pub fn write_png(&self, path: &Path) {
        let path = CString::new(path.to_string_lossy().as_bytes()).unwrap();
        unsafe {
            ulBitmapWritePNG(self.bitmap, path.as_ptr());
        }
    }

    pub fn swap_red_blue(&mut self) {
        unsafe {
            ulBitmapSwapRedBlueChannels(self.bitmap);
        }
    }
}

impl<'a> From<GenericSurface<'a>> for BitmapSurface<'a> {
    fn from(surface: GenericSurface<'a>) -> Self {
        let bitmap = unsafe { ulBitmapSurfaceGetBitmap(surface.as_raw_ptr()) };
        Self {
            bitmap,
            _base_surface: surface,
        }
    }
}

impl Surface for BitmapSurface<'_> {
    fn width(&self) -> u32 {
        unsafe { ulBitmapGetWidth(self.bitmap) }
    }

    fn height(&self) -> u32 {
        unsafe { ulBitmapGetHeight(self.bitmap) }
    }

    fn row_bytes(&self) -> u32 {
        unsafe { ulBitmapGetRowBytes(self.bitmap) }
    }

    fn size(&self) -> usize {
        unsafe { ulBitmapGetSize(self.bitmap) }
    }

    fn pixels(&mut self) -> PixelGuard<Self> {
        PixelGuard::new(self)
    }

    unsafe fn lock_pixels(&self) -> *mut c_void {
        ulBitmapLockPixels(self.bitmap)
    }

    unsafe fn unlock_pixels(&self) {
        ulBitmapUnlockPixels(self.bitmap)
    }
}
