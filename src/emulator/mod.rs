mod bus;
mod cartridge;
mod cpu;
mod ppu;
mod video;

use std::{cell::RefCell, rc::Rc};

pub use video::{Color, Signal as VideoSignal};

pub struct Emulator {
    cpu: cpu::Cpu<bus::Bus>,
    ppu: PpuWrapper,

    color_palette: [video::Color; 64],
    cycle: usize,
}

impl Emulator {
    pub fn new<P>(rom_path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let cartridge = Rc::new(RefCell::new(cartridge::open_rom(rom_path.as_ref())));

        let ppu_cartridge = PpuCartridge(cartridge.clone());
        let ppu_bus = ppu::bus::Bus::new(Box::new(ppu_cartridge));
        let ppu = Rc::new(RefCell::new(ppu::Ppu::new(ppu_bus)));

        let cpu_cartridge = CpuCartridge(cartridge.clone());
        let ppu_regs = PpuRegs(ppu.clone());
        let bus = bus::Bus::new(Box::new(cpu_cartridge), Box::new(ppu_regs));
        let mut cpu = cpu::Cpu::new(bus);

        cpu.reset();

        Self {
            cpu,
            ppu: PpuWrapper(ppu),

            color_palette: video::DEFAULT_PALETTE.clone(),
            cycle: 0,
        }
    }

    pub fn video_signal(&self) -> video::Signal {
        let ppu = self.ppu.as_ref();
        video::Signal {
            x: ppu.dot,
            y: ppu.scanline,
            color: self.color_palette[ppu.color_idx],
        }
    }

    pub fn clock(&mut self) {
        // ~1.79mhz
        if self.cycle % 12 == 0 {
            self.cpu.clock();
        }

        // ~53.69mhz
        if self.cycle % 4 == 0 {
            self.ppu.as_mut().clock();
        }

        self.cycle += 1;
    }
}

struct PpuWrapper(Rc<RefCell<ppu::Ppu>>);

impl PpuWrapper {
    fn as_mut(&mut self) -> std::cell::RefMut<ppu::Ppu> {
        self.0.borrow_mut()
    }

    fn as_ref(&self) -> std::cell::Ref<ppu::Ppu> {
        self.0.borrow()
    }
}

struct PpuRegs(Rc<RefCell<ppu::Ppu>>);
impl bus::PpuRegsIO for PpuRegs {
    fn read(&self, addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, val: u8) {}
}

struct PpuCartridge(Rc<RefCell<cartridge::Cartridge>>);
impl ppu::bus::CartridgeIO for PpuCartridge {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow().read_chr(addr)
    }
}

struct CpuCartridge(Rc<RefCell<cartridge::Cartridge>>);
impl bus::CartridgeIO for CpuCartridge {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow().read_prg(addr)
    }
}
