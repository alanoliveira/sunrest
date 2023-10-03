use super::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Sweep {
    enabled: bool,
    reload_flag: bool,
    negate: bool,
    shift: u8,
    timer: Timer,
}

impl Sweep {
    pub fn shift(&self) -> u8 {
        self.shift
    }

    pub fn negate(&self) -> bool {
        self.negate
    }

    pub fn clock(&mut self) {
        if self.reload_flag {
            self.timer.reset();
            self.reload_flag = false;
        } else {
            self.timer.clock();
        }
    }

    pub fn active(&self) -> bool {
        self.timer.is_zero() && self.enabled && self.shift != 0
    }
}

impl From<u8> for Sweep {
    fn from(value: u8) -> Self {
        Self {
            enabled: value & 0x80 != 0,
            negate: value & 0x08 != 0,
            shift: value & 0x07,
            timer: Timer::new(((value >> 4) & 0x07) as u16),
            reload_flag: true,
        }
    }
}
