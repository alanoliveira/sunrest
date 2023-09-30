mod cartridge_io;
mod ppu_regs;
mod wram;

pub use cartridge_io::*;
pub use ppu_regs::*;

use super::*;

const WRAM_START: u16 = 0x0000;
const WRAM_END: u16 = 0x1FFF;

const PRG_START: u16 = 0x8000;
const PRG_END: u16 = 0xFFFF;

const OAM_DMA_ADDR: u16 = 0x4014;

const PPU_REGS_START: u16 = 0x2000;
const PPU_REGS_OAM_WRITE: u16 = 0x2004;
const PPU_REGS_END: u16 = 0x3FFF;

const INPUT_PORT_CTRL_ADDR: u16 = 0x4016;
const INPUT_PORT_1_ADDR: u16 = 0x4016;
const INPUT_PORT_2_ADDR: u16 = 0x4017;

pub struct Bus {
    cartridge_io: Box<dyn cartridge_io::CartridgeIO>,
    ppu_regs: ppu_regs::PpuRegs,
    wram: wram::Wram,
    oam_dma_page: Option<u8>,

    pub input_ctrl_write: Option<u8>,
    pub device1_state: std::cell::Cell<Option<u8>>,
    pub device2_state: std::cell::Cell<Option<u8>>,
}

impl Bus {
    pub fn new(cartridge_io: Box<dyn CartridgeIO>, ppu_regs_io: Box<dyn PpuRegsIO>) -> Self {
        Self {
            cartridge_io,
            ppu_regs: ppu_regs::PpuRegs(ppu_regs_io),
            wram: wram::Wram::new(),
            oam_dma_page: None,

            input_ctrl_write: None,
            device1_state: std::cell::Cell::new(None),
            device2_state: std::cell::Cell::new(None),
        }
    }

    pub fn take_oam_dma_page(&mut self) -> Option<u8> {
        self.oam_dma_page.take()
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_START..=WRAM_END => self.wram.write(addr - WRAM_START, val),
            PPU_REGS_START..=PPU_REGS_END => self.ppu_regs.write(addr, val),
            OAM_DMA_ADDR => self.oam_dma_page = Some(val),
            INPUT_PORT_CTRL_ADDR => self.input_ctrl_write = Some(val),
            _ => log!("Attempted to write to unmapped CPU address: {addr:04X}"),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            WRAM_START..=WRAM_END => self.wram.read(addr - WRAM_START),
            PPU_REGS_START..=PPU_REGS_END => self.ppu_regs.read(addr),
            PRG_START..=PRG_END => self.cartridge_io.read(addr - PRG_START),
            INPUT_PORT_1_ADDR => self.device1_state.take().unwrap_or(0),
            INPUT_PORT_2_ADDR => self.device2_state.take().unwrap_or(0),
            _ => {
                log!("Attempted to read from unmapped CPU address: {addr:04X}");
                0
            }
        }
    }
}

impl cpu::Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        self.read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.write(addr, val)
    }
}

impl oam_dma::IO for Bus {
    fn read(&self, addr: u16) -> u8 {
        self.read(addr)
    }

    fn write(&mut self, val: u8) {
        self.write(PPU_REGS_OAM_WRITE, val)
    }
}
