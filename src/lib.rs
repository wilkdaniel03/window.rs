use x11::xlib;

static mut DPY: *mut xlib::Display = std::ptr::null_mut();
static mut SCREEN: i32 = 0;

fn open_display() {
    unsafe {
        DPY = xlib::XOpenDisplay(std::ptr::null());
        SCREEN = xlib::XDefaultScreen(DPY);
    }
}

pub fn create_window() {
    open_display();
    unsafe {
        println!("{:?},{}",DPY,SCREEN);
    }
}

pub fn say_hi() {
    println!("Hello, world!");
}
