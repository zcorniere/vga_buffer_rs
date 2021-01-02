#![no_std]
#![feature(llvm_asm)]

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub mod buffer;
pub mod cursor;
pub mod vga_buffer;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorPair(u8);

impl ColorPair {
    fn new(fg: Color, bg: Color) -> Self {
        ColorPair((bg as u8) << 4 | (fg as u8))
    }
}

impl Default for ColorPair {
    fn default() -> Self {
        Self::new(Color::Green, Color::Black)
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Rust representation of a vga char.
///
/// 1 byte is for the char itself
/// 4 bits are for the foreground color
/// 4 bits are for the background color
pub struct ScreenChar {
    pub ascii_char: u8,
    pub color_code: ColorPair,
}

/// Implement very basic buffer manipulation
/// - print string
/// - new line
pub trait BasicBufferManipulation {
    fn write_byte(&mut self, byte: u8);
    fn write_string(&mut self, s: &str);
    fn new_line(&mut self);
    fn clear_row(&mut self, row: usize);
    fn clear(&mut self);
}
