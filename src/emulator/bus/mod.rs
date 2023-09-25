mod wram;

use super::*;

const WRAM_START: u16 = 0x0000;
const WRAM_END: u16 = 0x1FFF;

const PRG_START: u16 = 0x8000;
const PRG_END: u16 = 0xFFFF;

pub struct Bus {
    cartridge: cartridge::Cartridge,
    wram: wram::Wram,
}

impl Bus {
    pub fn new(cartridge: cartridge::Cartridge) -> Self {
        Self {
            cartridge,
            wram: wram::Wram::new(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_START..=WRAM_END => self.wram.write(addr, val),
            _ => log!("Attempted to write to unmapped CPU address: {addr:04X}"),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            WRAM_START..=WRAM_END => self.wram.read(addr),
            PRG_START..=PRG_END => self.cartridge.read_prg(addr),
            _ => {
                log!("Attempted to read from unmapped CPU address: {addr:04X}");
                0
            }
        }
    }
}

impl cpu::IO for Bus {
    fn read(&self, addr: u16) -> u8 {
        self.read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.write(addr, val)
    }
}
