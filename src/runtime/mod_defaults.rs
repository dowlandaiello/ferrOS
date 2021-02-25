use crate::drivers::io::vgat_out::{
    VgatOut, DEFAULT_VGA_TEXT_BUFF_HEIGHT, DEFAULT_VGA_TEXT_BUFF_WIDTH,
};

// Default I/O module definitions.
lazy_static! {
    /// The default stdout handle acquired through the VGA text buffer.
    pub static ref STDOUT: VgatOut<'static, DEFAULT_VGA_TEXT_BUFF_WIDTH, DEFAULT_VGA_TEXT_BUFF_HEIGHT> = VgatOut::default();
}
