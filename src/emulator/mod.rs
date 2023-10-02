#[cfg(test)]
mod tests;

mod apu;
mod audio;
mod bus;
mod cpu;
mod oam_dma;
mod ppu;
mod video;

pub mod cartridge;
pub mod input_devices;

pub use video::{Color, Signal as VideoSignal};

use input_devices::InputDevice;

use std::{cell::RefCell, rc::Rc};

type Cpu = cpu::Cpu<bus::Bus>;
type Ppu = ppu::Ppu<ppu::bus::Bus>;

pub struct Emulator {
    cpu: Cpu,
    ppu: PpuWrapper,
    apu: ApuWrapper,
    oam_dma: oam_dma::OamDma,

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

    pub fn audio_signal(&self) -> audio::Signal {
        let apu = self.apu.as_ref();
        audio::Signal {
            pulse1: apu.pulse1.output(),
            pulse2: apu.pulse2.output(),
            triangle: apu.triangle.output(),
            noise: 0,
            dmc: 0,
        }
    }

    pub fn clock_input_devices(&mut self, dev1: &mut dyn InputDevice, dev2: &mut dyn InputDevice) {
        if let Some(input_ctrl) = self.cpu.mem.input_ctrl_write.take() {
            dev1.write(input_ctrl);
            self.cpu.mem.device1_state.replace(None);
            dev2.write(input_ctrl);
            self.cpu.mem.device2_state.replace(None);
        } else {
            if self.cpu.mem.device1_state.get().is_none() {
                self.cpu.mem.device1_state.replace(Some(dev1.read()));
            }
            if self.cpu.mem.device2_state.get().is_none() {
                self.cpu.mem.device2_state.replace(Some(dev2.read()));
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

            self.apu.as_mut().clock_timer();
        }

        // ~53.69mhz
        if self.cycle % 4 == 0 {
            let mut ppu = self.ppu.as_mut();
            ppu.clock();
            if ppu.take_nmi() {
                self.cpu.set_signal(cpu::Signal::Nmi);
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
}

struct CpuCartridge(Rc<RefCell<cartridge::Cartridge>>);
impl bus::Addressable for CpuCartridge {
    fn read(&self, addr: u16) -> u8 {
        self.0.borrow().read_prg(addr)
    }

    fn write(&mut self, _: u16, _: u8) {
        log!("Attempted to write to cartridge: {addr:04X} = {val:02X}");
    }
}
