mod channels;

enum SequencerPeriod {
    FourSteps,
    FiveSteps,
}

pub struct Apu {
    pub pulse1: channels::pulse::Pulse,
    pub pulse2: channels::pulse::Pulse,
    pub triangle: channels::triangle::Triangle,

    irq_inhibit: bool,
    sequencer_period: SequencerPeriod,
    timer_cycle: usize,
    sequencer_cycle: usize,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulse1: channels::pulse::Pulse::new(channels::pulse::Kind::Pulse1),
            pulse2: channels::pulse::Pulse::new(channels::pulse::Kind::Pulse2),
            triangle: channels::triangle::Triangle::new(),

            irq_inhibit: false,
            sequencer_period: SequencerPeriod::FourSteps,
            timer_cycle: 0,
            sequencer_cycle: 0,
        }
    }

    pub fn clock_timer(&mut self) {
        self.timer_cycle += 1;
        if self.timer_cycle % 2 == 0 {
            self.pulse1.clock_timer();
            self.pulse2.clock_timer();
        }
        self.triangle.clock_timer();
    }

    pub fn clock_sequencer(&mut self) {
        match self.sequencer_period {
            SequencerPeriod::FourSteps => match self.sequencer_cycle % 4 {
                0 | 2 => {
                    self.clock_envelope();
                }
                1 | 3 => {
                    self.clock_envelope();
                    self.clock_length();
                }
                _ => {}
            },
            SequencerPeriod::FiveSteps => match self.sequencer_cycle % 5 {
                0 | 2 => {
                    self.clock_envelope();
                }
                1 | 4 => {
                    self.clock_envelope();
                    self.clock_length();
                }
                _ => {}
            },
        }
        self.sequencer_cycle += 1;
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00..=0x03 => self.pulse1.write(addr, val),
            0x04..=0x07 => self.pulse2.write(addr - 0x04, val),
            0x08..=0x0B => self.triangle.write(addr - 0x08, val),
            0x15 => {
                self.pulse1.set_enabled(val & 0x01 != 0);
                self.pulse2.set_enabled(val & 0x02 != 0);
                self.triangle.set_enabled(val & 0x04 != 0);
            }
            0x17 => {
                if val & 0x80 == 0 {
                    self.sequencer_period = SequencerPeriod::FourSteps;
                } else {
                    self.sequencer_period = SequencerPeriod::FiveSteps;
                    self.clock_envelope();
                    self.clock_length();
                }
                self.irq_inhibit = val & 0x40 != 0;
            }
            _ => {}
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x15 => {
                let mut status = 0;
                status |= self.pulse1.enabled() as u8;
                status |= (self.pulse2.enabled() as u8) << 1;
                status |= (self.triangle.enabled() as u8) << 2;
                status
            }
            _ => {
                log!("Attempted to read from unmapped APU address: {addr:04X}");
                0
            }
        }
    }

    fn clock_length(&mut self) {
        self.pulse1.clock_length();
        self.pulse2.clock_length();
        self.pulse1.clock_sweep();
        self.pulse2.clock_sweep();
        self.triangle.clock_length();
    }

    fn clock_envelope(&mut self) {
        self.pulse1.clock_envelope();
        self.pulse2.clock_envelope();
        self.triangle.clock_linear_counter();
    }
}
