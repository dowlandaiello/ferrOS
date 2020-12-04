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
#[derive(Clone, Copy)]
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
    pub unsafe fn new(vgat_buff_start: *mut &'a mut [VgatChar; W]) -> Self {
        Self {
            char_buffer: core::slice::from_raw_parts_mut::<&'a mut [VgatChar; W]>(
                vgat_buff_start,
                W * H,
            )
            .try_into()
            .expect("failed to obtain a vga framebuffer with the specified dimensions"),
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
        unsafe { Self::new(DEFAULT_VGA_TEXT_BUFF_START) }
    }
}

impl<const W: usize, const H: usize> Write for VgatOut<'_, W, H> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Whenever an ANSI sequence begins, change the output format to
        const NEXT_ANSI_ESC_CHAR: [char; 4] = ['\\', 'x', '1', 'b'];
        // e.g., when \ is inputted, 1
        // when x is inputted, 2, etc...
        let mut ansi_esc_i = 0;
        // index of any parts of the input that were skipped because we thought
        // they were in the ansi escape sequence, but turned out not to be
        let mut skipped_for_ansi = (0, 0);
        // the current ANSI display style
        // the part of the ANSI display param we are parsing (i.e., what comes
        // after \x1b): [d;dd;ddm

        // TODO: Finish ANSI parsing
        // let mut ansi_param_i = 0;
        // Digits in an ANSI color without special constraints (e.g., the most
        // significant digit) should be >=0 but <= 7
        const TEST_COLOR_IN_BOUNDS: fn(char) -> bool =
            |c| c.to_digit(16).map(|i| i <= 7).unwrap_or_default();
        const NEXT_ANSI_PARAM_VER: [fn(char) -> bool; 10] = [
            // Params must start with [
            |c| c == '[',
            // First digit shows normal (0) or blink (5)
            |c| c == '0' || c == '5',
            |c| c == ';',
            |c| c == '0' || c == '3' || c == '9',
            TEST_COLOR_IN_BOUNDS,
            // Go to bg color or no bg provided, and we're done with ansi param
            |c| c == ';' || c == 'm',
            |c| c == '0' || c == '4' || c == '1',
            TEST_COLOR_IN_BOUNDS,
            TEST_COLOR_IN_BOUNDS,
            |c| c == 'm',
        ];
        // TODO: See above TODO
        let curr_display_style = VgatDisplayStyle {
            blinking: false,
            bg_color: Color::Black,
            fg_color: Color::White,
        };

        for (i, c) in s.chars().enumerate() {
            // We've hit the last necessary character in an ANSI escape
            // seq., and are entering the params of the sequence itself
            if ansi_esc_i == 4 {
                ansi_esc_i = 0;
                skipped_for_ansi = (0, 0);
            } else if c != NEXT_ANSI_ESC_CHAR[ansi_esc_i] && ansi_esc_i > 0 && ansi_esc_i < 4 {
                // No longer in an ANSI escape sequence - parts before current
                // char weren't printed since they might have been, but now we
                // can backprint
                skipped_for_ansi.1 = i;
                ansi_esc_i = 0;

                // Put each buffered char onto the screen
                s[skipped_for_ansi.0..skipped_for_ansi.1]
                    .chars()
                    .map(|c| VgatChar {
                        style: (&curr_display_style).into(),
                        value: c as u8,
                    })
                    .for_each(|c| self.write_char(c));
            }

            // Continuing an escape sequence or starting one
            if c == NEXT_ANSI_ESC_CHAR[ansi_esc_i] {
                // Still in the ANSI escape sequence - continue parsing it
                if ansi_esc_i == 0 {
                    skipped_for_ansi.0 = i;
                }

                ansi_esc_i += 1;
            } else {
                self.write_char(VgatChar {
                    style: (&curr_display_style).into(),
                    value: c as u8,
                });
            }
        }

        Ok(())
    }
}
