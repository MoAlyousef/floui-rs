use floui_rs::*;

#[no_mangle]
extern "C" fn floui_main(vc: *mut UIViewController) {
    let fvc = ViewController::new(vc);
    let _main_view = MainView::new(&fvc, &[
        &Button::new("Increment"),
        &Text::new("0"),
        &Button::new("Decrement")
    ]);
}

fn main() {}