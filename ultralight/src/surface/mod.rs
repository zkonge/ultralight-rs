mod bitmap_surface;
mod generic_surface;

use std::{ffi::c_void, slice};

pub use bitmap_surface::BitmapSurface;
pub use generic_surface::GenericSurface;

pub trait Surface: Sized {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn row_bytes(&self) -> u32;
    fn size(&self) -> usize;
    fn pixels(&mut self) -> PixelGuard<Self>;

    /// # Safety
    ///
    /// The returned pointer must be valid for the lifetime of the surface.
    ///
    /// User should keep the pointer be unique.
    unsafe fn lock_pixels(&self) -> *mut c_void;

    /// # Safety
    ///
    /// The pointer passed to this function must be the same as the one returned by `lock_pixels`.
    unsafe fn unlock_pixels(&self);
}

pub struct PixelGuard<'a, S: Surface> {
    surface: &'a mut S,
    pixels: &'a [u8],
}

impl<'a, S: Surface> PixelGuard<'a, S> {
    pub fn pixels(&self) -> &[u8] {
        self.pixels
    }
}

impl<'a, S: Surface> PixelGuard<'a, S> {
    pub(crate) fn new(surface: &'a mut S) -> Self {
        let pixels =
            unsafe { slice::from_raw_parts_mut(surface.lock_pixels().cast(), surface.size()) };
        Self { surface, pixels }
    }
}

impl<'a, S: Surface> Drop for PixelGuard<'a, S> {
    fn drop(&mut self) {
        unsafe {
            self.surface.unlock_pixels();
        }
    }
}
