use crate::enums::Color;
use crate::prelude::*;
use floui_sys;
use std::sync::Arc;

/// A label
#[derive(Clone)]
pub struct Text {
    inner: Arc<*mut floui_sys::CText>,
}

unsafe impl Sync for Text {}
unsafe impl Send for Text {}

unsafe impl WidgetExt for Text {
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
    /// Creates a new label
    pub fn new(label: &str) -> Self {
        let label = std::ffi::CString::new(label).unwrap();
        let inner = unsafe { Arc::new(floui_sys::CText_new(label.as_ptr())) };
        Self { inner }
    }

    /// Sets the text
    pub fn text(self, label: &str) -> Text {
        let label = std::ffi::CString::new(label).unwrap();
        unsafe { floui_sys::CText_text(*self.inner, label.as_ptr()) };
        self
    }

    /// Centers the text
    pub fn center(self) -> Text {
        unsafe { floui_sys::CText_center(*self.inner) };
        self
    }

    /// Sets the text to be bold
    pub fn bold(self) -> Text {
        unsafe { floui_sys::CText_bold(*self.inner) };
        self
    }

    /// Sets the font size
    pub fn fontsize(self, size: i32) -> Text {
        unsafe { floui_sys::CText_fontsize(*self.inner, size) };
        self
    }

    /// Sets the foreground color
    pub fn foreground(self, col: Color) -> Text {
        unsafe { floui_sys::CText_foreground(*self.inner, col.0) }
        self
    }
}

/// A text field
#[derive(Clone)]
pub struct TextField {
    inner: Arc<*mut floui_sys::CTextField>,
}

unsafe impl Sync for TextField {}
unsafe impl Send for TextField {}

unsafe impl WidgetExt for TextField {
    fn inner(&self) -> *mut floui_sys::CWidget {
        *self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: Arc::new(ptr as _),
        }
    }
}

impl TextField {
    /// Creates a new TextField
    pub fn new() -> Self {
        let inner = unsafe { Arc::new(floui_sys::CTextField_new()) };
        Self { inner }
    }

    /// Sets the text
    pub fn text(self, label: &str) -> TextField {
        let label = std::ffi::CString::new(label).unwrap();
        unsafe { floui_sys::CTextField_text(*self.inner, label.as_ptr()) };
        self
    }

    /// Centers the text
    pub fn center(self) -> TextField {
        unsafe { floui_sys::CTextField_center(*self.inner) };
        self
    }

    /// Sets the font size
    pub fn fontsize(self, size: i32) -> TextField {
        unsafe { floui_sys::CTextField_fontsize(*self.inner, size) };
        self
    }

    /// Sets the foreground color
    pub fn foreground(self, col: Color) -> TextField {
        unsafe { floui_sys::CTextField_foreground(*self.inner, col.0) }
        self
    }
}
