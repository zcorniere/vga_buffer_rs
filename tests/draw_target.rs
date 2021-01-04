use vga_buffer_rs::{
    buffer::Buffer,
    draw::{BoxShape, SHAPE_SIDE, SHAPE_UP},
    DrawTarget, BUFFER_HEIGHT, BUFFER_SIZE, BUFFER_WIDTH,
};

#[test]
fn test_draw_not_transparent() {
    let bo2 = BoxShape::new((0, 0), (20, 20), false);
    let bo = BoxShape::new((0, 0), BUFFER_SIZE, false);
    let mut b = Buffer::new();
    b.draw(&bo2);
    b.draw(&bo);

    for i in &b.buffer.chars {
        for j in i {
            print!("{}", j.ascii_char as char);
        }
        println!();
    }

    for i in 1..b.buffer.chars.len() - 1 {
        assert_eq!(b.buffer.chars[i][0].ascii_char, SHAPE_SIDE);
        for x in 1..BUFFER_WIDTH - 1 {
            assert_eq!(b.buffer.chars[i][x].ascii_char, b' ');
        }
        assert_eq!(b.buffer.chars[i][BUFFER_WIDTH - 1].ascii_char, SHAPE_SIDE);
    }
    assert_eq!(b.buffer.chars[0][0].ascii_char, SHAPE_SIDE);
    assert_eq!(b.buffer.chars[0][BUFFER_WIDTH - 1].ascii_char, SHAPE_SIDE);
    for i in 1..b.buffer.chars[0].len() - 1 {
        assert_eq!(b.buffer.chars[0][i].ascii_char, SHAPE_UP);
    }
    assert_eq!(b.buffer.chars[BUFFER_HEIGHT - 1][0].ascii_char, SHAPE_SIDE);
    assert_eq!(
        b.buffer.chars[BUFFER_HEIGHT - 1][BUFFER_WIDTH - 1].ascii_char,
        SHAPE_SIDE
    );
    for i in 1..b.buffer.chars[BUFFER_HEIGHT - 1].len() - 1 {
        assert_eq!(b.buffer.chars[0][i].ascii_char, SHAPE_UP);
    }
}

#[test]
fn test_draw_transparent() {
    let bo2 = BoxShape::new((0, 0), (20, 20), false);
    let bo = BoxShape::new((0, 0), BUFFER_SIZE, true);
    let mut b = Buffer::new();
    b.draw(&bo2);
    b.draw(&bo);

    for i in &b.buffer.chars {
        for j in i {
            print!("{}", j.ascii_char as char);
        }
        println!();
    }
    for i in 1..19 {
        assert_eq!(b.buffer.chars[i][19].ascii_char, SHAPE_SIDE);
    }
    for i in 1..b.buffer.chars.len() - 1 {
        assert_eq!(b.buffer.chars[i][0].ascii_char, SHAPE_SIDE);
        assert_eq!(b.buffer.chars[i][BUFFER_WIDTH - 1].ascii_char, SHAPE_SIDE);
    }
    assert_eq!(b.buffer.chars[0][0].ascii_char, SHAPE_SIDE);
    assert_eq!(b.buffer.chars[0][BUFFER_WIDTH - 1].ascii_char, SHAPE_SIDE);
    for i in 1..b.buffer.chars[0].len() - 1 {
        assert_eq!(b.buffer.chars[0][i].ascii_char, SHAPE_UP);
    }
    assert_eq!(b.buffer.chars[BUFFER_HEIGHT - 1][0].ascii_char, SHAPE_SIDE);
    assert_eq!(
        b.buffer.chars[BUFFER_HEIGHT - 1][BUFFER_WIDTH - 1].ascii_char,
        SHAPE_SIDE
    );
    for i in 1..b.buffer.chars[BUFFER_HEIGHT - 1].len() - 1 {
        assert_eq!(b.buffer.chars[0][i].ascii_char, SHAPE_UP);
    }
}
