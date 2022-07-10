use floui_sys;
use std::sync::Arc;
use crate::{prelude::*, enums};


#[derive(Clone)]
pub struct Button {
    inner: Arc<*mut floui_sys::CButton>,
}

unsafe impl Sync for Button {}
unsafe impl Send for Button {}

impl WidgetExt for Button {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl Button {
    pub fn new(label: &str) -> Self {
        let label = std::ffi::CString::new(label).unwrap();
        let inner = unsafe { Arc::new(floui_sys::CButton_new(label.as_ptr())) };
        Self { inner }
    }

    pub fn action<F: 'static + FnMut(&Self)>(self, cb: F) -> Self {
        unsafe {
            unsafe extern "C" fn shim(
                wid: *mut floui_sys::CWidget,
                data: *mut std::os::raw::c_void,
            ) {
                let wid = Button::from_widget_ptr(wid as *mut _);
                let a = data as *mut Box<dyn FnMut(&Button)>;
                let f: &mut (dyn FnMut(&Button)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&wid)));
            }
            let a: *mut Box<dyn FnMut(&Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let callback: floui_sys::CFlouiCallback = Some(shim);
            floui_sys::CButton_action(*self.inner, callback, data);
        }
        self
    }

    pub fn foreground(self, col: enums::Color) -> Self {
        unsafe { floui_sys::CButton_foreground(*self.inner, col.0) }
        self
    }
}