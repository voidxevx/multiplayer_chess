#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BoardLocation {
    x: u32,
    y: u32,
}

impl BoardLocation {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    pub fn transpose(&self) -> Option<u32> {
        if self.x > 8 || self.y > 8 {
            return None;
        }
        Some((self.y * 8) + self.x)
    }
}