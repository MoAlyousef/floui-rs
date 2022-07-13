use floui::{enums::*, prelude::*, widgets::*};
use std::cell::RefCell;
use std::rc::Rc;

fn mygui(vc: &ViewController) -> MainView {
    let count = Rc::from(RefCell::from(0));
    MainView::new(
        &vc,
        &[
            &Button::new("Increment").foreground(Color::Blue).action({
                let count = count.clone();
                move |_| {
                    log("Increment clicked");
                    let mut c = count.borrow_mut();
                    *c += 1;
                    let t: Text = from_id("mytext").unwrap();
                    t.text(&format!("{}", c));
                }
            }),
            &Text::new("0").id("mytext").center().bold(),
            &Button::new("Decrement")
                .foreground(Color::Red)
                .action(move |_| {
                    log("Increment clicked");
                    let mut c = count.borrow_mut();
                    *c -= 1;
                    let t: Text = from_id("mytext").unwrap();
                    t.text(&format!("{}", c));
                }),
        ],
    )
}

use std::os::raw::c_void;

#[no_mangle]
extern "C" fn floui_handle_events(arg1: *mut c_void) {
    ViewController::handle_events(arg1);
}

#[no_mangle]
extern "C" fn floui_main(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void {
    let vc = ViewController::new(arg1, arg2, arg3);
    mygui(&vc).underlying() as _
}

fn main() {}
