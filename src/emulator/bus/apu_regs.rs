use super::*;

const APU_REGS_BIT_MASK: u16 = 0x1F;

pub struct ApuRegs(pub Box<dyn Addressable>);

impl ApuRegs {
    pub fn read(&self, addr: u16) -> u8 {
        self.0.read(addr & APU_REGS_BIT_MASK)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0.write(addr & APU_REGS_BIT_MASK, val)
    }
}
