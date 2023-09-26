const PPU_REGS_BIT_MASK: u16 = 0x07;

pub trait PpuRegsIO {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub struct PpuRegs(pub Box<dyn PpuRegsIO>);

impl PpuRegs {
    pub fn read(&self, addr: u16) -> u8 {
        self.0.read(addr & PPU_REGS_BIT_MASK)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0.write(addr & PPU_REGS_BIT_MASK, val)
    }
}
