mod duty_cycle;
mod sweep;

use super::*;
use duty_cycle::*;
use sweep::*;

#[derive(Clone)]
pub enum Kind {
    Pulse1,
    Pulse2,
}

#[derive(Clone)]
pub struct Pulse {
    pub length: Length,
    timer: Timer,
    duty_cycle: DutyCycle,
    sequencer: Sequencer<8>,
    kind: Kind,
    envelope: Envelope,
    sweep: Sweep,
}

impl Pulse {
    pub fn new(kind: Kind) -> Self {
        Self {
            timer: Timer::new(0),
            duty_cycle: DutyCycle::Duty12_5,
            sequencer: Sequencer::default(),
            length: Length::default(),
            envelope: Envelope::default(),
            sweep: Sweep::default(),
            kind,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => {
                self.duty_cycle = (val >> 6).into();
                self.envelope.timer.period = (val & 0x0F) as u16;
                self.envelope.repeat = val & 0x20 != 0;
                self.length.halted = val & 0x20 != 0;
                self.envelope.fade = val & 0x10 == 0;
                self.envelope.start();
            }
            0x01 => self.sweep = val.into(),
            0x02 => self.timer.set_period_lo(val),
            0x03 => {
                self.timer.set_period_hi(val & 0x07);
                self.sequencer.reset();
                self.length.set_by_index(val >> 3);
            }
            _ => unreachable!(),
        }
    }

    pub fn output(&self) -> u8 {
        let active = self.length.enabled()
            && self.duty_cycle.output(self.sequencer.get())
            && self.timer.period >= 8
            && self.timer.period <= 0x7FF;
        if active {
            self.envelope.output()
        } else {
            0
        }
    }

    pub fn clock_timer(&mut self) {
        if self.timer.clock() {
            self.sequencer.clock();
        }
    }

    pub fn clock_length(&mut self) {
        self.length.clock();
    }

    pub fn clock_envelope(&mut self) {
        self.envelope.clock();
    }

    pub fn clock_sweep(&mut self) {
        if self.sweep.active() {
            let mut delta = self.timer.period >> self.sweep.shift();
            if self.sweep.negate() {
                delta = match self.kind {
                    Kind::Pulse1 => !delta,
                    Kind::Pulse2 => !delta + 1,
                };
            }
            self.timer.increment_period(delta)
        }
        self.sweep.clock();
    }
}
