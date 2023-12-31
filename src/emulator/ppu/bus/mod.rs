mod cartridge_io;
mod palette_ram;
mod time_machine;
mod vram;
mod vram_mirroring;

pub use cartridge_io::*;
pub use time_machine::TimeMachine;

use super::*;
use vram_mirroring::Mirroring;

const CARTRIDGE_START: u16 = 0x0000;
const CARTRIDGE_END: u16 = 0x1FFF;

const VRAM_START: u16 = 0x2000;
const VRAM_END: u16 = 0x3EFF;

const PALLETE_START: u16 = 0x3F00;
const PALLETE_END: u16 = 0x3FFF;

pub struct Bus {
    pub vram: vram::Vram,
    pub palette_ram: palette_ram::PaletteRam,
    pub cartridge_io: Box<dyn cartridge_io::CartridgeIO>,
}

impl Bus {
    pub fn new(cartridge_io: Box<dyn cartridge_io::CartridgeIO>) -> Self {
        Self {
            vram: vram::Vram::new(),
            palette_ram: palette_ram::PaletteRam::new(),
            cartridge_io,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            CARTRIDGE_START..=CARTRIDGE_END => self.cartridge_io.read(addr - CARTRIDGE_START),
            VRAM_START..=VRAM_END => self.vram.read(self.vram_addr(addr)),
            PALLETE_START..=PALLETE_END => self.palette_ram.read(addr - PALLETE_START),
            _ => {
                log!("Attempted to read from unmapped PPU address: {addr:04X}");
                0
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            CARTRIDGE_START..=CARTRIDGE_END => self.cartridge_io.write(addr - CARTRIDGE_START, val),
            VRAM_START..=VRAM_END => self.vram.write(self.vram_addr(addr), val),
            PALLETE_START..=PALLETE_END => self.palette_ram.write(addr - PALLETE_START, val),
            _ => {
                log!("Attempted to write to unmapped PPU address: {addr:04X}");
            }
        }
    }

    fn vram_addr(&self, addr: u16) -> u16 {
        self.cartridge_io.mirror_mode().mirror(addr - VRAM_START)
    }
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 {
        self.read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.write(addr, val)
    }
}
