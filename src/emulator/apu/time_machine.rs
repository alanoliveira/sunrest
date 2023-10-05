use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    pulse1: channels::pulse::Pulse,
    pulse2: channels::pulse::Pulse,
    triangle: channels::triangle::Triangle,
    noise: channels::noise::Noise,
    dmc: channels::dmc::Dmc,
    irq_inhibit: bool,
    sequencer_period: SequencerPeriod,
    timer_cycle: usize,
    sequencer_cycle: usize,
}

impl TimeMachine {
    pub fn save(ppu: &Apu) -> Self {
        Self {
            pulse1: ppu.pulse1.clone(),
            pulse2: ppu.pulse2.clone(),
            triangle: ppu.triangle.clone(),
            noise: ppu.noise.clone(),
            dmc: ppu.dmc.clone(),
            irq_inhibit: ppu.irq_inhibit,
            sequencer_period: ppu.sequencer_period,
            timer_cycle: ppu.timer_cycle,
            sequencer_cycle: ppu.sequencer_cycle,
        }
    }

    pub fn load(self, ppu: &mut Apu) {
        ppu.pulse1 = self.pulse1;
        ppu.pulse2 = self.pulse2;
        ppu.triangle = self.triangle;
        ppu.noise = self.noise;
        ppu.dmc = self.dmc;
        ppu.irq_inhibit = self.irq_inhibit;
        ppu.sequencer_period = self.sequencer_period;
        ppu.timer_cycle = self.timer_cycle;
        ppu.sequencer_cycle = self.sequencer_cycle;
    }
}
