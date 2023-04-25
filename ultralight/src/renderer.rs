use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

use ultralight_sys::*;

use crate::{config::Config, session::Session, AsULRawPtr};

static LOADED: AtomicBool = AtomicBool::new(false);

pub struct Renderer(ULRenderer);

impl Renderer {
    pub fn new(config: &Config) -> Rc<Self> {
        match LOADED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(x) | Err(x) => {
                if !x {
                    panic!("Ultralight has already been loaded. You can only load it once.")
                }
            }
        }

        Rc::new(Renderer(unsafe { ulCreateRenderer(config.as_raw_ptr()) }))
    }

    pub fn create_session(self: Rc<Self>, is_persistent: bool, name: &str) -> Rc<Session> {
        Session::new(self, is_persistent, name)
    }

    /// Get the default [`Session`] for the given [`Renderer`].
    /// The default session should not be destroyed. So we add some magic in [`Session::drop`].
    pub fn default_session(self: Rc<Self>) -> Rc<Session> {
        let renderer_ptr = self.as_raw_ptr();
        let session = unsafe { Session::from_raw(self, ulDefaultSession(renderer_ptr)) };

        Rc::new(session)
    }

    /// [`Renderer`] can't create [`View`]. Create it with [`Session`].
    pub fn create_view() -> ! {
        unimplemented!("`Renderer` can't create `View`. Create it with `Session`.")
    }

    pub fn update(&self) {
        unsafe { ulUpdate(self.0) }
    }

    pub fn render(&self) {
        unsafe { ulRender(self.0) }
    }

    pub fn purge_memory(&self) {
        unsafe { ulPurgeMemory(self.0) }
    }

    pub fn log_memory_usage(&self) {
        unsafe { ulLogMemoryUsage(self.0) }
    }
}

impl AsULRawPtr<ULRenderer> for Renderer {
    fn as_raw_ptr(&self) -> ULRenderer {
        self.0
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { ulDestroyRenderer(self.0) }
    }
}
