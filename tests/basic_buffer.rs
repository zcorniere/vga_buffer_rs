use vga_buffer_rs::buffer::Buffer;
use vga_buffer_rs::BasicBufferManipulation;
use vga_buffer_rs::BUFFER_HEIGHT;

#[test]
fn many_print() {
    let mut buf = Buffer::new();
    for _ in 0..200 {
        buf.write_string("test output line");
    }
}

#[test]
fn many_print_but_write() {
    use std::fmt::Write;
    let mut buf = Buffer::new();
    for _ in 0..200 {
        writeln!(buf, "test output line").unwrap();
    }
}
#[test]
fn test_println_output() {
    let mut buf = Buffer::new();
    let s = "Some test string that fits on a single line";
    buf.write_string(format!("\n{}\n", s).as_str());
    for (i, c) in s.chars().enumerate() {
        let screen_char = buf.buffer.chars[BUFFER_HEIGHT - 2][i];
        assert_eq!(char::from(screen_char.ascii_char), c);
    }
}

#[test]
fn test_println_output_but_write() {
    use std::fmt::Write;
    let mut buf = Buffer::new();
    let s = "Some test string that fits on a single line";
    writeln!(buf, "\n{}", s).unwrap();
    for (i, c) in s.chars().enumerate() {
        let screen_char = buf.buffer.chars[BUFFER_HEIGHT - 2][i];
        assert_eq!(char::from(screen_char.ascii_char), c);
    }
}
