mod shift;

use super::*;
use shift::*;

#[derive(Clone)]
pub struct Noise {
    shift: Shift,
    envelope: Envelope,
    timer: Timer,
    pub length: Length,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            shift: Shift::default(),
            envelope: Envelope::default(),
            timer: Timer::default(),
            length: Length::default(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                self.envelope.fade = val & 0x10 == 0;
                self.envelope.timer.period = (val & 0x0F) as u16;
                self.envelope.repeat = val & 0x20 != 0;
                self.length.halted = val & 0x20 != 0;
                self.envelope.start();
            }
            0x01 => {} // unused
            0x02 => {
                self.timer.period = TIMER_PERIOD[(val & 0x0F) as usize];
                let mode = if val & 0x80 != 0 {
                    ShiftMode::Six
                } else {
                    ShiftMode::One
                };
                self.shift.set_mode(mode)
            }
            0x03 => {
                self.length.set_by_index(val >> 3);
                self.envelope.start();
            }
            _ => unreachable!(),
        }
    }

    pub fn clock_timer(&mut self) {
        if self.timer.clock() {
            self.shift.clock();
        }
    }

    pub fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    pub fn clock_length(&mut self) {
        self.length.clock();
    }

    pub fn output(&self) -> u8 {
        let active = self.length.enabled() && self.shift.output();
        if active {
            self.envelope.output()
        } else {
            0
        }
    }
}

const TIMER_PERIOD: [u16; 16] = [
    0x004, 0x008, 0x010, 0x020, 0x040, 0x060, 0x080, 0x0A0, 0x0CA, 0x0FE, 0x17C, 0x1FC, 0x2FA,
    0x3F8, 0x7F2, 0xFE4,
];
