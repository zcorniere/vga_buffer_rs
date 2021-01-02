use crate::BUFFER_WIDTH;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Structure to manipulate the cursor of the vga buffer
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    /// Create new cursor at the given coords
    ///
    /// - x : horizontal coordinate
    /// - y : vertical coordinate
    /// - disable : remove the blinking cursor
    pub fn new(x: usize, y: usize, disable: bool) -> Self {
        let mut ret = Self { x, y };
        unsafe {
            if disable {
                ret.disable();
            }
            ret.update();
        }
        ret
    }

    /// Update cursor position
    pub unsafe fn update(&self) {
        let pos: u16 = self.y as u16 * BUFFER_WIDTH as u16 + self.x as u16;
        llvm_asm!("outb %al,%dx"
            :
            :"{dx}"(0x3D4),"{al}"(0x0F)
            :
        );
        llvm_asm!("outb %al,%dx"
            :
            :"{dx}"(0x3D5),"{al}"(pos & 0xFF)
            :
        );
        llvm_asm!("outb %al,%dx"
            :
            :"{dx}"(0x3D4),"{al}"(0x0E)
            :
        );
        llvm_asm!("outb %al,%dx"
            :
            :"{dx}"(0x3D5),"{al}"( (pos>>8) & 0xFF)
            :
        );
    }

    /// Disable the blinking cursor
    pub unsafe fn disable(&mut self) {
        llvm_asm!("outb %al, %dx"
            :
            :"{dx}"(0x3d4), "{al}"(0x0a)
            :
        );
        llvm_asm!("outb %al, %dx"
            :
            :"{dx}"(0x3d5), "{al}"(0x20)
            :
        );
    }
}
