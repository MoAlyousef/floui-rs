use crate::prelude::*;
use floui_sys;
use std::sync::Arc;

#[derive(Clone)]
pub struct MainView {
    inner: Arc<*mut floui_sys::CMainView>,
}

unsafe impl Sync for MainView {}
unsafe impl Send for MainView {}

impl WidgetExt for MainView {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl MainView {
    pub fn new(vc: &ViewController, widgets: &[&dyn WidgetExt]) -> Self {
        let inner = unsafe { Arc::new(floui_sys::CMainView_new(*vc.inner)) };
        for w in widgets {
            unsafe {
                floui_sys::CMainView_add(*inner, w.inner());
            }
        }
        Self { inner }
    }

    pub fn add(&self, widget: &impl WidgetExt) {
        unsafe {
            floui_sys::CMainView_add(*self.inner, widget.inner());
        }
    }

    pub fn remove(&self, widget: &impl WidgetExt) {
        unsafe {
            floui_sys::CMainView_remove(*self.inner, widget.inner());
        }
    }

    pub fn clear(&self) {
        unsafe {
            floui_sys::CMainView_clear(*self.inner);
        }
    }
}
