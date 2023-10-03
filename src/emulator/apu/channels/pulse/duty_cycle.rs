#[derive(Debug, Clone, Copy)]
pub enum DutyCycle {
    Duty12_5 = 0b0100_0000,
    Duty25 = 0b0110_0000,
    Duty50 = 0b0111_1000,
    Duty25Neg = 0b1001_1111,
}

impl DutyCycle {
    pub fn output(&self, step: u8) -> bool {
        let cyc = *self as u8;
        cyc & (1 << step) != 0
    }
}

impl From<u8> for DutyCycle {
    fn from(value: u8) -> Self {
        match value {
            0b00 => DutyCycle::Duty12_5,
            0b01 => DutyCycle::Duty25,
            0b10 => DutyCycle::Duty50,
            0b11 => DutyCycle::Duty25Neg,
            _ => unreachable!(),
        }
    }
}
