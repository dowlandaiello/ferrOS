use crate::drivers::io::vgat_out::{
    VgatOut, DEFAULT_VGA_TEXT_BUFF_HEIGHT, DEFAULT_VGA_TEXT_BUFF_WIDTH,
};
use spin::Mutex;

// Default I/O module definitions.

/// The default stdout handle acquired through the VGA text buffer.
pub static mut STDOUT: Mutex<
    VgatOut<'static, DEFAULT_VGA_TEXT_BUFF_WIDTH, DEFAULT_VGA_TEXT_BUFF_HEIGHT>,
> = Mutex::new(VgatOut::default());
