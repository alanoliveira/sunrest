use super::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Sweep {
    pub enabled: bool,
    pub reload_flag: bool,
    pub negate: bool,
    pub shift: u8,
    pub timer: Timer,
}

impl Sweep {
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
