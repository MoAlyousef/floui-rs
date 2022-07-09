use floui_sys;
// use std::sync::Arc;

pub trait WidgetExt {
    fn inner(&self) -> *mut floui_sys::CWidget;
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self where Self: Sized;
}

#[derive(Clone)]
pub struct ViewController {
    inner: *mut floui_sys::CFlouiViewController,
}

pub enum UIViewController {}

impl ViewController {
    pub fn new(uvc: *mut UIViewController) -> Self {
        let inner = unsafe {
            floui_sys::CFlouiViewController_new(uvc as _, std::ptr::null_mut(), std::ptr::null_mut())
        };
        Self {
            inner
        }
    }
}

#[derive(Clone)]
pub struct MainView {
    inner: *mut floui_sys::CMainView,
}

impl WidgetExt for MainView {
    fn inner(&self) -> *mut floui_sys::CWidget {
        self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: ptr as _,
        }
    }
}

impl MainView {
    pub fn new(vc: &ViewController, widgets: &[&dyn WidgetExt]) -> Self {
        let inner = unsafe { floui_sys::CMainView_new(vc.inner) };
        for w in widgets {
            unsafe {
                floui_sys::CMainView_add(inner, w.inner());
            }
        }
        Self {
            inner
        }
    }
}

#[derive(Clone)]
pub struct Button {
    inner: *mut floui_sys::CButton,
}

impl WidgetExt for Button {
    fn inner(&self) -> *mut floui_sys::CWidget {
        self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: ptr as _,
        }
    }
}

impl Button {
    pub fn new(label: &str) -> Self {
        let label = std::ffi::CString::new(label).unwrap();
        let inner = unsafe { floui_sys::CButton_new(label.as_ptr()) };
        Self {
            inner
        }
    }

    pub fn action<F: 'static + FnMut(&Self)>(&self, cb: F) {
        unsafe {
            unsafe extern "C" fn shim(wid: *mut floui_sys::CWidget, data: *mut std::os::raw::c_void) {
                let wid = Button::from_widget_ptr(wid as *mut _);
                let a = data as *mut Box<dyn FnMut(&Button)>;
                let f: &mut (dyn FnMut(&Button)) = &mut **a;
                let _ =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&wid)));
            }
            let a: *mut Box<dyn FnMut(&Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let callback: floui_sys::CFlouiCallback = Some(shim);
            floui_sys::CButton_action(self.inner, callback, data);
        }
    }
}

#[derive(Clone)]
pub struct Text {
    inner: *mut floui_sys::CText,
}

impl WidgetExt for Text {
    fn inner(&self) -> *mut floui_sys::CWidget {
        self.inner as _
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self {
        Self {
            inner: ptr as _,
        }
    }
}

impl Text {
    pub fn new(label: &str) -> Self {
        let label = std::ffi::CString::new(label).unwrap();
        let inner = unsafe { floui_sys::CText_new(label.as_ptr()) };
        Self {
            inner
        }
    }

    pub fn text(&self, label: &str) -> &Text {
        let label = std::ffi::CString::new(label).unwrap();
        unsafe { floui_sys::CText_text(self.inner, label.as_ptr()) };
        self
    }
}

pub fn log(s: &str) {
    let s = std::ffi::CString::new(s).unwrap();
    unsafe { floui_sys::Cfloui_log(s.as_ptr()); }
}