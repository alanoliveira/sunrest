const PALETTE_RAM_SIZE: usize = 0x20;
const PALETE_RAM_MASK: u16 = 0x001F;

#[derive(Clone)]
pub struct PaletteRam(Box<[u8; PALETTE_RAM_SIZE]>);

impl Default for PaletteRam {
    fn default() -> Self {
        Self(Box::new(PALETTE_POWER_UP_STATE))
    }
}

impl PaletteRam {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[self.resolve_address(addr)] = val & 0x3F;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[self.resolve_address(addr)]
    }

    fn resolve_address(&self, addr: u16) -> usize {
        let masked_addr = (addr & PALETE_RAM_MASK) as usize;
        if masked_addr >= 0x10 && masked_addr % 4 == 0 {
            masked_addr - 0x10
        } else {
            masked_addr
        }
    }
}

// It was taken from blargg's power_up_palette test
// The values seems to change from NES to NES
const PALETTE_POWER_UP_STATE: [u8; PALETTE_RAM_SIZE] = [
    0x09, 0x01, 0x00, 0x01, 0x00, 0x02, 0x02, 0x0D, 0x08, 0x10, 0x08, 0x24, 0x00, 0x00, 0x04, 0x2C,
    0x09, 0x01, 0x34, 0x03, 0x00, 0x04, 0x00, 0x14, 0x08, 0x3A, 0x00, 0x02, 0x00, 0x20, 0x2C, 0x08,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut palette_ram = super::PaletteRam::new();
        palette_ram.write(0x00, 0x00);
        palette_ram.write(0x01, 0x15);
        palette_ram.write(0x02, 0x40);
        palette_ram.write(0x03, 0x45);

        assert_eq!(palette_ram.0[0x00], 0x00);
        assert_eq!(palette_ram.0[0x01], 0x15);
        assert_eq!(palette_ram.0[0x02], 0x00);
        assert_eq!(palette_ram.0[0x03], 0x05);

        palette_ram.write(0x10, 0x0A);
        palette_ram.write(0x11, 0x0B);
        assert_eq!(
            palette_ram.0[0], 0x0A,
            "Address 0x10 should be mirrored to 0x00"
        );
        assert_eq!(palette_ram.0[0x11], 0x0B);
    }

    #[test]
    fn test_read() {
        let mut palette_ram = super::PaletteRam::new();
        (0..PALETTE_RAM_SIZE).for_each(|i| palette_ram.0[i] = i as u8);

        assert_eq!(palette_ram.read(0x00), 0x00);
        assert_eq!(palette_ram.read(0x01), 0x01);
        assert_eq!(palette_ram.read(0x04), 0x04);
        assert_eq!(
            palette_ram.read(0x10),
            0x00,
            "Address 0x10 should be mirrored to 0x00"
        );
        assert_eq!(palette_ram.read(0x11), 0x11);
    }
}
