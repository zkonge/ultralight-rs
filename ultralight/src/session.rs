use std::rc::Rc;

use ultralight_sys::*;

use crate::{config::ViewConfig, renderer::Renderer, string::UString, view::View, AsULRawPtr};

pub struct Session {
    session: ULSession,
    renderer: Rc<Renderer>,
}

impl Session {
    pub fn is_persistent(&self) -> bool {
        unsafe { ulSessionIsPersistent(self.session) }
    }

    pub fn name(&self) -> String {
        unsafe { UString::from_raw(ulSessionGetName(self.session)) }.to_owned()
    }

    pub fn id(&self) -> u64 {
        unsafe { ulSessionGetId(self.session) }
    }

    pub fn disk_path(&self) -> String {
        unsafe { UString::from_raw(ulSessionGetDiskPath(self.session)) }.to_owned()
    }

    pub fn create_view(
        self: Rc<Session>,
        width: u32,
        height: u32,
        view_config: &ViewConfig,
    ) -> View {
        View::new(self, width, height, view_config)
    }
}

impl Session {
    pub(crate) fn new(renderer: Rc<Renderer>, is_persistent: bool, name: &str) -> Rc<Session> {
        let s = UString::from(name);
        let session =
            unsafe { ulCreateSession(renderer.as_raw_ptr(), is_persistent, s.as_raw_ptr()) };
        Rc::new(Session { session, renderer })
    }

    pub(crate) unsafe fn from_raw(renderer: Rc<Renderer>, session: ULSession) -> Session {
        Session { session, renderer }
    }

    pub(crate) fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }
}

impl AsULRawPtr<ULSession> for Session {
    fn as_raw_ptr(&self) -> ULSession {
        self.session
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let default_session = unsafe { ulDefaultSession(self.renderer.as_raw_ptr()) };

        // default session should not be destroyed.
        if self.session == default_session {
            return;
        }

        unsafe { ulDestroySession(self.session) }
    }
}
