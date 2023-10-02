use super::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct LinearCounter {
    pub enabled: bool,
    pub timer: Timer,
    pub halt: bool,
}

impl LinearCounter {
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.halt = !enabled;
    }

    pub fn clock(&mut self) {
        if self.halt {
            self.timer.reset();
        } else {
            self.timer.clock();
        }

        if self.enabled {
            self.halt = false;
        }
    }

    pub fn halt(&mut self) {
        self.halt = true;
    }
}
