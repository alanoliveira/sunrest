mod linear_counter;

use super::*;
use linear_counter::*;

pub struct Triangle {
    linear_counter: LinearCounter,
    sequencer: Sequencer<32>,
    timer: Timer,
    pub length: Length,
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            sequencer: Sequencer::default(),
            timer: Timer::default(),
            length: Length::default(),
            linear_counter: LinearCounter::default(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                self.length.halted = val & 0x80 != 0;
                self.linear_counter.set_control_flag(val & 0x80 != 0);
                self.linear_counter.set_load(val & 0x7F);
            }
            0x01 => {} // unused
            0x02 => self.timer.set_period_lo(val),
            0x03 => {
                self.timer.set_period_hi(val & 0x07);
                self.linear_counter.set_reload_flag(true);
                self.length.set_by_index(val >> 3);
            }
            _ => unreachable!(),
        }
    }

    pub fn clock_timer(&mut self) {
        if self.timer.clock() && self.length.enabled() && !self.linear_counter.ended() {
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
        let active = self.length.enabled() && self.timer.period >= 3;

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
