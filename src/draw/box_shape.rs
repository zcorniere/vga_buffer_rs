use crate::{
    draw::{SHAPE_SIDE, SHAPE_UP},
    Draw,
};
use alloc::vec::Vec;

pub struct BoxShape {
    pos: (usize, usize),
    size: (usize, usize),
    trans: bool,
    shape: Vec<Vec<u8>>,
}

impl BoxShape {
    pub fn new(pos: (usize, usize), size: (usize, usize), trans: bool) -> Self {
        let mut shape = BoxShape {
            pos,
            size,
            trans,
            shape: Vec::new(),
        };
        shape.shape.resize(size.0 - pos.0, Vec::new());
        for i in &mut shape.shape {
            i.resize(size.1 - pos.1, b' ');
        }
        // draw upper line
        shape.shape[0].iter_mut().map(|i| *i = SHAPE_UP).count();
        // draw lower line
        shape.shape[size.0 - pos.0 - 1]
            .iter_mut()
            .map(|i| *i = SHAPE_UP)
            .count();
        // draw left and right line
        shape
            .shape
            .iter_mut()
            .map(|i| {
                i[0] = SHAPE_SIDE;
                i[size.1 - pos.1 - 1] = SHAPE_SIDE;
            })
            .count();
        shape
    }
}

impl Draw for BoxShape {
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }

    fn get_size(&self) -> (usize, usize) {
        self.size
    }

    fn is_transparent(&self) -> bool {
        self.trans
    }

    fn get_line(&self, l: usize) -> Option<&[u8]> {
        if l > self.size.0 {
            return None;
        }
        Some(&self.shape[l])
    }
}
