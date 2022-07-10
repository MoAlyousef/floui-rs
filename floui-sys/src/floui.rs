/* automatically generated by rust-bindgen 0.56.0 */

extern "C" {
    pub fn Cfloui_log(s: *const ::std::os::raw::c_char);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFlouiViewController {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CFlouiViewController_new(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> *mut CFlouiViewController;
}
extern "C" {
    pub fn CFlouiViewController_handle_events(arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CWidget {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CWidget_id(self_: *mut CWidget, id: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn CWidget_background(self_: *mut CWidget, col: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn CWidget_size(self_: *mut CWidget, w: ::std::os::raw::c_int, h: ::std::os::raw::c_int);
}
extern "C" {
    pub fn CWidget_inner(self_: *mut CWidget) -> *mut ::std::os::raw::c_void;
}
pub type CFlouiCallback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut CWidget, data: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMainView {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CMainView_id(self_: *mut CMainView, id: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn CMainView_background(self_: *mut CMainView, col: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn CMainView_size(
        self_: *mut CMainView,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn CMainView_inner(self_: *mut CMainView) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn CMainView_add(self_: *mut CMainView, w: *const CWidget);
}
extern "C" {
    pub fn CMainView_remove(self_: *mut CMainView, w: *const CWidget);
}
extern "C" {
    pub fn CMainView_clear(self_: *mut CMainView);
}
extern "C" {
    pub fn CMainView_spacing(self_: *mut CMainView, spacing: ::std::os::raw::c_int);
}
extern "C" {
    pub fn CMainView_new(fvc: *const CFlouiViewController) -> *mut CMainView;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CButton {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CButton_new(label: *const ::std::os::raw::c_char) -> *mut CButton;
}
extern "C" {
    pub fn CButton_action(
        self_: *mut CButton,
        cb: CFlouiCallback,
        data: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CText {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CText_new(label: *const ::std::os::raw::c_char) -> *mut CText;
}
extern "C" {
    pub fn CText_text(self_: *mut CText, t: *const ::std::os::raw::c_char);
}
