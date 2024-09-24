use libc;
use x11::xlib;

extern "C" {
    fn XOpenDisplay(_1: *const libc::c_char) -> *mut xlib::Display;
    fn XDefaultScreen(_1: *mut xlib::Display) -> libc::c_int;
    fn XCloseDisplay(_1: *mut xlib::Display) -> libc::c_int;
    fn XRootWindow(_1: *mut xlib::Display, _2: i32) -> xlib::Window;
    fn XCreateSimpleWindow(
        _1: *mut xlib::Display,
        _2: xlib::Window,
        _3: i32,
        _4: i32,
        _5: u32,
        _6: u32,
        _7: u32,
        _8: u64,
        _9: u64,
    ) -> xlib::Window;
    fn XBlackPixel(_1: *mut xlib::Display, _2: i32) -> u64;
    fn XWhitePixel(_1: *mut xlib::Display, _2: i32) -> u64;
    fn XDestroyWindow(_1: *mut xlib::Display, _2: xlib::Window) -> libc::c_int;
    fn XMapWindow(_1: *mut xlib::Display, _2: xlib::Window) -> libc::c_int;
    fn XNextEvent(_1: *mut xlib::Display, _2: *mut xlib::XEvent) -> libc::c_int;
    fn XUnmapWindow(_1: *mut xlib::Display, _2: xlib::Window) -> libc::c_int;
}

pub fn open_display() -> *mut xlib::Display {
    unsafe {
        let dpy: *mut xlib::Display;
        // Handle string as display specification
        dpy = XOpenDisplay(std::ptr::null());
        if dpy == std::ptr::null_mut() {
            eprintln!("Failed to open display");
        }
        dpy
    }
}

pub fn get_default_screen(dpy: *mut xlib::Display) -> i32 {
    unsafe {
        let screen: i32;
        screen = XDefaultScreen(dpy);
        screen
    }
}

pub fn create_window(dpy: *mut xlib::Display, screen: i32, width: u32, height: u32) -> (xlib::Window,xlib::Window) {
    unsafe {
        let event: *mut xlib::XEvent  = std::ptr::null_mut();
        let root_window = XRootWindow(dpy,screen);
        let window = XCreateSimpleWindow(dpy,root_window,0,0,width,height,1,XBlackPixel(dpy,screen),XWhitePixel(dpy,screen));
        // handle possible error
        XMapWindow(dpy,window);
        while XNextEvent(dpy,event) == 0 {}
        (root_window,window)
    }
}

pub fn close_all(dpy: *mut xlib::Display, win: xlib::Window) {
    unsafe {
        // Handle possible errors
        XUnmapWindow(dpy,win);
        XDestroyWindow(dpy,win);
        XCloseDisplay(dpy);
    }
}
