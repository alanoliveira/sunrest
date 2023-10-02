use super::*;

pub struct Triangle {
    enabled: bool,
    linear_counter: LinearCounter,
    sequencer: Sequencer<32>,
    timer: Timer,
    length: Length,
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            enabled: false,
            sequencer: Sequencer::default(),
            timer: Timer::default(),
            length: Length::default(),
            linear_counter: LinearCounter::default(),
        }
    }

    pub fn enabled(&self) -> bool {
        self.length.output()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.length.reset();
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                self.linear_counter.set_enabled(val & 0x80 != 0);
                self.linear_counter.timer.set_period((val & 0x7F) as u16);
            }
            0x01 => {} // unused
            0x02 => {
                self.timer
                    .set_period((self.timer.period() & 0xFF00) | val as u16);
            }
            0x03 => {
                self.timer
                    .set_period((self.timer.period() & 0x00FF) | ((val as u16 & 0x07) << 8));
                self.linear_counter.halt = true;

                if self.enabled {
                    self.length.set(val >> 3);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn clock_timer(&mut self) {
        if self.timer.clock() && self.length.output() && !self.linear_counter.timer.is_zero() {
            self.sequencer.clock();
        }
    }

    pub fn clock_linear_counter(&mut self) {
        self.linear_counter.clock();
    }

    pub fn clock_length(&mut self) {
        self.length.clock();
    }

    pub fn output(&self) -> u8 {
        let active = self.enabled && self.length.output() && self.timer.period() >= 3;

        if active {
            TRIANGLE_SEQUENCE[self.sequencer.get() as usize]
        } else {
            0
        }
    }
}

const TRIANGLE_SEQUENCE: [u8; 32] = [
    0xF, 0xE, 0xD, 0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0, 0x0, 0x1, 0x2,
    0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
];
