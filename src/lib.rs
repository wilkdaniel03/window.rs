mod x;

pub fn create_window() {
    let dpy = x::open_display();
    let screen = x::get_default_screen(dpy);
    let (_root,win) = x::create_window(dpy,screen,300,300);
    x::close_all(dpy,win);
}

pub fn say_hi() {
    println!("Hello, world!");
}
