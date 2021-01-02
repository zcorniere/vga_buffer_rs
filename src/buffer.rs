use crate::BasicBufferManipulation;
use crate::ColorPair;
use crate::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// Raw buffer representing the vga buffer area
pub struct RawBuffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy)]
/// A buffer that represent a vga buffer.
pub struct Buffer {
    col_pos: usize,
    pub buffer: RawBuffer,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            col_pos: 0,
            buffer: RawBuffer {
                chars: [[ScreenChar::default(); BUFFER_WIDTH]; BUFFER_HEIGHT],
            },
        }
    }
}

impl BasicBufferManipulation for Buffer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[BUFFER_HEIGHT - 1][self.col_pos] = ScreenChar {
                    ascii_char: byte,
                    color_code: ColorPair::default(),
                };
                self.col_pos += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: ColorPair::default(),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.col_pos = 0;
    }
}

impl core::fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
