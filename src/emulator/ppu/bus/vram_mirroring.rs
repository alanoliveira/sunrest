use crate::emulator::cartridge::MirrorMode;

const VRAM_MASK: u16 = 0x03FF;
const NAME_TABLE_SIZE: u16 = 0x0400;

pub trait Mirroring {
    fn mirror(&self, addr: u16) -> u16;
}

impl Mirroring for MirrorMode {
    fn mirror(&self, addr: u16) -> u16 {
        let masked_addr = addr & VRAM_MASK;
        match self {
            MirrorMode::Horizontal => masked_addr | ((addr >> 1) & NAME_TABLE_SIZE),
            MirrorMode::Vertical => masked_addr | (addr & NAME_TABLE_SIZE),
            MirrorMode::SingleScreen0 => masked_addr,
            MirrorMode::SingleScreen1 => masked_addr + NAME_TABLE_SIZE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirroring() {
        assert_eq!(MirrorMode::Horizontal.mirror(0x0000), 0x0000);
        assert_eq!(MirrorMode::Horizontal.mirror(0x0400), 0x0000);
        assert_eq!(MirrorMode::Horizontal.mirror(0x0800), 0x0400);
        assert_eq!(MirrorMode::Horizontal.mirror(0x0C00), 0x0400);

        assert_eq!(MirrorMode::Vertical.mirror(0x0000), 0x0000);
        assert_eq!(MirrorMode::Vertical.mirror(0x0400), 0x0400);
        assert_eq!(MirrorMode::Vertical.mirror(0x0800), 0x0000);
        assert_eq!(MirrorMode::Vertical.mirror(0x0C00), 0x0400);

        assert_eq!(MirrorMode::SingleScreen0.mirror(0x0000), 0x0000);
        assert_eq!(MirrorMode::SingleScreen0.mirror(0x0400), 0x0000);
        assert_eq!(MirrorMode::SingleScreen0.mirror(0x0800), 0x0000);
        assert_eq!(MirrorMode::SingleScreen0.mirror(0x0C00), 0x0000);

        assert_eq!(MirrorMode::SingleScreen1.mirror(0x0000), 0x0400);
        assert_eq!(MirrorMode::SingleScreen1.mirror(0x0400), 0x0400);
        assert_eq!(MirrorMode::SingleScreen1.mirror(0x0800), 0x0400);
        assert_eq!(MirrorMode::SingleScreen1.mirror(0x0C00), 0x0400);
    }
}
