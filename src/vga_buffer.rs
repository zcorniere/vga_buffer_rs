use crate::cursor::Cursor;
use crate::BasicBufferManipulation;
use crate::ColorPair;
use crate::{ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};
use volatile::Volatile;

pub const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[repr(transparent)]
#[derive(Debug)]
struct Vga {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaBuffer {
    pub cursor: Cursor,
    buffer: &'static mut Vga,
}

impl VgaBuffer {
    pub fn new(cursor_on: bool) -> Self {
        Self {
            cursor: Cursor::new(0, BUFFER_HEIGHT - 1, !cursor_on),
            ..Default::default()
        }
    }
}

impl Default for VgaBuffer {
    fn default() -> Self {
        Self {
            cursor: Cursor::default(),
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Vga) },
        }
    }
}

impl BasicBufferManipulation for VgaBuffer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.cursor.x >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[BUFFER_HEIGHT - 1][self.cursor.x].write(ScreenChar {
                    ascii_char: byte,
                    color_code: ColorPair::default(),
                });
                self.cursor.x += 1;
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
        unsafe {
            self.cursor.update();
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.cursor.x = 0;
        unsafe {
            self.cursor.update();
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: ColorPair::default(),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
        unsafe {
            self.cursor.update();
        }
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.cursor.x = 0;
        unsafe {
            self.cursor.update();
        }
    }
}

impl core::fmt::Write for VgaBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
