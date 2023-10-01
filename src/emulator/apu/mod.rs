enum SequencerPeriod {
    FourSteps,
    FiveSteps,
}

pub struct Apu {
    irq_inhibit: bool,
    sequencer_period: SequencerPeriod,
    timer_cycle: usize,
    sequencer_cycle: usize,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            irq_inhibit: false,
            sequencer_period: SequencerPeriod::FourSteps,
            timer_cycle: 0,
            sequencer_cycle: 0,
        }
    }

    pub fn clock_timer(&mut self) {
        self.timer_cycle += 1;
    }

    pub fn clock_sequencer(&mut self) {
        match self.sequencer_period {
            SequencerPeriod::FourSteps => match self.sequencer_cycle % 4 {
                _ => {}
            },
            SequencerPeriod::FiveSteps => match self.sequencer_cycle % 5 {
                _ => {}
            },
        }
        self.sequencer_cycle += 1;
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x17 => {
                if val & 0x80 == 0 {
                    self.sequencer_period = SequencerPeriod::FourSteps;
                } else {
                    self.sequencer_period = SequencerPeriod::FiveSteps;
                }
                self.irq_inhibit = val & 0x40 != 0;
            }
            _ => log!("Attempted to write {val:02X} to unmapped APU address: {addr:04X}"),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            _ => {
                log!("Attempted to read from unmapped APU address: {addr:04X}");
                0
            }
        }
    }
}
