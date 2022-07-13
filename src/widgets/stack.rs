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

#[derive(Clone)]
pub struct VStack {
    inner: Arc<*mut floui_sys::CVStack>,
}

unsafe impl Sync for VStack {}
unsafe impl Send for VStack {}

impl WidgetExt for VStack {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl VStack {
    pub fn new(widgets: &[&dyn WidgetExt]) -> Self {
        let inner = unsafe { Arc::new(floui_sys::CVStack_new()) };
        for w in widgets {
            unsafe {
                floui_sys::CVStack_add(*inner, w.inner());
            }
        }
        Self { inner }
    }

    pub fn add(&self, widget: &impl WidgetExt) {
        unsafe {
            floui_sys::CVStack_add(*self.inner, widget.inner());
        }
    }

    pub fn remove(&self, widget: &impl WidgetExt) {
        unsafe {
            floui_sys::CVStack_remove(*self.inner, widget.inner());
        }
    }

    pub fn clear(&self) {
        unsafe {
            floui_sys::CVStack_clear(*self.inner);
        }
    }
}

#[derive(Clone)]
pub struct HStack {
    inner: Arc<*mut floui_sys::CHStack>,
}

unsafe impl Sync for HStack {}
unsafe impl Send for HStack {}

impl WidgetExt for HStack {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl HStack {
    pub fn new(widgets: &[&dyn WidgetExt]) -> Self {
        let inner = unsafe { Arc::new(floui_sys::CHStack_new()) };
        for w in widgets {
            unsafe {
                floui_sys::CHStack_add(*inner, w.inner());
            }
        }
        Self { inner }
    }

    pub fn add(&self, widget: &impl WidgetExt) {
        unsafe {
            floui_sys::CHStack_add(*self.inner, widget.inner());
        }
    }

    pub fn remove(&self, widget: &impl WidgetExt) {
        unsafe {
            floui_sys::CHStack_remove(*self.inner, widget.inner());
        }
    }

    pub fn clear(&self) {
        unsafe {
            floui_sys::CHStack_clear(*self.inner);
        }
    }
}
