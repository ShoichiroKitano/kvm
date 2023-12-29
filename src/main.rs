use x11_dl::xlib;
use x11_dl::xlib::Xlib;

fn main() {
    unsafe {
        let xlib_struct = Xlib::open().unwrap();
        let display = (xlib_struct.XOpenDisplay)(std::ptr::null());
        let root = (xlib_struct.XDefaultRootWindow)(display);
        (xlib_struct.XGrabPointer)(
            display,
            root,
            0, // false
            // なんかしらんけどsigned longで定義されてるからキャスト
            (xlib::ButtonPressMask | xlib::ButtonReleaseMask) as u32,
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
            0, // None
            0, // None
            xlib::CurrentTime,
            );
        (xlib_struct.XGrabKeyboard)(
            display,
            root,
            0, // false
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
            xlib::CurrentTime,
        );

        let mut event = xlib::XEvent {type_: 0};
        let mut loop_count = 0;
        while true {
            (xlib_struct.XNextEvent)(display, &mut event as *mut xlib::XEvent);

            match event.get_type() {
                xlib::ButtonPress => println!("Mouse button pressed"),
                xlib::ButtonRelease => println!("Mouse button released"),
                xlib::KeyPress => {
                    let mut buffer: [std::os::raw::c_char; 2] = Default::default();
                    (xlib_struct.XLookupString)(
                        &mut event.key as *mut xlib::XKeyEvent,
                        &mut buffer as *mut std::os::raw::c_char,
                        2,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        );
                    println!(
                        "key pressed symbol = {}, code = {}",
                        std::ffi::CStr::from_ptr(&buffer as *const std::os::raw::c_char).to_str().unwrap(),
                        event.key.keycode,
                        );
                },
                xlib::KeyRelease => {
                    println!("key released");
                },
                _ => panic!("not match"),
            }
            if loop_count == 10 { // || exit_code) {
                break;
            }
            loop_count += 1;
        }
    }
    println!("Hello, world!");
}
