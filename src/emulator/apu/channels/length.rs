#[derive(Debug, Default, Clone, Copy)]
pub struct Length(u8);

impl Length {
    pub fn clock(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn output(&self) -> bool {
        self.0 > 0
    }

    pub fn set(&mut self, val: u8) {
        self.0 = LENGTH_TABLE[val as usize];
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];
