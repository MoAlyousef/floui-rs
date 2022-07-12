use floui::{enums::*, prelude::*, widgets::*};
use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;

#[no_mangle]
extern "C" fn floui_handle_events(arg1: *mut c_void) {
    ViewController::handle_events(arg1);
}

#[no_mangle]
extern "C" fn floui_main(arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void {
    let fvc = ViewController::new(arg1, arg2, arg3);
    let count = Rc::from(RefCell::from(0));
    MainView::new(
        &fvc,
        &[
            &Button::new("Increment")
                .foreground(Color::Blue)
                .action({
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
    .underlying() as _
}

fn main() {}
