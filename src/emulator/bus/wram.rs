const WRAM_SIZE: usize = 0x0800;
const WRAM_BIT_MASK: u16 = 0x07FF;

#[derive(Clone)]
pub struct Wram(Box<[u8; WRAM_SIZE]>);

impl Wram {
    pub fn new() -> Self {
        Self(Box::new([0; WRAM_SIZE]))
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[self.resolve_address(addr) as usize] = val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[self.resolve_address(addr) as usize]
    }

    fn resolve_address(&self, addr: u16) -> u16 {
        addr & WRAM_BIT_MASK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wram() {
        let mut wram = Wram::new();
        wram.write(0x0000, 0x01);
        wram.write(0x0001, 0x02);
        wram.write(0x0802, 0x03);
        wram.write(0x1003, 0x04);

        assert_eq!(wram.read(0x0000), 0x01);
        assert_eq!(wram.read(0x0001), 0x02);
        assert_eq!(wram.read(0x0002), 0x03);
        assert_eq!(wram.read(0x0003), 0x04);
        assert_eq!(wram.read(0x0800), 0x01);
        assert_eq!(wram.read(0x0801), 0x02);
        assert_eq!(wram.read(0x1002), 0x03);
        assert_eq!(wram.read(0x1003), 0x04);
    }
}
