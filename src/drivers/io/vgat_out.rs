use core::{
    convert::TryInto,
    default::Default,
    fmt::{self, Write},
    ops::{Add, Sub},
};
use num_derive::FromPrimitive;

/// The default starting location of the vga text mode framebuffer.
pub const DEFAULT_VGA_TEXT_BUFF_START: *mut VgatBuffer<
    DEFAULT_VGA_TEXT_BUFF_WIDTH,
    DEFAULT_VGA_TEXT_BUFF_HEIGHT,
> = 0xb8000 as *mut VgatBuffer<DEFAULT_VGA_TEXT_BUFF_WIDTH, DEFAULT_VGA_TEXT_BUFF_HEIGHT>;

/// The default dimensions of the VGA framebuffer.
pub const DEFAULT_VGA_TEXT_BUFF_WIDTH: usize = 80;
pub const DEFAULT_VGA_TEXT_BUFF_HEIGHT: usize = 25;

/// A VGA text framebuffer color (see below wikipedia link for explanation).
#[derive(Clone, Copy, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Color {
    Black = 0x0 as u8,
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
    Yellow = 0xe,
    White = 0xf,
}

impl Color {
    pub fn dim_variant(&self) -> Self {
        if *self >= Self::DarkGray {
            *self - 0x8 as u8
        } else {
            *self
        }
    }

    pub fn bold_variant(&self) -> Self {
        self.dim_variant() + 0x8
    }

    /// Converts the index of the ANSI color to the color primitive.
    pub fn from_ansi_code(ansi_code: char) -> Self {
        match ansi_code {
            '0' => Self::Black,
            '1' => Self::Red,
            '2' => Self::Green,
            '3' => Self::Brown,
            '4' => Self::Blue,
            '5' => Self::Magenta,
            '6' => Self::Cyan,
            '7' => Self::LightGray,
            _ => Self::White,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

impl Add<u8> for Color {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        num::FromPrimitive::from_u8(self as u8 + other).unwrap_or_default()
    }
}

impl Sub<u8> for Color {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        num::FromPrimitive::from_u8(self as u8 - other).unwrap_or_default()
    }
}

/// See [VGA text mode](https://en.wikipedia.org/wiki/VGA_text_mode): vga text
/// mode chars include two chars: char style (e.g., blink) & the actual utf char.
pub struct VgatDisplayStyle {
    blinking: bool,
    bg_color: Color,
    fg_color: Color,
}

impl Default for VgatDisplayStyle {
    fn default() -> Self {
        Self {
            blinking: false,
            bg_color: Color::Black,
            fg_color: Color::White,
        }
    }
}

impl Into<u8> for &VgatDisplayStyle {
    fn into(self) -> u8 {
        (self.blinking as u8) << 7 | (self.bg_color as u8) << 4 | self.fg_color as u8
    }
}

// Non-copying conversion
impl Into<u8> for VgatDisplayStyle {
    fn into(self) -> u8 {
        (self.blinking as u8) << 7 | (self.bg_color as u8) << 4 | self.fg_color as u8
    }
}

/// A character displayed via the vga text mode driver with:
/// - a UTF-8 char being displayed
/// - a blinking status (0 | 1)
/// - a background color OR a foreground color
#[repr(C)]
pub struct VgatChar {
    value: u8,
    style: u8,
}

impl VgatChar {
    pub fn new(value: u8, style: VgatDisplayStyle) -> Self {
        Self {
            value: value,
            style: style.into(),
        }
    }
}

impl From<char> for VgatChar {
    fn from(c: char) -> Self {
        Self::new(c as u8, VgatDisplayStyle::default())
    }
}

#[repr(transparent)]
pub struct VgatBuffer<const W: usize, const H: usize> {
    buff: [[VgatChar; W]; H],
}

/// An output that implements a byte-sink writer for the vga text mode out.
pub struct VgatOut<'a, const W: usize, const H: usize> {
    // The resolution of the screen in terms of chars displayable
    char_buffer: &'a mut VgatBuffer<W, H>,

    // The index of the next position in which a char can be inserted
    head_pos: (usize, usize),

    // ANSI contexts are preserved from line to line
    color_state: VgatDisplayStyle,
}

/// Obtain a safe, bounds-checked slice of framebuffer slots from an unsafe,
/// raw pointer provided by the developer (namely, vgat_buff_start).
/// Dimensions are indicated by the associated W and H constants, also
/// (optionally) provided by the developer (see provided Default implementation).
impl<'a, const W: usize, const H: usize> VgatOut<'a, W, H> {
    pub unsafe fn new(vgat_buff_start: *mut VgatBuffer<W, H>) -> Self {
        Self {
            char_buffer: &mut *vgat_buff_start,
            head_pos: (0, 0),
            color_state: VgatDisplayStyle::default(),
        }
    }

    /// Writes a VGA char to the screen.
    pub fn write_char(&mut self, c: VgatChar) {
        self.char_buffer.buff[self.head_pos.0][self.head_pos.1] = c.into();
        self.head_pos.1 += 1;

        if self.head_pos.1 >= W {
            self.advance_print_feed();
        }
    }

    /// Moves the vga writer to the next print line.
    fn advance_print_feed(&mut self) {
        self.head_pos.1 = 0;
        self.head_pos.0 = (self.head_pos.0 + 1) % H;
    }

    /// Adopts the context specified in the given ANSI string. Returns the number
    /// of characters that were found to be part of the ANSI context escape sequence.
    fn adopt_ansi(&mut self, s: &str) -> u8 {
        let mut i = 0;

        let mut bold = false;

        for (i, c) in s.chars().enumerate() {
            match (i, c) {
                (0, '\\') | (1, 'x') | (2, '1') | (3, 'b') | (4, '[') => (),
                (5, n) => {
                    match n {
                        '0' => self.color_state = VgatDisplayStyle::default(),
                        '1' => {
                            self.color_state.fg_color = self.color_state.fg_color.bold_variant();
                            bold = true;
                        }
                        '2' => {
                            self.color_state.fg_color = self.color_state.fg_color.dim_variant();
                            bold = false;
                        }
                        _ => (),
                    };
                }
                (6, ';') => (),
                (6, 'm') => break,
                (7, '0') => {
                    self.color_state.fg_color = VgatDisplayStyle::default().fg_color;
                }
                (7, '3') => (),
                (8, ';') => (),
                (8, n) => {
                    self.color_state.fg_color = Color::from_ansi_code(n);

                    if bold {
                        self.color_state.fg_color = self.color_state.fg_color.bold_variant();
                    }
                }
                (9, 'm') => break,
                (9, ';') => (),
                (10, '0') => {
                    self.color_state.bg_color = VgatDisplayStyle::default().bg_color;
                }
                (10, '4') => (),
                (11, n) => {
                    self.color_state.bg_color = Color::from_ansi_code(n);

                    if bold {
                        self.color_state.bg_color = self.color_state.bg_color.bold_variant();
                    }
                }
                _ => break,
            };
        }

        i
    }
}

impl VgatOut<'static, DEFAULT_VGA_TEXT_BUFF_WIDTH, DEFAULT_VGA_TEXT_BUFF_HEIGHT> {
    const fn default() -> Self {
        unsafe {
            Self {

            }
        }
    }
}

/// If the user doesn't provide a specific framebuffer, use the default one,
/// which has a static context.
impl Default for VgatOut<'static, DEFAULT_VGA_TEXT_BUFF_WIDTH, DEFAULT_VGA_TEXT_BUFF_HEIGHT> {
    fn default() -> Self {
        Self::default()
    }
}

impl<'a, const W: usize, const H: usize> Write for VgatOut<'a, W, H> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut skipped_ansi_chars = 0;

        for (i, c) in s.chars().enumerate() {
            if skipped_ansi_chars > 0 {
                skipped_ansi_chars -= 1;
            }

            if skipped_ansi_chars == 0 {
                if c == '\\' {
                    skipped_ansi_chars = self.adopt_ansi(&s[i..]);
                } else if c == '\n' {
                    self.advance_print_feed();
                } else {
                    self.write_char(VgatChar {
                        value: c as u8,
                        style: (&self.color_state).into(),
                    });
                }
            }
        }

        Ok(())
    }
}
