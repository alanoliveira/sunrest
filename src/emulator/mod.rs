mod bus;
mod cartridge;
mod cpu;

pub struct Emulator {
    cpu: cpu::Cpu<bus::Bus>,

    cycle: usize,
}

impl Emulator {
    pub fn new<P>(rom_path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let cartridge = cartridge::open_rom(rom_path.as_ref());

        let mut cpu = cpu::Cpu::new(bus::Bus::new(cartridge));
        cpu.reset();

        Self { cpu, cycle: 0 }
    }

    pub fn clock(&mut self) {
        // ~1.79mhz
        if self.cycle % 12 == 0 {
            self.cpu.clock();
        }

        self.cycle += 1;
    }
}
