#[cfg(test)]
mod tests;

mod apu;
mod audio;
mod bus;
mod cpu;
mod dmc_dma;
mod oam_dma;
mod ppu;
mod time_machine;
mod video;

pub mod cartridge;
pub use audio::Signal as AudioSignal;
pub use bus::InputPort;
pub use time_machine::TimeMachine;
pub use video::{Color, Signal as VideoSignal};
pub mod input_devices;

use std::{cell::RefCell, rc::Rc};

type Cpu = cpu::Cpu<bus::Bus>;
type Ppu = ppu::Ppu<ppu::bus::Bus>;

pub struct Emulator {
    cpu: Cpu,
    ppu: PpuWrapper,
    apu: ApuWrapper,
    oam_dma: oam_dma::OamDma,
    dmc_dma: dmc_dma::DmcDma,
    cartridge: Rc<RefCell<cartridge::Cartridge>>,

    color_palette: [video::Color; 64],
    pub cycle: usize,
}

impl Emulator {
    pub fn new(cartridge: cartridge::Cartridge) -> Self {
        let cartridge = Rc::new(RefCell::new(cartridge));

        let ppu_cartridge = PpuCartridge(cartridge.clone());
        let ppu_bus = ppu::bus::Bus::new(Box::new(ppu_cartridge));
        let ppu = Rc::new(RefCell::new(ppu::Ppu::new(ppu_bus)));

        let apu = Rc::new(RefCell::new(apu::Apu::new()));

        let cpu_cartridge = CpuCartridge(cartridge.clone());
        let ppu_regs = PpuWrapper(ppu.clone());
        let apu_regs = ApuWrapper(apu.clone());
        let bus = bus::Bus::new(
            Box::new(cpu_cartridge),
            Box::new(ppu_regs),
            Box::new(apu_regs),
        );

        let mut cpu = cpu::Cpu::new(bus);

        cpu.reset();

        Self {
            cpu,
            ppu: PpuWrapper(ppu),
            apu: ApuWrapper(apu),
            oam_dma: oam_dma::OamDma::new(),
            dmc_dma: dmc_dma::DmcDma::new(),
            cartridge,

            color_palette: video::DEFAULT_PALETTE,
            cycle: 0,
        }
    }

    pub fn connect_port1(&mut self, port: Option<Box<dyn InputPort>>) {
        self.cpu.mem.port1 = port;
    }

    pub fn connect_port2(&mut self, port: Option<Box<dyn InputPort>>) {
        self.cpu.mem.port2 = port;
    }

    pub fn rom_info(&self) -> cartridge::RomInfo {
        self.cartridge.borrow().rom_info().clone()
    }

    pub fn save_state(&self) -> TimeMachine {
        TimeMachine::save(self)
    }

    pub fn load_state(&mut self, state: TimeMachine) {
        state.load(self);
    }

    pub fn video_signal(&self) -> video::Signal {
        let ppu = self.ppu.as_ref();
        video::Signal {
            x: ppu.dot.wrapping_sub(1),
            y: ppu.scanline,
            color: self.color_palette[ppu.color_idx],
        }
    }

    pub fn audio_signal(&self) -> audio::Signal {
        let apu = self.apu.as_ref();
        audio::Signal {
            pulse1: apu.pulse1.output(),
            pulse2: apu.pulse2.output(),
            triangle: apu.triangle.output(),
            noise: apu.noise.output(),
            dmc: apu.dmc.output(),
        }
    }

    pub fn clock(&mut self) {
        self.cpu.mem.update_ports_latch();

        // ~1.79mhz
        if self.cycle % 12 == 0 {
            if self.oam_dma.is_active() {
                self.clock_oam_dma();
            } else if self.dmc_dma.is_active() {
                self.clock_dmc_dma();
            } else {
                self.clock_cpu();
                self.check_dmc_dma();
            }

            self.apu.as_mut().clock_timer();
            self.check_dmc_dma();
        }

        // ~53.69mhz
        if self.cycle % 4 == 0 {
            {
                let mut ppu = self.ppu.as_mut();
                ppu.clock();
                if ppu.take_nmi() {
                    self.cpu.set_signal(cpu::Signal::Nmi);
                }
            }
            if self.cartridge.borrow_mut().take_irq() {
                self.cpu.set_signal(cpu::Signal::Irq);
            }
        }

        // ~240hz
        if self.cycle % 89490 == 0 {
            self.apu.as_mut().clock_sequencer();
        }

        self.cycle += 1;
    }

    fn clock_cpu(&mut self) {
        self.cpu.clock();
        if let Some(page) = self.cpu.mem.take_oam_dma_page() {
            self.oam_dma.prepare(page);
        }
    }

    fn clock_oam_dma(&mut self) {
        if (self.cycle / 12) % 2 == 1 {
            self.oam_dma.write(&mut self.cpu.mem);
        } else {
            self.oam_dma.read(&self.cpu.mem);
        }
    }

    fn clock_dmc_dma(&mut self) {
        let mut apu = self.apu.as_mut();
        if apu.is_hi_cycle() {
            self.dmc_dma.dummy();
        } else {
            self.dmc_dma.read(&self.cpu.mem);
            if !self.dmc_dma.is_active() {
                apu.dmc.load_sample_buffer(self.dmc_dma.buffer);
            }
        }
    }

    fn check_dmc_dma(&mut self) {
        if self.dmc_dma.is_active() {
            return;
        }

        if let Some(dmc_dma_addr) = self.apu.as_ref().dmc.is_waiting() {
            self.dmc_dma.prepare(dmc_dma_addr);
        }
    }
}

struct ApuWrapper(Rc<RefCell<apu::Apu>>);

impl ApuWrapper {
    fn as_mut(&mut self) -> std::cell::RefMut<apu::Apu> {
        self.0.borrow_mut()
    }

    fn as_ref(&self) -> std::cell::Ref<apu::Apu> {
        self.0.borrow()
    }
}

impl bus::Addressable for ApuWrapper {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow_mut().read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.0.borrow_mut().write(addr, val);
    }
}

struct PpuWrapper(Rc<RefCell<Ppu>>);

impl PpuWrapper {
    fn as_mut(&mut self) -> std::cell::RefMut<Ppu> {
        self.0.borrow_mut()
    }

    fn as_ref(&self) -> std::cell::Ref<Ppu> {
        self.0.borrow()
    }
}

impl bus::Addressable for PpuWrapper {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow_mut().io_ports().read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.0.borrow_mut().io_ports().write(addr, val);
    }
}

struct PpuCartridge(Rc<RefCell<cartridge::Cartridge>>);
impl ppu::bus::CartridgeIO for PpuCartridge {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow().read_chr(addr)
    }

    fn write(&self, addr: u16, val: u8) {
        self.0.borrow_mut().write_chr(addr, val)
    }

    fn mirror_mode(&self) -> cartridge::MirrorMode {
        self.0.borrow().mirror_mode()
    }
}

struct CpuCartridge(Rc<RefCell<cartridge::Cartridge>>);
impl bus::Addressable for CpuCartridge {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow().read_prg(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.0.borrow_mut().write_prg(addr, val);
    }
}
