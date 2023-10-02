use super::*;

const MAX_DECAY: u8 = 15;

#[derive(Debug, Default, Clone, Copy)]
pub struct Envelope {
    decay: u8,
    start_flag: bool,
    pub fade: bool,
    pub repeat: bool,
    pub timer: Timer,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            decay: 0,
            start_flag: false,
            fade: false,
            repeat: false,
            timer: Timer::new(0),
        }
    }

    pub fn output(&self) -> u8 {
        if self.fade {
            self.decay
        } else {
            self.timer.period() as u8
        }
    }

    pub fn start(&mut self) {
        self.start_flag = true;
    }

    pub fn clock(&mut self) {
        let start_flag = self.start_flag;
        if start_flag {
            self.start_flag = false;
            self.decay = MAX_DECAY;
            self.timer.reset();
            self.timer.clock();
        }

        if self.timer.clock() && !start_flag {
            if self.repeat {
                self.decay = MAX_DECAY;
            } else if self.decay > 0 {
                self.decay -= 1;
            }
        }
    }
}
