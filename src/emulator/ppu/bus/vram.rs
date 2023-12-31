const VRAM_SIZE: usize = 0x0800;
const VRAM_BIT_MASK: u16 = 0x07FF; // @TODO: temporary, fix when implementing mirroring

#[derive(Clone)]
pub struct Vram(Box<[u8; VRAM_SIZE]>);

impl Vram {
    pub fn new() -> Self {
        Self(Box::new([0; VRAM_SIZE]))
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[self.resolve_address(addr) as usize] = val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[self.resolve_address(addr) as usize]
    }

    fn resolve_address(&self, addr: u16) -> u16 {
        addr & VRAM_BIT_MASK
    }
}
