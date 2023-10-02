#[derive(Debug, Default, Clone, Copy)]
pub struct Sequencer {
    duty_cycle: u8,
    sequence: u8,
}

impl Sequencer {
    pub fn clock(&mut self) {
        self.sequence = (self.sequence + 1) % 8;
    }

    pub fn output(&self) -> bool {
        self.duty_cycle & (1 << self.sequence) != 0
    }

    pub fn set(&mut self, duty_cycle: u8) {
        self.duty_cycle = DUTY_CYCLES[duty_cycle as usize];
    }

    pub fn reset(&mut self) {
        self.sequence = 0;
    }
}

const DUTY_CYCLES: [u8; 4] = [0b0100_0000, 0b0110_0000, 0b0111_1000, 0b1001_1111];
