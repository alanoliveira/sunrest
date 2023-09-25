mod bus;
mod cpu;

pub struct Emulator {
    cpu: cpu::Cpu<bus::Bus>,

    cycle: usize,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(bus::Bus::new()),

            cycle: 0,
        }
    }

    pub fn clock(&mut self) {
        // ~1.79mhz
        if self.cycle % 12 == 0 {
            self.cpu.clock();
        }

        self.cycle += 1;
    }
}
