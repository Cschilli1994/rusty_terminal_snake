use std::{time::Duration, thread};




pub fn hide_cursor() {
    print!("\x1b[?25l")
}
pub fn show_cursor() {
    print!("\x1b[?25h")
}


pub fn sleep(seconds: u64) {
    thread::sleep(Duration::from_millis(seconds));
}