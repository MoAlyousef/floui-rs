use floui_sys;
use std::os::raw::c_void;
use std::sync::Arc;
use std::collections::HashMap;
use std::any::Any;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref WIDGET_MAP: Mutex<HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>> = Mutex::new(HashMap::default());
}

pub fn from_id<T: 'static + WidgetExt + Clone>(id: &str) -> Option<T> {
    if let Some(w) = WIDGET_MAP.lock().unwrap().get(&id) {
        if let Some(t) = w.downcast_ref::<T>() {
            Some(t.clone())
        } else {
            None
        }
    } else {
        None
    }
}

pub trait WidgetExt {
    fn inner(&self) -> *mut floui_sys::CWidget;
    fn underlying(&self) -> *mut c_void {
        unsafe { floui_sys::CWidget_inner(&mut *self.inner() as _) }
    }
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self
    where
        Self: Sized;
    fn id(self, id: &'static str) -> Self where Self: 'static + Sync + Sized + Clone + Send {
        WIDGET_MAP
            .lock()
            .unwrap()
            .insert(id, Box::new(self.clone()));
        self
    }
}

#[derive(Clone)]
pub struct ViewController {
    inner: Arc<*mut floui_sys::CFlouiViewController>,
}

impl ViewController {
    pub fn new(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> Self {
        let inner = unsafe {
            Arc::new(floui_sys::CFlouiViewController_new(
                arg1,
                arg2,
                arg3,
            ))
        };
        Self { inner }
    }
    pub fn handle_events(view: *mut c_void) {
        unsafe {
            floui_sys::CFlouiViewController_handle_events(view);
        }
    }
}

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
}

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
}

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

pub fn log(s: &str) {
    let s = std::ffi::CString::new(s).unwrap();
    unsafe {
        floui_sys::Cfloui_log(s.as_ptr());
    }
}
