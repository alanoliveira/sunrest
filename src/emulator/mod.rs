mod bus;
mod cartridge;
mod cpu;
mod oam_dma;
mod ppu;
mod video;

pub mod input_devices;

pub use video::{Color, Signal as VideoSignal};

use input_devices::InputDevice;

use std::{cell::RefCell, rc::Rc};

pub struct Emulator {
    cpu: cpu::Cpu<bus::Bus>,
    ppu: PpuWrapper,
    oam_dma: oam_dma::OamDma,

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
            oam_dma: oam_dma::OamDma::new(),

            color_palette: video::DEFAULT_PALETTE.clone(),
            cycle: 0,
        }
    }

    pub fn video_signal(&self) -> video::Signal {
        let ppu = self.ppu.as_ref();
        video::Signal {
            x: ppu.dot.wrapping_sub(1),
            y: ppu.scanline,
            color: self.color_palette[ppu.color_idx],
        }
    }

    pub fn clock_input_devices(&mut self, dev1: &mut dyn InputDevice, dev2: &mut dyn InputDevice) {
        if let Some(input_ctrl) = self.cpu.io.input_ctrl_write.take() {
            dev1.write(input_ctrl);
            self.cpu.io.device1_state.replace(None);
            dev2.write(input_ctrl);
            self.cpu.io.device2_state.replace(None);
        } else {
            if self.cpu.io.device1_state.get().is_none() {
                self.cpu.io.device1_state.replace(Some(dev1.read()));
            }
            if self.cpu.io.device2_state.get().is_none() {
                self.cpu.io.device2_state.replace(Some(dev2.read()));
            }
        }
    }

    pub fn clock(&mut self) {
        // ~1.79mhz
        if self.cycle % 12 == 0 {
            if self.oam_dma.is_active() {
                self.clock_oam_dma();
            } else {
                self.clock_cpu();
            }
        }

        // ~53.69mhz
        if self.cycle % 4 == 0 {
            let mut ppu = self.ppu.as_mut();
            ppu.clock();
            if ppu.take_nmi() {
                self.cpu.set_signal(cpu::Signal::Nmi);
            }
        }

        self.cycle += 1;
    }

    fn clock_cpu(&mut self) {
        self.cpu.clock();
        if let Some(page) = self.cpu.io.take_oam_dma_page() {
            self.oam_dma.prepare(page);
        }
    }

    fn clock_oam_dma(&mut self) {
        if (self.cycle / 12) % 2 == 1 {
            self.oam_dma.write(&mut self.cpu.io);
        } else {
            self.oam_dma.read(&self.cpu.io);
        }
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
        self.0.borrow_mut().io().read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.0.borrow_mut().io().write(addr, val);
    }
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
