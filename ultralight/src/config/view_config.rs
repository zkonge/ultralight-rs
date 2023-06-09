use std::{
    fmt::{self, Debug, Formatter},
    sync::Mutex,
};

use ultralight_sys::*;

use crate::{string::UString, AsULRawPtr};

macro_rules! lock_in_scope {
    ($mutex:expr) => {
        let _guard = $mutex.lock().unwrap();
    };
}

pub struct ViewConfig(ULViewConfig, Mutex<()>);

impl ViewConfig {
    /// Whether to render using the GPU renderer (accelerated) or the CPU renderer (un-accelerated).
    ///
    /// This option is only valid if you're managing the Renderer yourself (eg, you've previously
    /// called ulCreateRenderer() instead of ulCreateApp()).
    ///
    /// When true, the View will be rendered to an offscreen GPU texture using the GPU driver set in
    /// ulPlatformSetGPUDriver(). You can fetch details for the texture via ulViewGetRenderTarget().
    ///
    /// When false (the default), the View will be rendered to an offscreen pixel buffer using the
    /// multithreaded CPU renderer. This pixel buffer can optionally be provided by the user--
    ///
    /// for more info see ulViewGetSurface().
    pub fn set_is_accelerated(&mut self, is_accelerated: bool) {
        lock_in_scope!(self.1);
        unsafe { ulViewConfigSetIsAccelerated(self.0, is_accelerated) }
    }

    /// Set whether images should be enabled
    ///
    /// (Default = True)
    pub fn set_is_transparent(&mut self, is_transparent: bool) {
        lock_in_scope!(self.1);
        unsafe { ulViewConfigSetIsTransparent(self.0, is_transparent) }
    }

    /// The initial device scale, ie. the amount to scale page units to screen pixels. This should be
    /// set to the scaling factor of the device that the View is displayed on.
    ///
    /// @note 1.0 is equal to 100% zoom (no scaling), 2.0 is equal to 200% zoom (2x scaling)
    ///
    /// (Default = 1.0)
    pub fn set_initial_device_scale(&mut self, initial_device_scale: f64) {
        lock_in_scope!(self.1);
        unsafe { ulViewConfigSetInitialDeviceScale(self.0, initial_device_scale) }
    }

    /// Whether or not the View should initially have input focus.
    ///
    /// (Default = [`true`])
    pub fn set_initial_focus(&mut self, is_focused: bool) {
        lock_in_scope!(self.1);
        unsafe { ulViewConfigSetInitialFocus(self.0, is_focused) }
    }

    /// Set whether images should be enabled.
    ///
    /// (Default = True)
    pub fn set_enable_images(&mut self, enabled: bool) {
        lock_in_scope!(self.1);
        unsafe { ulViewConfigSetEnableImages(self.0, enabled) }
    }

    /// Set whether JavaScript should be enabled.
    ///
    /// (Default = True)
    pub fn set_enable_javascript(&mut self, enabled: bool) {
        lock_in_scope!(self.1);
        unsafe { ulViewConfigSetEnableJavaScript(self.0, enabled) }
    }

    /// Set default font-family to use.
    ///
    /// (Default = Times New Roman)
    pub fn set_font_family_standard(&mut self, font_name: &str) {
        lock_in_scope!(self.1);
        let s = UString::from(font_name);
        unsafe { ulViewConfigSetFontFamilyStandard(self.0, s.as_raw_ptr()) }
    }

    /// Set default font-family to use for fixed fonts, eg <pre> and <code>.
    ///
    /// (Default = Courier New)
    pub fn set_font_family_fixed(&mut self, font_name: &str) {
        lock_in_scope!(self.1);
        let s = UString::from(font_name);
        unsafe { ulViewConfigSetFontFamilyFixed(self.0, s.as_raw_ptr()) }
    }

    /// Set default font-family to use for serif fonts.
    ///
    /// (Default = Times New Roman)
    pub fn set_font_family_serif(&mut self, font_name: &str) {
        lock_in_scope!(self.1);
        let s = UString::from(font_name);
        unsafe { ulViewConfigSetFontFamilySerif(self.0, s.as_raw_ptr()) }
    }

    /// Set default font-family to use for sans-serif fonts.
    ///
    /// (Default = Arial)
    pub fn set_font_family_sans_serif(&mut self, font_name: &str) {
        lock_in_scope!(self.1);
        let s = UString::from(font_name);
        unsafe { ulViewConfigSetFontFamilySansSerif(self.0, s.as_raw_ptr()) }
    }

    /// Set user agent string.
    ///
    /// (See <Ultralight/platform/Config.h> for the default)
    pub fn set_user_agent(&mut self, agent_string: &str) {
        lock_in_scope!(self.1);
        let s = UString::from(agent_string);
        unsafe { ulViewConfigSetUserAgent(self.0, s.as_raw_ptr()) }
    }
}

impl AsULRawPtr<ULViewConfig> for ViewConfig {
    fn as_raw_ptr(&self) -> ULViewConfig {
        self.0
    }
}

impl Drop for ViewConfig {
    fn drop(&mut self) {
        unsafe { ulDestroyViewConfig(self.0) }
    }
}

impl Debug for ViewConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ViewConfig").finish_non_exhaustive()
    }
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self(unsafe { ulCreateViewConfig() }, Mutex::new(()))
    }
}

unsafe impl Send for ViewConfig {}
unsafe impl Sync for ViewConfig {}
