use std::{cell::Cell, ffi, marker::PhantomData, mem::ManuallyDrop, ptr::null_mut, rc::Rc};

use ultralight_sys::*;

use crate::{
    config::ViewConfig, session::Session, string::UString, surface::GenericSurface, AsULRawPtr,
};

pub struct View<'a> {
    view: ULView,
    _session: Rc<Session>,
    callback_lifetime: PhantomData<Cell<&'a ()>>,
}

impl<'a> View<'a> {
    pub fn url(&self) -> String {
        let url = unsafe { UString::from_raw(ulViewGetURL(self.view)) };
        url.to_owned()
    }

    pub fn title(&self) -> String {
        let title = unsafe { UString::from_raw(ulViewGetTitle(self.view)) };
        title.to_owned()
    }

    pub fn width(&self) -> u32 {
        unsafe { ulViewGetWidth(self.view) }
    }

    pub fn height(&self) -> u32 {
        unsafe { ulViewGetHeight(self.view) }
    }

    pub fn device_scale(&self) -> f64 {
        unsafe { ulViewGetDeviceScale(self.view) }
    }

    pub fn set_device_scale(&mut self, scale: f64) {
        unsafe {
            ulViewSetDeviceScale(self.view, scale);
        }
    }

    pub fn is_accelerated(&self) -> bool {
        unsafe { ulViewIsAccelerated(self.view) }
    }

    pub fn is_transparent(&self) -> bool {
        unsafe { ulViewIsTransparent(self.view) }
    }

    pub fn is_loading(&self) -> bool {
        unsafe { ulViewIsLoading(self.view) }
    }

    pub fn surface(&mut self) -> GenericSurface {
        let surface = unsafe { ulViewGetSurface(self.view) };
        unsafe { GenericSurface::from_raw(surface) }
    }

    pub fn load_html(&mut self, html_string: &str) {
        let html = UString::from(html_string);
        unsafe {
            ulViewLoadHTML(self.view, html.as_raw_ptr());
        }
    }

    pub fn load_url(&mut self, url: &str) {
        let url = UString::from(url);
        unsafe {
            ulViewLoadURL(self.view, url.as_raw_ptr());
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            ulViewResize(self.view, width, height);
        }
    }

    /// NOTICE: [`Ok`] doesn't mean no exception, return with no [`Err`] and "throw '';" is indistinguishable
    /// with simple [`ulViewEvaluateScript`].
    pub fn evaluate_script(&mut self, script: &str) -> Result<String, String> {
        let script = UString::from(script);
        let mut exception: ULString = null_mut();

        let result =
            unsafe { ulViewEvaluateScript(self.view, script.as_raw_ptr(), &mut exception) };

        let [result, exception] = [result, exception]
            .map(|x| unsafe { UString::from_raw(x) })
            .map(ManuallyDrop::new);

        if exception.is_empty() {
            Ok(result.to_string())
        } else {
            Err(exception.to_string())
        }
    }

    // TODO: impl all callbacks with macro
    pub fn set_begin_loading_callback<F>(&mut self, callback: &'a F)
    where
        F: Fn(ULView, u64, bool, &str),
    {
        unsafe extern "C" fn wrapper<F>(
            user_data: *mut ffi::c_void,
            caller: ULView,
            frame_id: ffi::c_ulonglong,
            is_main_frame: bool,
            url: ULString,
        ) where
            F: Fn(ULView, u64, bool, &str),
        {
            let cb = unsafe { &*(user_data as *const F) };
            let url = ManuallyDrop::new(UString::from_raw(url));
            cb(caller, frame_id, is_main_frame, &url);
        }

        unsafe {
            ulViewSetBeginLoadingCallback(
                self.view,
                Some(wrapper::<F>),
                callback as *const _ as *mut _,
            );
        }
    }

    pub fn set_finish_loading_callback<F>(&mut self, callback: &'a F)
    where
        F: Fn(ULView, u64, bool, &str),
    {
        unsafe extern "C" fn wrapper<F>(
            user_data: *mut ffi::c_void,
            caller: ULView,
            frame_id: ffi::c_ulonglong,
            is_main_frame: bool,
            url: ULString,
        ) where
            F: Fn(ULView, u64, bool, &str),
        {
            let cb = unsafe { &*(user_data as *const F) };
            let url = ManuallyDrop::new(UString::from_raw(url));
            cb(caller, frame_id, is_main_frame, &url);
        }

        unsafe {
            ulViewSetFinishLoadingCallback(
                self.view,
                Some(wrapper::<F>),
                callback as *const _ as *mut _,
            );
        }
    }

    pub fn set_fail_loading_callback<F>(&mut self, callback: &'a F)
    where
        F: Fn(ULView, u64, bool, &str, &str, &str, i32),
    {
        unsafe extern "C" fn wrapper<F>(
            user_data: *mut ffi::c_void,
            caller: ULView,
            frame_id: ffi::c_ulonglong,
            is_main_frame: bool,
            url: ULString,
            description: ULString,
            error_domain: ULString,
            error_code: ffi::c_int,
        ) where
            F: Fn(ULView, u64, bool, &str, &str, &str, i32),
        {
            let cb = unsafe { &*(user_data as *const F) };
            let url = ManuallyDrop::new(UString::from_raw(url));
            let description = ManuallyDrop::new(UString::from_raw(description));
            let error_domain = ManuallyDrop::new(UString::from_raw(error_domain));
            cb(
                caller,
                frame_id,
                is_main_frame,
                &url,
                &description,
                &error_domain,
                error_code,
            );
        }

        unsafe {
            ulViewSetFailLoadingCallback(
                self.view,
                Some(wrapper::<F>),
                callback as *const _ as *mut _,
            );
        }
    }
}

impl View<'_> {
    pub(crate) fn new(
        session: Rc<Session>,
        width: u32,
        height: u32,
        view_config: &ViewConfig,
    ) -> Self {
        let view = unsafe {
            ulCreateView(
                session.renderer().as_raw_ptr(),
                width,
                height,
                view_config.as_raw_ptr(),
                session.as_raw_ptr(),
            )
        };
        Self {
            view,
            _session: session,
            callback_lifetime: PhantomData,
        }
    }
}

impl AsULRawPtr<ULView> for View<'_> {
    fn as_raw_ptr(&self) -> ULView {
        self.view
    }
}

impl Drop for View<'_> {
    fn drop(&mut self) {
        unsafe {
            ulDestroyView(self.view);
        }
    }
}
