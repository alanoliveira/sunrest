pub enum Mode {
    One,
    Six,
}

pub struct NoiseShift {
    data: u16,
    pub mode: Mode,
}

impl NoiseShift {
    pub fn new() -> Self {
        Self {
            data: 1,
            mode: Mode::One,
        }
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
            Mode::One => (self.data >> 1) & 1,
            Mode::Six => (self.data >> 6) & 1,
        };
        bit1 ^ bit2
    }
}
