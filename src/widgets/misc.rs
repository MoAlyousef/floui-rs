use crate::{enums, prelude::*};
use floui_sys;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct Spacer {
    inner: Arc<*mut floui_sys::CSpacer>,
}

unsafe impl Sync for Spacer {}
unsafe impl Send for Spacer {}

impl WidgetExt for Spacer {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl Spacer {
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CSpacer_new()) };
        Self { inner }
    }
}

#[derive(Clone)]
pub struct ScrollView {
    inner: Arc<*mut floui_sys::CScrollView>,
}

unsafe impl Sync for ScrollView {}
unsafe impl Send for ScrollView {}

impl WidgetExt for ScrollView {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl ScrollView {
    pub fn new(widget: &impl WidgetExt) -> Self {
        let inner = unsafe { Arc::new(floui_sys::CScrollView_new(widget.inner())) };
        Self { inner }
    }
}

#[derive(Clone)]
pub struct Slider {
    inner: Arc<*mut floui_sys::CSlider>,
}

unsafe impl Sync for Slider {}
unsafe impl Send for Slider {}

impl WidgetExt for Slider {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl Slider {
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CSlider_new()) };
        Self { inner }
    }

    pub fn action<F: 'static + FnMut(&Self)>(self, cb: F) -> Self {
        unsafe {
            unsafe extern "C" fn shim(
                wid: *mut floui_sys::CWidget,
                data: *mut std::os::raw::c_void,
            ) {
                let wid = Slider::from_widget_ptr(wid as *mut _);
                let a = data as *mut Box<dyn FnMut(&Slider)>;
                let f: &mut (dyn FnMut(&Slider)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&wid)));
            }
            let a: *mut Box<dyn FnMut(&Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let callback: floui_sys::CFlouiCallback = Some(shim);
            floui_sys::CSlider_action(*self.inner, callback, data);
        }
        self
    }

    pub fn foreground(self, col: enums::Color) -> Self {
        unsafe { floui_sys::CSlider_foreground(*self.inner, col.0) }
        self
    }

    pub fn set_value(&self, val: f64) {
        unsafe { floui_sys::CSlider_set_value(*self.inner, val) }
    }

    pub fn with_value(self, val: f64) -> Self {
        self.set_value(val);
        self
    }

    pub fn value(&self) -> f64 {
        unsafe { floui_sys::CSlider_value(*self.inner) }
    }
}

#[derive(Clone)]
pub struct ImageView {
    inner: Arc<*mut floui_sys::CImageView>,
}

unsafe impl Sync for ImageView {}
unsafe impl Send for ImageView {}

impl WidgetExt for ImageView {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl ImageView {
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CImageView_new()) };
        Self { inner }
    }

    pub fn load<P: AsRef<Path>>(path: &P) -> Self {
        let path = std::ffi::CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let inner = unsafe { Arc::new(floui_sys::CImageView_load(path.as_ptr())) };
        Self { inner }
    }

    pub fn image<P: AsRef<Path>>(self, path: &P) -> Self {
        let path = std::ffi::CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            floui_sys::CImageView_image(*self.inner, path.as_ptr());
        }
        self
    }
}

#[derive(Clone)]
pub struct WebView {
    inner: Arc<*mut floui_sys::CWebView>,
}

unsafe impl Sync for WebView {}
unsafe impl Send for WebView {}

impl WidgetExt for WebView {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl WebView {
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CWebView_new()) };
        Self { inner }
    }

    pub fn load_url<P: AsRef<Path>>(self, path: &P) -> Self {
        let path = std::ffi::CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe { floui_sys::CWebView_load_url(*self.inner, path.as_ptr()) }
        self
    }

    pub fn load_html(self, html: &str) -> Self {
        let html = std::ffi::CString::new(html).unwrap();
        unsafe { floui_sys::CWebView_load_html(*self.inner, html.as_ptr()) }
        self
    }
}
