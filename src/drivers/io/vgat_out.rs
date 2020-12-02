use core::{
    convert::TryInto,
    default::Default,
    fmt::{self, Write},
};

/// The default starting location of the vga text mode framebuffer.
pub const DEFAULT_VGA_TEXT_BUFF_START: *mut &mut [VgatChar; DEFAULT_VGA_TEXT_BUFF_WIDTH] =
    0xb8000 as *mut &mut [VgatChar; DEFAULT_VGA_TEXT_BUFF_WIDTH];

/// The default dimensions of the VGA framebuffer.
pub const DEFAULT_VGA_TEXT_BUFF_WIDTH: usize = 25;
pub const DEFAULT_VGA_TEXT_BUFF_HEIGHT: usize = 80;

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
/// mode chars include two chars: char style (e.g., blink) & the actual utf char.
pub struct VgatDisplayStyle {
    blinking: bool,
    bg_color: Color,
    fg_color: Color,
}

impl Into<u8> for VgatDisplayStyle {
    fn into(self) -> u8 {
        (self.blinking as u8) << 7 | (self.bg_color as u8) << 4 | self.fg_color as u8
    }
}

/// A character displayed via the vga text mode driver with:
/// - a UTF-8 char being displayed
/// - a blinking status (0 | 1)
/// - a background color OR a foreground color
pub struct VgatChar {
    style: u8,
    value: u8,
}

impl VgatChar {
    pub fn new(value: u8, style: VgatDisplayStyle) -> Self {
        Self {
            value: value,
            style: style.into(),
        }
    }
}

/// An output that implements a byte-sink writer for the vga text mode out.
pub struct VgatOut<'a, const W: usize, const H: usize> {
    // The resolution of the screen in terms of chars displayable
    char_buffer: &'a mut [&'a mut [VgatChar; W]; H],

    // The index of the next position in which a char can be inserted
    head_pos: (usize, usize),
}

/// Obtain a safe, bounds-checked slice of framebuffer slots from an unsafe,
/// raw pointer provided by the developer (namely, vgat_buff_start).
/// Dimensions are indicated by the associated W and H constants, also
/// (optionall) provided by the developer (see provided Default implementation).
impl<'a, const W: usize, const H: usize> VgatOut<'a, W, H> {
    pub fn new(vgat_buff_start: *mut &'a mut [VgatChar; W]) -> Self {
        Self {
            char_buffer: unsafe {
                core::slice::from_raw_parts_mut::<&'a mut [VgatChar; W]>(vgat_buff_start, W * H)
                    .try_into()
                    .expect("failed to obtain a vga framebuffer with the specified dimensions")
            },
            head_pos: (0, 0),
        }
    }

    /// Writes a VGA char to the screen.
    pub fn write_char(&mut self, c: VgatChar) {
        self.char_buffer[self.head_pos.1][self.head_pos.0] = c.into();

        self.head_pos.0 = (self.head_pos.0 + 1) % W;
        self.head_pos.1 = (self.head_pos.1 + 1) % H;
    }
}

/// If the user doesn't provide a specific framebuffer, use the default one,
/// which has a static context.
impl Default for VgatOut<'static, DEFAULT_VGA_TEXT_BUFF_WIDTH, DEFAULT_VGA_TEXT_BUFF_HEIGHT> {
    fn default() -> Self {
        Self::new(DEFAULT_VGA_TEXT_BUFF_START)
    }
}

impl<const W: usize, const H: usize> Write for VgatOut<'_, W, H> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // If the sum of chars following an ANSI escape code matches one of
        // the below, render it correctly
        let mut ansi_sum: u16 = 0;
        // Current display style set by the working ANSI esc code
        let mut curr_ansi_style: u8 = VgatDisplayStyle {
            blinking: false,
            bg_color: Color::Black,
            fg_color: Color::White,
        }
        .into();
        let in_ansi_esc = false;

        //const ANSI_ESCAPE_CODES: [&'static char] = [r#"\u001b"#, r#"\033"#, r#"\x1b"#];
        const ACCEPTED_ANSI_CHECKSUMS: [u16; 3] = [
            // \u001b ASCII sum
            92 + 117 + 48 + 48 + 49 + 98,
            // \033 ASCII sum
            92 + 48 + 51 + 51,
            // \x1b ASCII sum
            92 + 120 + 49 + 98,
        ];
        const MIN_ACCEPTABLE_ANSI_ESC_CHECKSUM: u16 = ACCEPTED_ANSI_CHECKSUMS[1];
        const MAX_ACCEPTABLE_ANSI_ESC_CHECKSUM: u16 = ACCEPTED_ANSI_CHECKSUMS[0];

        for c in s.chars() {
            // Each \ indicates, at least, that a new ANSI escape code is
            // beginning - regardless of if the \ is extraneous or not
            if c == '\\' {
                ansi_sum = 0;
                in_ansi_esc = true;
            }

            ansi_sum += c as u16;

            if ansi_sum > MAX_ACCEPTABLE_ANSI_ESC_CHECKSUM {
                ansi_sum = 0;
            }

            // Current ANSI escape code checksum must be at least that of the
            // shortest ANSI escape pref. (i.e., \033)
            if ansi_sum >= MIN_ACCEPTABLE_ANSI_ESC_CHECKSUM {
                for accepted_checksum in ACCEPTED_ANSI_CHECKSUMS.iter() {
                    if ansi_sum == *accepted_checksum {
                    }
                }
            }

            if !in_ansi_esc {
                // Put a char on the screen with the current ANSI style
                self.write_char(VgatChar {
                    style: curr_ansi_style,
                    value: c as u8,
                });
            }
        }

        Ok(())
    }
}
