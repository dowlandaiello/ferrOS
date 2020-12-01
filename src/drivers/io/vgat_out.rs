use core::default::Default;
use std::io::Write;

/// The default starting location of the vga text mode framebuffer.
pub const DEFAULT_VGA_TEXT_BUFF_START: *const char = 0xb8000;

/// The default dimensions of the VGA framebuffer.
pub const DEFAULT_VGA_TEXT_BUFF_DIMS: (u8, u8) = (25, 80);

/// A VGA text framebuffer color (see below wikipedia link for explanation).
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yello = 0xe,
    White = 0xf,
}

/// See [VGA text mode](https://en.wikipedia.org/wiki/VGA_text_mode): vga text
/// mode chars include two chars: char style (e.g., blink) & the actual utf char
#[repr(C)]
/// A background or foreground coloring style.
enum VgatColoringStyle {
    WithBgColor(Color),
    WithFgColor(Color),
}

impl Into<char> for VgatColoringStyle {
    fn into(self) -> char {
        // See above wikipedia link, wherein the format for a vga text mode
        // char is laid out: BLINK_BIT 3_BG_COLOR_BITS 4_FG_COLOR_BITS
        match self {
            Self::WithBgColor(c) => c.into() << 4,
            Self::WithFgColor(c) => c.into(),
        }
    }
}

/// How a character should be displayed on the screen.
struct VgatDisplayStyle {
    blinking: bool,
    coloring: VgatColoringStyle,
}

impl Into<char> for VgatDisplayStyle {
    fn into(self) -> char {
        self.blinking << 7 | self.coloring.into()
    }
}

/// A character displayed via the vga text mode driver with:
/// - a UTF-8 char being displayed
/// - a blinking status (0 | 1)
/// - a background color OR a foreground color
pub struct VgatChar {
    style: char,
    value: char,
}

impl VgatChar {
    pub fn new(value: char, style: VgatDisplayStyle) -> Self {
        Self {
            value: value,
            style: style.into(),
        }
    }
}

/// An output that implements a byte-sink writer for the vga text mode out.
pub struct VgatOut<const W: usize, const H: usize> {
    // The resolution of the screen in terms of chars displayable
    char_buffer: [[VgatChar; W]; H],
}

impl Default for VgatOut {
    fn default() -> Self {
        Self {
            char_buffer: unsafe {
                std::slice::from_raw_parts_mut(
                    DEFAULT_VGA_TEXT_BUFF_START,
                    DEFAULT_VGA_TEXT_BUFF_DIMS.0 * DEFAULT_VGA_TEXT_BUFF_DIMS.1,
                )
            },
        }
    }
}
