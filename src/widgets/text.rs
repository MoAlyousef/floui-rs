use floui_sys;
use std::sync::Arc;
use crate::prelude::*;


#[derive(Clone)]
pub struct Text {
    inner: Arc<*mut floui_sys::CText>,
}

unsafe impl Sync for Text {}
unsafe impl Send for Text {}

impl WidgetExt for Text {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl Text {
    pub fn new(label: &str) -> Self {
        let label = std::ffi::CString::new(label).unwrap();
        let inner = unsafe { Arc::new(floui_sys::CText_new(label.as_ptr())) };
        Self { inner }
    }

    pub fn text(&self, label: &str) -> &Text {
        let label = std::ffi::CString::new(label).unwrap();
        unsafe { floui_sys::CText_text(*self.inner, label.as_ptr()) };
        self
    }
}