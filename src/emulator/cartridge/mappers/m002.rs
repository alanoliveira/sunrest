use super::*;

pub struct Mapper002 {
    lo_prg_bank: Bank<0x4000>,
    hi_prg_bank: Bank<0x4000>,
    mirror_mode: MirrorMode,
}

impl Mapper002 {
    pub fn new(info: CartridgeInfo) -> Self {
        Self {
            lo_prg_bank: Bank(0),
            hi_prg_bank: Bank(info.prg_banks - 1),
            mirror_mode: info.mirror_mode,
        }
    }
}

impl Mapper for Mapper002 {
    fn configure(&mut self, _addr: u16, val: u8) {
        self.lo_prg_bank.select(val as usize)
    }

    fn prg_addr(&self, addr: u16) -> usize {
        if addr >= 0x4000 {
            self.hi_prg_bank.resolve_address(addr)
        } else {
            self.lo_prg_bank.resolve_address(addr)
        }
    }

    fn chr_addr(&self, addr: u16) -> usize {
        addr as usize
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
        let mut mapper = Mapper002::new(mk_info(3, MirrorMode::Horizontal));
        assert_eq!(mapper.prg_addr(0x4000), 0x8000);
        assert_eq!(mapper.prg_addr(0x7FFF), 0xBFFF);
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x3FFF), 0x3FFF);
        mapper.configure(1, 1);
        assert_eq!(mapper.prg_addr(0x0000), 0x4000);
        assert_eq!(mapper.prg_addr(0x3FFF), 0x7FFF);
    }

    #[test]
    fn test_chr_addr() {
        let mapper = Mapper002::new(mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.chr_addr(0x0000), 0x0000);
        assert_eq!(mapper.chr_addr(0x1000), 0x1000);
        assert_eq!(mapper.chr_addr(0x1FFF), 0x1FFF);
    }

    #[test]
    fn test_mirror_mode() {
        let mapper = Mapper002::new(mk_info(1, MirrorMode::Horizontal));
        assert_eq!(mapper.mirror_mode(), MirrorMode::Horizontal);

        let mapper = Mapper002::new(mk_info(1, MirrorMode::Vertical));
        assert_eq!(mapper.mirror_mode(), MirrorMode::Vertical);
    }
}
