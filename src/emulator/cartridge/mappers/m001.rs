use super::*;

pub struct Mapper001 {
    info: CartridgeInfo,
    load_register: LoadRegister,
    control_register: ControlRegister,

    prg_bank_16_hi: Bank<0x4000>,
    prg_bank_16_lo: Bank<0x4000>,
    prg_bank_32: Bank<0x8000>,

    chr_bank_4_hi: Bank<0x1000>,
    chr_bank_4_lo: Bank<0x1000>,
    chr_bank_8: Bank<0x2000>,
}

impl Mapper001 {
    pub fn new(info: CartridgeInfo) -> Self {
        Self {
            info,
            load_register: LoadRegister::new(),
            control_register: ControlRegister(0x0C),

            prg_bank_16_hi: Bank(info.prg_banks - 1),
            prg_bank_16_lo: Bank(0),
            prg_bank_32: Bank(0),

            chr_bank_4_hi: Bank(0),
            chr_bank_4_lo: Bank(0),
            chr_bank_8: Bank(0),
        }
    }

    fn configure_control_register(&mut self) {
        self.control_register.write(self.load_register.read() as u8);
    }

    fn configure_char_bank_lo(&mut self) {
        match self.control_register.chr_rom_mode() {
            ChrRomMode::SwitchTwo4KB => self.chr_bank_4_hi.select(self.load_register.read()),
            ChrRomMode::Switch8KB => self.chr_bank_8.select(self.load_register.read()),
        }
    }

    fn configure_char_bank_hi(&mut self) {
        if let ChrRomMode::SwitchTwo4KB = self.control_register.chr_rom_mode() {
            self.chr_bank_4_lo.select(self.load_register.read())
        }
    }

    fn configure_prg_bank(&mut self) {
        match self.control_register.prg_rom_mode() {
            PrgRomMode::Switch32KB => self
                .prg_bank_32
                .select((self.load_register.read() & 0b11110) >> 1),
            PrgRomMode::Switch16KBFixFirst => {
                self.prg_bank_16_hi.select(self.load_register.read());
                self.prg_bank_16_lo.select(0);
            }
            PrgRomMode::Switch16KBFixLast => {
                self.prg_bank_16_hi.select(self.info.prg_banks - 1);
                self.prg_bank_16_lo.select(self.load_register.read());
            }
        }
    }
}

impl Mapper for Mapper001 {
    fn configure(&mut self, addr: u16, val: u8) {
        if val & 0b1000_0000 != 0 {
            self.load_register.reset();
            self.control_register.write(0x0C);
        } else {
            self.load_register.write(val);
        }

        if self.load_register.complete() {
            match addr {
                0x0000..=0x1FFF => self.configure_control_register(),
                0x2000..=0x3FFF => self.configure_char_bank_lo(),
                0x4000..=0x5FFF => self.configure_char_bank_hi(),
                0x6000..=0x7FFF => self.configure_prg_bank(),
                _ => unreachable!(),
            }
        }
    }

    fn prg_addr(&self, addr: u16) -> usize {
        match self.control_register.prg_rom_mode() {
            PrgRomMode::Switch32KB => self.prg_bank_32.resolve_address(addr),
            _ => match addr {
                0x0000..=0x3FFF => self.prg_bank_16_lo.resolve_address(addr),
                0x4000..=0x7FFF => self.prg_bank_16_hi.resolve_address(addr),
                _ => unreachable!(),
            },
        }
    }

    fn chr_addr(&self, addr: u16) -> usize {
        match self.control_register.chr_rom_mode() {
            ChrRomMode::Switch8KB => self.chr_bank_8.resolve_address(addr),
            ChrRomMode::SwitchTwo4KB => match addr {
                0x0000..=0x0FFF => self.chr_bank_4_hi.resolve_address(addr),
                0x1000..=0x1FFF => self.chr_bank_4_lo.resolve_address(addr),
                _ => unreachable!(),
            },
        }
    }

    fn mirror_mode(&self) -> MirrorMode {
        self.control_register.mirror_mode()
    }
}

struct LoadRegister(u8);

impl LoadRegister {
    fn new() -> Self {
        Self(0b0010_0000)
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn read(&self) -> usize {
        ((self.0 as usize) >> 1) & 0b11111
    }

    fn write(&mut self, val: u8) {
        if self.complete() {
            self.reset();
        }

        self.0 >>= 1;
        self.0 |= (val & 1) << 5;
    }

    fn complete(&self) -> bool {
        self.0 & 1 == 1
    }
}

#[derive(Debug)]
enum PrgRomMode {
    Switch32KB,
    Switch16KBFixFirst,
    Switch16KBFixLast,
}

#[derive(Debug)]
enum ChrRomMode {
    Switch8KB,
    SwitchTwo4KB,
}

struct ControlRegister(u8);

impl ControlRegister {
    fn write(&mut self, val: u8) {
        self.0 = val;
    }

    fn mirror_mode(&self) -> MirrorMode {
        match self.0 & 0b11 {
            0 => MirrorMode::SingleScreen0,
            1 => MirrorMode::SingleScreen1,
            2 => MirrorMode::Vertical,
            3 => MirrorMode::Horizontal,
            _ => unreachable!(),
        }
    }

    fn prg_rom_mode(&self) -> PrgRomMode {
        match (self.0 & 0b1100) >> 2 {
            0 | 1 => PrgRomMode::Switch32KB,
            2 => PrgRomMode::Switch16KBFixFirst,
            3 => PrgRomMode::Switch16KBFixLast,
            _ => unreachable!(),
        }
    }

    fn chr_rom_mode(&self) -> ChrRomMode {
        match (self.0 & 0b10000) >> 4 {
            0 => ChrRomMode::Switch8KB,
            1 => ChrRomMode::SwitchTwo4KB,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_info(chr_banks: usize, prg_banks: usize) -> CartridgeInfo {
        CartridgeInfo {
            chr_banks,
            prg_banks,
            ..Default::default()
        }
    }

    macro_rules! conf_reg {
        ($reg:expr, $addr:expr, $val:expr) => {
            for i in 0..5 {
                $reg.configure($addr, $val >> i);
            }
        };
    }

    #[test]
    fn test_prg_addr_switch_32() {
        let mut mapper = Mapper001::new(mk_info(3, 3));
        conf_reg!(mapper, 0x0000, 0b0000);
        conf_reg!(mapper, 0x6000, 0b0000);
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x2000), 0x2000);
        assert_eq!(mapper.prg_addr(0x7FFF), 0x7FFF);
        conf_reg!(mapper, 0x6000, 0b0010);
        assert_eq!(mapper.prg_addr(0x0000), 0x8000);
        conf_reg!(mapper, 0x6000, 0b0100);
        assert_eq!(mapper.prg_addr(0x0000), 0x10000);
    }

    #[test]
    fn test_prg_addr_switch_16_fix_first() {
        let mut mapper = Mapper001::new(mk_info(3, 3));
        conf_reg!(mapper, 0x0000, 0b1000);
        conf_reg!(mapper, 0x6000, 0b0010);
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x4000), 0x8000);
    }

    #[test]
    fn test_prg_addr_switch_16_fix_last() {
        let mut mapper = Mapper001::new(mk_info(3, 3));
        conf_reg!(mapper, 0x0000, 0b1100);
        conf_reg!(mapper, 0x6000, 0b0001);
        assert_eq!(mapper.prg_addr(0x0000), 0x4000);
        assert_eq!(mapper.prg_addr(0x4000), 0x8000);
    }

    #[test]
    fn test_chr_addr_one_8k() {
        let mut mapper = Mapper001::new(mk_info(3, 3));
        conf_reg!(mapper, 0x0000, 0b00000);
        conf_reg!(mapper, 0x2000, 0b0000);
        assert_eq!(mapper.chr_addr(0x0000), 0x0000);
        assert_eq!(mapper.chr_addr(0x1FFF), 0x1FFF);
        conf_reg!(mapper, 0x2000, 0b0010);
        assert_eq!(mapper.chr_addr(0x0000), 0x4000);
        conf_reg!(mapper, 0x4000, 0b0001);
        assert_eq!(
            mapper.chr_addr(0x0000),
            0x4000,
            "register 0x4000 should not affect 8k mode"
        );
    }

    #[test]
    fn test_chr_addr_two_4k() {
        let mut mapper = Mapper001::new(mk_info(3, 3));
        conf_reg!(mapper, 0x0000, 0b10000);
        conf_reg!(mapper, 0x2000, 0b0000);
        conf_reg!(mapper, 0x4000, 0b0011);
        assert_eq!(mapper.chr_addr(0x0000), 0x0000);
        assert_eq!(mapper.chr_addr(0x0800), 0x0800);
        assert_eq!(mapper.chr_addr(0x1000), 0x3000);
        assert_eq!(mapper.chr_addr(0x1800), 0x3800);
        conf_reg!(mapper, 0x2000, 0b0011);
        conf_reg!(mapper, 0x4000, 0b0010);
        assert_eq!(mapper.chr_addr(0x0000), 0x3000);
        assert_eq!(mapper.chr_addr(0x0800), 0x3800);
        assert_eq!(mapper.chr_addr(0x1000), 0x2000);
        assert_eq!(mapper.chr_addr(0x1800), 0x2800);
    }

    #[test]
    fn test_mirror_mode() {
        let mut mapper = Mapper001::new(mk_info(3, 3));
        assert_eq!(mapper.mirror_mode(), MirrorMode::SingleScreen0);
        conf_reg!(mapper, 0x0000, 0b10);
        assert_eq!(mapper.mirror_mode(), MirrorMode::Vertical);
        conf_reg!(mapper, 0x0000, 0b11);
        assert_eq!(mapper.mirror_mode(), MirrorMode::Horizontal);
        conf_reg!(mapper, 0x0000, 0b00);
        assert_eq!(mapper.mirror_mode(), MirrorMode::SingleScreen0);
        conf_reg!(mapper, 0x0000, 0b01);
        assert_eq!(mapper.mirror_mode(), MirrorMode::SingleScreen1);
    }
}
