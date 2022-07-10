use std::collections::HashMap;
use std::any::Any;
use std::sync::Mutex;
use std::sync::Arc;
use std::os::raw::c_void;

lazy_static::lazy_static! {
    static ref WIDGET_MAP: Mutex<HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>> = Mutex::new(HashMap::default());
}

pub fn log(s: &str) {
    let s = std::ffi::CString::new(s).unwrap();
    unsafe {
        floui_sys::Cfloui_log(s.as_ptr());
    }
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
    pub(crate) inner: Arc<*mut floui_sys::CFlouiViewController>,
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