#[derive(Clone)]
pub enum ShiftMode {
    One,
    Six,
}

#[derive(Clone)]
pub struct Shift {
    data: u16,
    mode: ShiftMode,
}

impl Default for Shift {
    fn default() -> Self {
        Self {
            data: 1,
            mode: ShiftMode::One,
        }
    }
}

impl Shift {
    pub fn set_mode(&mut self, mode: ShiftMode) {
        self.mode = mode;
    }

    pub fn clock(&mut self) {
        let random_bit = self.random_bit();
        self.data >>= 1;
        self.data |= random_bit << 14;
    }

    pub fn output(&self) -> bool {
        (self.data & 1) == 0
    }

    fn random_bit(&self) -> u16 {
        let bit1 = self.data & 1;
        let bit2 = match self.mode {
            ShiftMode::One => (self.data >> 1) & 1,
            ShiftMode::Six => (self.data >> 6) & 1,
        };
        bit1 ^ bit2
    }
}
