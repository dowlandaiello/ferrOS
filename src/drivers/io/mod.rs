pub mod vgat_out;

use core::fmt::{Write, self};
use spin::Mutex;
use vgat_out::{VgatOut, DEFAULT_VGA_TEXT_BUFF_HEIGHT as VGAT_H, DEFAULT_VGA_TEXT_BUFF_WIDTH as VGAT_W, DEFAULT_VGA_TEXT_BUFF_START as VGAT_S};

macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        print($crate::format_args_nl!($($arg)*));
    })
}

macro_rules! print {
    ($($arg:tt)*) => ()
}

pub fn _print(args: fmt::Arguments<'_>) {
    if let Some(w) = &mut *STDOUT.lock() {
        w.write_fmt(args);
    }
}
