mod apu_regs;
mod ppu_regs;
mod sram;
mod time_machine;
mod wram;

pub use apu_regs::*;
pub use ppu_regs::*;
pub use time_machine::TimeMachine;

use super::*;

const WRAM_START: u16 = 0x0000;
const WRAM_END: u16 = 0x1FFF;

const PRG_START: u16 = 0x8000;
const PRG_END: u16 = 0xFFFF;

const SRAM_START: u16 = 0x6000;
const SRAM_END: u16 = 0x7FFF;

const OAM_DMA_ADDR: u16 = 0x4014;

const PPU_REGS_START: u16 = 0x2000;
const PPU_REGS_OAM_WRITE: u16 = 0x2004;
const PPU_REGS_END: u16 = 0x3FFF;

const INPUT_PORT_CTRL_ADDR: u16 = 0x4016;
const INPUT_PORT_1_ADDR: u16 = 0x4016;
const INPUT_PORT_2_ADDR: u16 = 0x4017;

const APU_REGS_START: u16 = 0x4000;
const APU_REGS_END: u16 = 0x4013;
const APU_STATUS_ADDR: u16 = 0x4015;
const APU_FRAME_COUNTER_ADDR: u16 = 0x4017;

pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub struct Bus {
    cartridge_io: Box<dyn Addressable>,
    ppu_regs: ppu_regs::PpuRegs,
    apu_regs: apu_regs::ApuRegs,
    wram: wram::Wram,
    sram: sram::Sram, // in reality this is on the cartridge
    oam_dma_page: Option<u8>,

    pub input_ctrl_write: Option<u8>,
    pub device1_state: std::cell::Cell<Option<u8>>,
    pub device2_state: std::cell::Cell<Option<u8>>,
}

impl Bus {
    pub fn new(
        cartridge_io: Box<dyn Addressable>,
        ppu_regs_io: Box<dyn Addressable>,
        apu_regs_io: Box<dyn Addressable>,
    ) -> Self {
        Self {
            cartridge_io,
            ppu_regs: ppu_regs::PpuRegs(ppu_regs_io),
            apu_regs: apu_regs::ApuRegs(apu_regs_io),
            wram: wram::Wram::new(),
            sram: sram::Sram::new(),
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
            PPU_REGS_START..=PPU_REGS_END => self.ppu_regs.write(addr - PPU_REGS_START, val),
            SRAM_START..=SRAM_END => self.sram.write(addr - SRAM_START, val),
            PRG_START..=PRG_END => self.cartridge_io.write(addr - PRG_START, val),
            OAM_DMA_ADDR => self.oam_dma_page = Some(val),
            INPUT_PORT_CTRL_ADDR => self.input_ctrl_write = Some(val),
            (APU_REGS_START..=APU_REGS_END) | APU_STATUS_ADDR | APU_FRAME_COUNTER_ADDR => {
                self.apu_regs.write(addr - APU_REGS_START, val)
            }
            _ => log!("Attempted to write to unmapped CPU address: {addr:04X}"),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            WRAM_START..=WRAM_END => self.wram.read(addr - WRAM_START),
            PPU_REGS_START..=PPU_REGS_END => self.ppu_regs.read(addr - PPU_REGS_START),
            SRAM_START..=SRAM_END => self.sram.read(addr - SRAM_START),
            PRG_START..=PRG_END => self.cartridge_io.read(addr - PRG_START),
            INPUT_PORT_1_ADDR => self.device1_state.take().unwrap_or(0),
            INPUT_PORT_2_ADDR => self.device2_state.take().unwrap_or(0),
            APU_STATUS_ADDR => self.apu_regs.read(addr - APU_REGS_START),
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
