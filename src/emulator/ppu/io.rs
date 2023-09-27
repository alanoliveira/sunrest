use super::*;

pub struct IO<'a>(&'a mut Ppu);

impl IO<'_> {
    pub fn new(ppu: &mut Ppu) -> IO {
        IO(ppu)
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x02 => self.read_status(),
            0x07 => self.read_data(),
            0x00..=0x07 => {
                log!("Attempted to read from unimplemented PPU address: {addr:04X}");
                0
            }
            _ => panic!("Invalid PPU read address: {:#06x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x00 => self.write_ctrl(val),
            0x01 => self.write_mask(val),
            0x05 => self.write_scroll(val),
            0x06 => self.write_address(val),
            0x07 => self.write_data(val),
            0x00..=0x07 => {
                log!("Attempted to write {val:02X} to unimplemented PPU address: {addr:04X}");
            }
            _ => panic!("Invalid PPU write address: {:#06x}", addr),
        }
    }

    pub fn read_status(&mut self) -> u8 {
        (self.0.regs.vblank.get() as u8) << 7
            | (self.0.regs.spr0_hit as u8) << 6
            | (self.0.regs.spr_overflow as u8) << 5
    }

    pub fn read_data(&mut self) -> u8 {
        let ret = self.0.regs.vram_data;
        let addr = self.0.regs.vram_addr.get();
        self.0.regs.vram_data = self.0.bus.read(addr);
        self.0.regs.increment_vram_address();
        ret
    }

    pub fn write_ctrl(&mut self, val: u8) {
        self.0.regs.name_table = match val & 0x03 {
            0 => registers::NameTable::Zero,
            1 => registers::NameTable::One,
            2 => registers::NameTable::Two,
            3 => registers::NameTable::Three,
            _ => unreachable!(),
        };

        self.0.regs.addres_increment = match val & 0x04 != 0 {
            false => registers::AddressIncrement::Increment1,
            true => registers::AddressIncrement::Increment32,
        };
        self.0.regs.spr_pattern_table = match val & 0x08 != 0 {
            false => registers::PatternTable::Zero,
            true => registers::PatternTable::One,
        };
        self.0.regs.bg_pattern_table = match val & 0x10 != 0 {
            false => registers::PatternTable::Zero,
            true => registers::PatternTable::One,
        };
        self.0.regs.spr_height = match val & 0x20 != 0 {
            false => registers::SprHeight::Eight,
            true => registers::SprHeight::Sixteen,
        };
        self.0.regs.nmi_enabled = val & 0x80 != 0;
    }

    pub fn write_mask(&mut self, val: u8) {
        self.0.regs.clip_bg = val & 0b0000_0010 == 0;
        self.0.regs.clip_spr = val & 0b0000_0100 == 0;
        self.0.regs.show_bg = val & 0b0000_1000 != 0;
        self.0.regs.show_spr = val & 0b0001_0000 != 0;
    }

    pub fn write_scroll(&mut self, val: u8) {
        match self.0.regs.latch.take() {
            None => self.0.regs.latch = Some(val),
            Some(latch) => self.0.regs.set_scroll(latch, val),
        }
    }

    pub fn write_address(&mut self, val: u8) {
        match self.0.regs.latch.take() {
            None => self.0.regs.latch = Some(val),
            Some(latch) => {
                let new_addr = (latch as u16) << 8 | val as u16;
                self.0.regs.set_vram_address(new_addr);
            }
        }
    }

    pub fn write_data(&mut self, val: u8) {
        self.0.bus.write(self.0.regs.vram_addr.get(), val);
        self.0.regs.increment_vram_address();
    }
}
