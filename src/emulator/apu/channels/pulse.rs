use super::*;

pub enum Kind {
    Pulse1,
    Pulse2,
}

pub struct Pulse {
    enabled: bool,
    timer: Timer,
    sequencer: Sequencer,
    length: Length,
    envelope: Envelope,
    sweep: Sweep,
    kind: Kind,
}

impl Pulse {
    pub fn new(kind: Kind) -> Self {
        Self {
            enabled: false,
            timer: Timer::new(0),
            sequencer: Sequencer::default(),
            length: Length::default(),
            envelope: Envelope::default(),
            sweep: Sweep::default(),
            kind,
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
                self.sequencer.set(val >> 6);
                self.envelope.timer.set_period((val & 0x0F) as u16);
                self.envelope.repeat = val & 0x20 != 0;
                self.envelope.fade = val & 0x10 == 0;
                self.envelope.start();
            }
            0x01 => {
                self.sweep.enabled = val & 0x80 != 0;
                self.sweep.negate = val & 0x08 != 0;
                self.sweep.shift = val & 0x07;
                self.sweep.timer.set_period((val >> 4) as u16 & 0x07);
                self.sweep.reload_flag = true;
            }
            0x02 => self.timer.set_period_lo(val),
            0x03 => {
                self.timer.set_period_hi(val & 0x07);
                self.sequencer.reset();
                if self.enabled {
                    self.length.set(val >> 3);
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn output(&self) -> u8 {
        let active = self.enabled
            && self.sequencer.output()
            && self.length.output()
            && self.timer.period() >= 8
            && self.timer.period() <= 0x7FF;
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
            let mut delta = self.timer.period() >> self.sweep.shift;
            if self.sweep.negate {
                delta = match self.kind {
                    Kind::Pulse1 => !delta,
                    Kind::Pulse2 => !delta + 1,
                };
            }
            self.timer
                .set_period(self.timer.period().wrapping_add(delta));
        }
        self.sweep.clock();
    }
}
