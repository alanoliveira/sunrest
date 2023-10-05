const SRAM_SIZE: usize = 0x2000;

#[derive(Clone)]
pub struct Sram(Box<[u8; SRAM_SIZE]>);

impl Sram {
    pub fn new() -> Self {
        Self(Box::new([0; SRAM_SIZE]))
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[addr as usize] = val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }
}
