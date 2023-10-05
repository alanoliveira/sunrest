use super::*;

#[derive(Clone)]
pub struct Mapper003 {
    chr_bank: Bank<0x2000>,
    mirror_mode: MirrorMode,
}

impl Mapper003 {
    pub fn new(info: &CartridgeData) -> Self {
        Self {
            chr_bank: Bank(0),
            mirror_mode: info.mirror_mode,
        }
    }
}

impl Mappable for Mapper003 {
    fn configure(&mut self, _addr: u16, val: u8) {
        self.chr_bank.select(val as usize)
    }

    fn prg_addr(&self, addr: u16) -> usize {
        addr as usize
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

    fn mk_info(chr_banks: usize, mirror_mode: MirrorMode) -> CartridgeData {
        CartridgeData {
            chr_banks,
            mirror_mode,
            ..Default::default()
        }
    }

    #[test]
    fn test_prg_addr() {
        let mapper = Mapper003::new(&mk_info(3, MirrorMode::Horizontal));
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x3FFF), 0x3FFF);
        assert_eq!(mapper.prg_addr(0x8000), 0x8000);
        assert_eq!(mapper.prg_addr(0xBFFF), 0xBFFF);
    }

    #[test]
    fn test_chr_addr() {
        let mut mapper = Mapper003::new(&mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.chr_addr(0x0000), 0x0000);
        assert_eq!(mapper.chr_addr(0x1000), 0x1000);
        assert_eq!(mapper.chr_addr(0x1FFF), 0x1FFF);
        mapper.configure(1, 2);
        assert_eq!(mapper.chr_addr(0x0000), 0x4000);
        assert_eq!(mapper.chr_addr(0x1FFF), 0x5FFF);
    }

    #[test]
    fn test_mirror_mode() {
        let mapper = Mapper003::new(&mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.mirror_mode(), MirrorMode::Horizontal);

        let mapper = Mapper003::new(&mk_info(1, MirrorMode::Vertical));
        assert_eq!(mapper.mirror_mode(), MirrorMode::Vertical);
    }
}
