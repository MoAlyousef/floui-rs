use std::any::Any;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::Arc;
use std::sync::Mutex;

use crate::enums::Color;

lazy_static::lazy_static! {
    static ref WIDGET_MAP: Mutex<HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>> = Mutex::new(HashMap::default());
}

/// Print to console
pub fn log(s: &str) {
    let s = std::ffi::CString::new(s).unwrap();
    unsafe {
        floui_sys::Cfloui_log(s.as_ptr());
    }
}

/// Obtain a specific widget from an id
/// ```rust,no_run
/// use floui::widgets::Text;
/// use floui::prelude::from_id;
/// let t: Text = from_id("mytext").unwrap();
/// ```
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

/// Traits implemented by all widgets
pub unsafe trait WidgetExt {
    /// Gets the inner wrapper
    fn inner(&self) -> *mut floui_sys::CWidget;
    /// Gets the underlying pointer (JNI jobject on android, UIView on iOS)
    fn underlying(&self) -> *mut c_void {
        unsafe { floui_sys::CWidget_inner(&mut *self.inner() as _) }
    }
    /// Constructs a new widget from a widget pointer
    fn from_widget_ptr(ptr: *mut floui_sys::CWidget) -> Self
    where
        Self: Sized;
    /// Adds the widget to the widget map, and assigns it an id
    fn id(self, id: &'static str) -> Self
    where
        Self: 'static + Sync + Sized + Clone + Send,
    {
        WIDGET_MAP
            .lock()
            .unwrap()
            .insert(id, Box::new(self.clone()));
        self
    }
    /// Sets the background color
    fn background(self, col: Color) -> Self
    where
        Self: Sized,
    {
        unsafe { floui_sys::CWidget_background(self.inner(), col.0) }
        self
    }
}

/// The view controller struct
#[derive(Clone)]
pub struct ViewController {
    pub(crate) inner: Arc<*mut floui_sys::CFlouiViewController>,
}

impl ViewController {
    /// On Android, arg1: JNIEnv, arg2: main view, arg3: ConstraintLayout
    pub unsafe fn new(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> Self {
        let inner = Arc::new(floui_sys::CFlouiViewController_new(arg1, arg2, arg3));
        Self { inner }
    }
    /// Required on android
    pub unsafe fn handle_events(view: *mut c_void) {
        floui_sys::CFlouiViewController_handle_events(view);
    }
}
