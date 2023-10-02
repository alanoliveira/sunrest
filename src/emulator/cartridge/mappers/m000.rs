use super::*;

pub struct Mapper000 {
    prg_bank1: Bank<0x4000>,
    prg_bank2: Bank<0x4000>,
    chr_bank: Bank<0x2000>,
    mirror_mode: MirrorMode,
}

impl Mapper000 {
    pub fn new(info: CartridgeInfo) -> Self {
        assert!(
            info.prg_banks > 0 && info.prg_banks <= 2,
            "Invalid number of PRG banks for mapper 000",
        );

        Self {
            prg_bank1: Bank(0),
            prg_bank2: Bank(info.prg_banks - 1),
            chr_bank: Bank(0),
            mirror_mode: info.mirror_mode,
        }
    }
}

impl Mapper for Mapper000 {
    fn configure(&mut self, _addr: u16, _val: u8) {}

    fn prg_addr(&self, addr: u16) -> usize {
        if addr < 0x4000 {
            self.prg_bank1.resolve_address(addr)
        } else {
            self.prg_bank2.resolve_address(addr)
        }
    }

    fn chr_addr(&self, addr: u16) -> usize {
        self.chr_bank.resolve_address(addr)
    }

    fn mirror_mode(&self) -> MirrorMode {
        self.mirror_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_info(prg_banks: usize, mirror_mode: MirrorMode) -> CartridgeInfo {
        CartridgeInfo {
            prg_banks,
            mirror_mode,
            ..Default::default()
        }
    }

    #[test]
    fn test_prg_addr() {
        let mapper = Mapper000::new(mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x2000), 0x2000);
        assert_eq!(mapper.prg_addr(0x4000), 0x0000);
        assert_eq!(mapper.prg_addr(0x7FFF), 0x3FFF);

        let mapper = Mapper000::new(mk_info(2, MirrorMode::Horizontal));
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x2000), 0x2000);
        assert_eq!(mapper.prg_addr(0x4000), 0x4000);
        assert_eq!(mapper.prg_addr(0x7FFF), 0x7FFF);
    }

    #[test]
    #[should_panic]
    fn test_prg_bank_validation() {
        Mapper000::new(mk_info(0, MirrorMode::Horizontal));
    }

    #[test]
    fn test_chr_addr() {
        let mapper = Mapper000::new(mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.chr_addr(0x0000), 0x0000);
        assert_eq!(mapper.chr_addr(0x1000), 0x1000);
        assert_eq!(mapper.chr_addr(0x1FFF), 0x1FFF);
    }

    #[test]
    fn test_mirror_mode() {
        let mapper = Mapper000::new(mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.mirror_mode(), MirrorMode::Horizontal);

        let mapper = Mapper000::new(mk_info(1, MirrorMode::Vertical));
        assert_eq!(mapper.mirror_mode(), MirrorMode::Vertical);
    }
}
