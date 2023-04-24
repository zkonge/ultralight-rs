use std::{
    fmt::{self, Debug, Formatter},
    time::Duration,
};

use ultralight_sys::*;

use crate::{string::UString, AsULRawPtr};

pub struct Config(ULConfig);

// SAFETY: ultralight can only run on systems, that c_int = i32.
#[repr(i32)]
pub enum FaceWinding {
    Clockwise = ULFaceWinding_kFaceWinding_Clockwise as i32,
    CounterClockwise = ULFaceWinding_kFaceWinding_CounterClockwise as i32,
}

// SAFETY: ultralight can only run on systems, that c_int = i32.
#[repr(i32)]
pub enum FontHinting {
    /// Lighter hinting algorithm
    ///
    /// glyphs are slightly fuzzier but better
    /// resemble their original shape. This is achieved by snapping glyphs to the
    /// pixel grid only vertically which better preserves inter-glyph spacing.
    Smooth = ULFontHinting_kFontHinting_Smooth as i32,

    /// Default hinting algorithm
    ///
    /// offers a good balance between sharpness and
    /// shape at smaller font sizes.
    Normal = ULFontHinting_kFontHinting_Normal as i32,

    /// Strongest hinting algorithm
    ///
    /// outputs only black/white glyphs. The result
    /// is usually unpleasant if the underlying TTF does not contain hints for
    /// this type of rendering.
    Monochrome = ULFontHinting_kFontHinting_Monochrome as i32,
}

impl Config {
    /// Set the file path to a writable directory that will be used to store cookies,
    /// cached resources, and other persistent data.
    ///
    /// Files are only written to disk when using a persistent Session.
    pub fn set_cache_path(&mut self, cache_path: &str) -> &mut Self {
        let s = UString::from(cache_path);
        unsafe { ulConfigSetCachePath(self.0, s.as_raw_ptr()) }
        self
    }

    /// The library loads bundled resources (eg, cacert.pem and other localized resources) from the
    /// FileSystem API (eg, `file:///resources/cacert.pem`).
    /// You can customize the prefix to use when loading resource URLs by modifying this setting.
    ///
    /// (Default = "resources/")
    pub fn set_resource_path_prefix(&mut self, resource_path_prefix: &str) -> &mut Self {
        let s = UString::from(resource_path_prefix);
        unsafe { ulConfigSetResourcePathPrefix(self.0, s.as_raw_ptr()) }
        self
    }

    /// The winding order for front-facing triangles.
    ///
    /// Note: This is only used with custom GPUDrivers.
    ///
    /// (Default = kFaceWinding_CounterClockwise)
    pub fn set_face_winding(&mut self, winding: FaceWinding) -> &mut Self {
        unsafe { ulConfigSetFaceWinding(self.0, winding as _) }
        self
    }

    /// The hinting algorithm to use when rendering fonts. See [`FontHinting`].
    ///
    /// (Default = [`FontHinting::Normal`])
    pub fn set_font_hinting(&mut self, font_hinting: FontHinting) -> &mut Self {
        unsafe { ulConfigSetFontHinting(self.0, font_hinting as _) }
        self
    }

    /// The gamma to use when compositing font glyphs, change this value to adjust contrast
    /// (Adobe and Apple prefer 1.8, others may prefer 2.2).
    ///
    /// (Default = 1.8)
    pub fn set_font_gamma(&mut self, font_gamma: f64) -> &mut Self {
        unsafe { ulConfigSetFontGamma(self.0, font_gamma) }
        self
    }

    /// Set user stylesheet (CSS)
    ///
    /// (Default = Empty)
    pub fn set_user_stylesheet(&mut self, css_string: &str) -> &mut Self {
        let s = UString::from(css_string);
        unsafe { ulConfigSetUserStylesheet(self.0, s.as_raw_ptr()) }
        self
    }

    /// Set whether or not we should continuously repaint any Views or compositor layers,
    /// regardless if they are dirty or not.
    /// This is mainly used to diagnose painting/shader issues.
    ///
    /// (Default = [`false`])
    pub fn set_force_repaint(&mut self, enabled: bool) -> &mut Self {
        unsafe { ulConfigSetForceRepaint(self.0, enabled) }
        self
    }

    /// Set the amount of time to wait before triggering another repaint when a CSS animation
    /// is active.
    ///
    /// (Default = 1.0 / 60.0)
    pub fn set_animation_timer_delay(&mut self, delay: Duration) -> &mut Self {
        unsafe { ulConfigSetAnimationTimerDelay(self.0, delay.as_secs_f64()) }
        self
    }

    /// When a smooth scroll animation is active, the amount of time (in seconds) to wait before
    /// triggering another repaint.
    ///
    /// (Default is 60 Hz)
    pub fn set_scroll_timer_delay(&mut self, delay: Duration) -> &mut Self {
        unsafe { ulConfigSetScrollTimerDelay(self.0, delay.as_secs_f64()) }
        self
    }

    /// The amount of time (in seconds) to wait before running the recycler (will attempt to return
    /// excess memory back to the system).
    ///
    /// (Default = 4.0)
    pub fn set_recycle_delay(&mut self, delay: Duration) -> &mut Self {
        unsafe { ulConfigSetRecycleDelay(self.0, delay.as_secs_f64()) }
        self
    }

    /// Set the size of WebCore's memory cache for decoded images, scripts, and other assets in bytes.
    ///
    /// (Default = 64 * 1024 * 1024)
    pub fn set_memory_cache_size(&mut self, size: u32) -> &mut Self {
        unsafe { ulConfigSetMemoryCacheSize(self.0, size) }
        self
    }

    /// Set the number of pages to keep in the cache.
    ///
    /// (Default = 0)
    pub fn set_page_cache_size(&mut self, size: u32) -> &mut Self {
        unsafe { ulConfigSetPageCacheSize(self.0, size) }
        self
    }

    /// JavaScriptCore tries to detect the system's physical RAM size to set reasonable allocation
    /// limits. Set this to anything other than 0 to override the detected value. Size is in bytes.
    /// This can be used to force JavaScriptCore to be more conservative with its allocation strategy
    /// (at the cost of some performance)
    ///
    /// (Default = 0)
    pub fn set_override_ram_size(&mut self, size: u32) -> &mut Self {
        unsafe { ulConfigSetOverrideRAMSize(self.0, size) }
        self
    }

    /// The minimum size of large VM heaps in JavaScriptCore. Set this to a lower value to make these
    /// heaps start with a smaller initial value.
    ///
    /// (Default = 32 * 1024 * 1024)
    pub fn set_min_large_heap_size(&mut self, size: u32) -> &mut Self {
        unsafe { ulConfigSetMinLargeHeapSize(self.0, size) }
        self
    }

    /// The minimum size of small VM heaps in JavaScriptCore. Set this to a lower value to make these
    /// heaps start with a smaller initial value.
    ///
    /// (Default = 1 * 1024 * 1024)
    pub fn set_min_small_heap_size(&mut self, size: u32) -> &mut Self {
        unsafe { ulConfigSetMinSmallHeapSize(self.0, size) }
        self
    }

    /// The number of threads to use in the Renderer (for parallel painting on the CPU, etc.).
    ///
    /// You can set this to a certain number to limit the number of threads to spawn.
    ///
    /// @note If this value is 0 (the default), the number of threads will be determined at runtime
    /// using the following formula:
    ///        max(PhysicalProcessorCount() - 1, 1)
    pub fn set_num_renderer_threads(&mut self, num_renderer_threads: u32) -> &mut Self {
        unsafe { ulConfigSetNumRendererThreads(self.0, num_renderer_threads) }
        self
    }

    /// The max amount of time (in seconds) to allow Renderer::Update to run per call. The library will
    /// attempt to throttle timers and/or reschedule work if this time budget is exceeded.
    ///
    /// (Default = 0.005)
    pub fn set_max_update_time(&mut self, max_update_time: Duration) -> &mut Self {
        unsafe { ulConfigSetMaxUpdateTime(self.0, max_update_time.as_secs_f64()) }
        self
    }

    /// The alignment (in bytes) of the BitmapSurface when using the CPU renderer.
    ///
    /// The underlying bitmap associated with each BitmapSurface will have row_bytes padded to reach
    /// this alignment.
    ///
    /// Aligning the bitmap helps improve performance when using the CPU renderer. Determining the
    /// proper value to use depends on the CPU architecture and max SIMD instruction set used.
    ///
    /// We generally target the 128-bit SSE2 instruction set across most PC platforms so '16' is a safe
    /// value to use.
    ///
    /// You can set this to '0' to perform no padding (row_bytes will always be width * 4) at a slight
    /// cost to performance.
    ///
    /// (Default = 16)
    pub fn set_cpu_bitmap_surface_alignment(&mut self, alignment: f64) -> &mut Self {
        unsafe { ulConfigSetBitmapAlignment(self.0, alignment) }
        self
    }
}

impl AsULRawPtr<ULConfig> for Config {
    fn as_raw_ptr(&self) -> ULConfig {
        self.0
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { ulDestroyConfig(self.0) }
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config").finish_non_exhaustive()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self(unsafe { ulCreateConfig() })
    }
}
