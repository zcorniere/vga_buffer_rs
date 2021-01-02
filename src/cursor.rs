use crate::BUFFER_WIDTH;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    disabled: bool,
}

impl Cursor {
    pub unsafe fn update(&self) {
        if self.disabled {
            return;
        }
        let pos: u16 = self.x as u16 * BUFFER_WIDTH as u16 + self.y as u16;
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

    pub unsafe fn disable(&mut self) {
        if self.disabled {
            return;
        }
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
        self.disabled = true;
    }
}
