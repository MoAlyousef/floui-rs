use crate::{enums, prelude::*};
use floui_sys;
use std::path::Path;
use std::sync::Arc;

/// A Spacer widget
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
    /// Constructs a new widget
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CSpacer_new()) };
        Self { inner }
    }
}

/// A ScrollView widget
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
    /// Constructs a new widget, Takes only a single widget
    pub fn new(widget: &impl WidgetExt) -> Self {
        let inner = unsafe { Arc::new(floui_sys::CScrollView_new(widget.inner())) };
        Self { inner }
    }
}

/// A Slider widget
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
    /// Constructs a new widget
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CSlider_new()) };
        Self { inner }
    }

    /// Sets the action on changing the slider
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

    /// Sets the foreground color
    pub fn foreground(self, col: enums::Color) -> Self {
        unsafe { floui_sys::CSlider_foreground(*self.inner, col.0) }
        self
    }

    /// Sets the value of the slider
    pub fn set_value(&self, val: f64) {
        unsafe { floui_sys::CSlider_set_value(*self.inner, val) }
    }

    /// Constructs a slider with a value val
    pub fn with_value(self, val: f64) -> Self {
        self.set_value(val);
        self
    }

    /// Gets the slider's value
    pub fn value(&self) -> f64 {
        unsafe { floui_sys::CSlider_value(*self.inner) }
    }
}

/// A ImageView widget
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
    /// Constructs a new widget
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CImageView_new()) };
        Self { inner }
    }

    /// Constructs an ImageView from an available image. Images should be in your platform's assets folder:
    /// On android: images need to be added to res/drawable.
    /// On iOS: images need to be added to Assets.xcassets
    pub fn load<P: AsRef<Path>>(path: &P) -> Self {
        let path = std::ffi::CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let inner = unsafe { Arc::new(floui_sys::CImageView_load(path.as_ptr())) };
        Self { inner }
    }

    /// Sets the image for the ImageView
    pub fn image<P: AsRef<Path>>(self, path: &P) -> Self {
        let path = std::ffi::CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            floui_sys::CImageView_image(*self.inner, path.as_ptr());
        }
        self
    }
}

/// A WebView widget
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
    /// Constructs a new widget
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CWebView_new()) };
        Self { inner }
    }

    /// Load a url.
    /// - on iOS: 
    ///     - Requires adding WebKit.framework under General > Frameworks, Libraries and Embedded Content.
    ///     - Requires enabling the `ios-webview` flag in your Cargo.toml.
    ///     - Local files can be loaded using WebView::load_url() but need to be preceded by `file:///`, the files should be added to your xcode project.
    /// - On Android:
    ///     - To load local files, precede them with `file:///` and the path of the file, which should be added to an assets folder (File > New > Folder > Assets folder). This then can be loaded using WebView::load_url().
    ///     - To load http requests, you need to enable the internet permission in your AndroidManifest.xml: `<uses-permission android:name="android.permission.INTERNET" />`
    pub fn load_url<P: AsRef<Path>>(self, path: &P) -> Self {
        let path = std::ffi::CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe { floui_sys::CWebView_load_url(*self.inner, path.as_ptr()) }
        self
    }

    /// Load an html string
    pub fn load_html(self, html: &str) -> Self {
        let html = std::ffi::CString::new(html).unwrap();
        unsafe { floui_sys::CWebView_load_html(*self.inner, html.as_ptr()) }
        self
    }
}
