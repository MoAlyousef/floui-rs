use floui::*;
use std::os::raw::c_void;

#[no_mangle]
extern "C" fn floui_handle_events(arg1: *mut c_void) {
    ViewController::handle_events(arg1);
}

#[no_mangle]
extern "C" fn floui_main(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void {
    let fvc = ViewController::new(arg1, arg2, arg3);
    MainView::new(&fvc, &[
        &Button::new("Increment").action(|_| { 
            log("Increment clicked");
            let t: Text = from_id("mytext").unwrap();
            t.text("works");
        }),
        &Text::new("0").id("mytext"),
        &Button::new("Decrement").action(|_| log("Increment clicked"))
    ]).inner() as _
}

fn main() {}