mod control;
mod status;

use super::*;

pub use control::Control;
pub use status::Status;

pub struct IO<'a>(&'a mut Ppu);

impl IO<'_> {
    pub fn new(ppu: &mut Ppu) -> IO {
        IO(ppu)
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x02 => self.read_status().into(),
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
            0x00..=0x07 => {
                log!("Attempted to write {val:02X} to unimplemented PPU address: {addr:04X}");
            }
            _ => panic!("Invalid PPU write address: {:#06x}", addr),
        }
    }

    pub fn read_status(&mut self) -> Status {
        let status = Status {
            vertical_blank: self.0.vblank.get(),
        };

        self.0.vblank.stop();
        status
    }

    pub fn write_ctrl(&mut self, val: impl Into<Control>) {
        let ctrl = val.into();
        self.0.vblank.enable_nmi(ctrl.nmi_enabled);
    }
}
