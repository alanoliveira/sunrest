use super::*;

#[derive(Clone)]
pub struct Mapper004 {
    mirror_mode: MirrorMode,
    prg_mode: PrgBankMode,
    chr_inversion: bool,
    selected_reg: usize,
    registers: [usize; 8],

    irq: std::cell::RefCell<Irq>,

    prg_ram_protect: bool,
    pgr_ram_enabled: bool,

    prg_banks: [Bank<0x2000>; 4],
    chr_banks: [Bank<0x0400>; 8],

    last_prg_bank: usize,
}

impl Mapper004 {
    pub fn new(info: &CartridgeData) -> Self {
        let mut mapper = Self {
            mirror_mode: MirrorMode::Horizontal,
            prg_mode: PrgBankMode::A,
            chr_inversion: false,
            selected_reg: 0,
            registers: [0; 8],

            irq: std::cell::RefCell::new(Irq::default()),

            prg_ram_protect: false,
            pgr_ram_enabled: false,

            prg_banks: [Bank(0); 4],
            chr_banks: [Bank(0); 8],

            last_prg_bank: (info.prg_banks * 2) - 1,
        };

        mapper.prg_banks[0].select(0);
        mapper.prg_banks[1].select(1);
        mapper.prg_banks[2].select(mapper.last_prg_bank - 1);
        mapper.prg_banks[3].select(mapper.last_prg_bank);
        mapper
    }
}

impl Mappable for Mapper004 {
    fn configure(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => match addr & 1 {
                0 => {
                    self.selected_reg = val as usize & 0x07;
                    self.prg_mode = PrgBankMode::from(val & 0x40 == 0);
                    self.chr_inversion = val & 0x80 != 0;
                }
                _ => {
                    self.registers[self.selected_reg] = val as usize;

                    if self.chr_inversion {
                        self.chr_banks[0].select(self.registers[2]);
                        self.chr_banks[1].select(self.registers[3]);
                        self.chr_banks[2].select(self.registers[4]);
                        self.chr_banks[3].select(self.registers[5]);
                        self.chr_banks[4].select(self.registers[0] & 0xFE);
                        self.chr_banks[5].select(self.registers[0] | 0x01);
                        self.chr_banks[6].select(self.registers[1] & 0xFE);
                        self.chr_banks[7].select(self.registers[1] | 0x01);
                    } else {
                        self.chr_banks[0].select(self.registers[0] & 0xFE);
                        self.chr_banks[1].select(self.registers[0] | 0x01);
                        self.chr_banks[2].select(self.registers[1] & 0xFE);
                        self.chr_banks[3].select(self.registers[1] | 0x01);
                        self.chr_banks[4].select(self.registers[2]);
                        self.chr_banks[5].select(self.registers[3]);
                        self.chr_banks[6].select(self.registers[4]);
                        self.chr_banks[7].select(self.registers[5]);
                    }

                    match self.prg_mode {
                        PrgBankMode::A => {
                            self.prg_banks[0].select(self.registers[6]);
                            self.prg_banks[2].select(self.last_prg_bank - 1);
                        }
                        PrgBankMode::B => {
                            self.prg_banks[0].select(self.last_prg_bank - 1);
                            self.prg_banks[2].select(self.registers[6]);
                        }
                    }
                    self.prg_banks[1].select(self.registers[7]);
                }
            },
            0x2000..=0x3FFF => match addr & 1 {
                0 => {
                    self.mirror_mode = if val & 1 == 0 {
                        MirrorMode::Vertical
                    } else {
                        MirrorMode::Horizontal
                    }
                }
                _ => {
                    self.prg_ram_protect = val & 0x40 == 0;
                    self.pgr_ram_enabled = val & 0x80 == 0;
                }
            },
            0x4000..=0x5FFF => match addr & 1 {
                0 => self.irq.borrow_mut().latch = val,
                _ => self.irq.borrow_mut().counter = 0,
            },
            0x6000..=0x7FFF => self.irq.borrow_mut().enabled = (addr & 0x0001) == 1,
            _ => unreachable!(),
        }
    }

    fn prg_addr(&self, addr: u16) -> usize {
        match addr {
            0x0000..=0x1FFF => self.prg_banks[0],
            0x2000..=0x3FFF => self.prg_banks[1],
            0x4000..=0x5FFF => self.prg_banks[2],
            0x6000..=0x7FFF => self.prg_banks[3],
            _ => unreachable!(),
        }
        .resolve_address(addr)
    }

    fn chr_addr(&self, addr: u16) -> usize {
        self.irq.borrow_mut().register_a12_state(addr);

        match addr {
            0x0000..=0x03FF => self.chr_banks[0],
            0x0400..=0x07FF => self.chr_banks[1],
            0x0800..=0x0BFF => self.chr_banks[2],
            0x0C00..=0x0FFF => self.chr_banks[3],
            0x1000..=0x13FF => self.chr_banks[4],
            0x1400..=0x17FF => self.chr_banks[5],
            0x1800..=0x1BFF => self.chr_banks[6],
            0x1C00..=0x1FFF => self.chr_banks[7],
            _ => unreachable!(),
        }
        .resolve_address(addr)
    }

    fn mirror_mode(&self) -> MirrorMode {
        self.mirror_mode
    }

    fn take_irq(&mut self) -> bool {
        self.irq.borrow_mut().irq.take().is_some()
    }
}

#[derive(Clone)]
enum PrgBankMode {
    A,
    B,
}

impl From<bool> for PrgBankMode {
    fn from(value: bool) -> Self {
        if value {
            Self::A
        } else {
            Self::B
        }
    }
}

#[derive(Default, Clone)]
struct Irq {
    pub a12_state: bool,
    pub enabled: bool,
    pub counter: u8,
    pub latch: u8,
    pub irq: Option<()>,
}

impl Irq {
    pub fn register_a12_state(&mut self, addr: u16) {
        let a12_state = addr & 0x1000 != 0;
        if !self.a12_state && a12_state {
            self.step_scanline();
        }
        self.a12_state = a12_state;
    }

    fn step_scanline(&mut self) {
        if self.counter == 0 {
            self.counter = self.latch;
        } else {
            self.counter -= 1;
            if self.counter == 0 && self.enabled {
                self.irq = Some(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_info() -> CartridgeData {
        CartridgeData {
            chr_banks: 15,
            prg_banks: 5,
            ..Default::default()
        }
    }

    #[test]
    fn test_irq() {
        let mut mapper = Mapper004::new(&mk_info());
        mapper.configure(0x6001, 0); // enable IRQ
        mapper.configure(0x4000, 4); // set latch
        mapper.configure(0x4001, 0); // clear counter
        for _ in 0..=4 {
            assert!(!mapper.take_irq());
            mapper.chr_addr(0x0000);
            mapper.chr_addr(0x1000);
        }
        assert!(mapper.take_irq());

        for _ in 0..=3 {
            assert!(!mapper.take_irq());
            mapper.chr_addr(0x0000);
            mapper.chr_addr(0x1000);
        }
        mapper.configure(0x4001, 0); // clear counter
        mapper.chr_addr(0x0000);
        mapper.chr_addr(0x1000);
        assert!(!mapper.take_irq());

        mapper.configure(0x6000, 0); // disable IRQ
        for _ in 0..=4 {
            assert!(!mapper.take_irq());
            mapper.chr_addr(0x0000);
            mapper.chr_addr(0x1000);
        }
        assert!(!mapper.take_irq());
    }

    #[test]
    fn test_mirror_mode() {
        let mut mapper = Mapper004::new(&mk_info());
        assert_eq!(mapper.mirror_mode(), MirrorMode::Horizontal);
        mapper.configure(0x2000, 0);
        assert_eq!(mapper.mirror_mode(), MirrorMode::Vertical);
        mapper.configure(0x2000, 1);
        assert_eq!(mapper.mirror_mode(), MirrorMode::Horizontal);
    }

    #[test]
    fn test_prg_addr() {
        let mut mapper = Mapper004::new(&mk_info());
        assert_eq!(mapper.prg_addr(0x0000), 0x0000);
        assert_eq!(mapper.prg_addr(0x2000), 0x2000);

        /*
         * 0x0000..=0x1FFF => bank 0
         * 0x2000..=0x3FFF => bank 1
         * 0x4000..=0x5FFF => bank 2
         * 0x6000..=0x7FFF => bank 3
         *
         * mode A:
         * bank 0 => register 6
         * bank 1 => register 7
         * bank 2 => second last bank
         * bank 3 => last bank
         *
         * mode B:
         * bank 0 => second last bank
         * bank 1 => register 7
         * bank 2 => register 6
         * bank 3 => last bank
         */
        mapper.configure(0x0000, 0b0000_0110); // set selected_reg to 6 mode A
        mapper.configure(0x0001, 0b0000_0001); // set registers[6] to 1
        mapper.configure(0x0000, 0b0000_0111); // set selected_reg to 7 mode A
        mapper.configure(0x0001, 0b0000_0100); // set registers[7] to 4
        assert_eq!(mapper.prg_addr(0x0000), 0x2000);
        assert_eq!(mapper.prg_addr(0x2000), 0x8000);
        assert_eq!(mapper.prg_addr(0x4000), 0x10000);
        assert_eq!(mapper.prg_addr(0x6000), 0x12000);

        mapper.configure(0x0000, 0b0100_0110); // set selected_reg to 6 mode B
        mapper.configure(0x0001, 0b0000_0001); // set registers[6] to 1
        mapper.configure(0x0000, 0b0100_0111); // set selected_reg to 7 mode B
        mapper.configure(0x0001, 0b0000_0011); // set registers[7] to 3
        assert_eq!(mapper.prg_addr(0x0000), 0x10000);
        assert_eq!(mapper.prg_addr(0x2000), 0x6000);
        assert_eq!(mapper.prg_addr(0x4000), 0x2000);
        assert_eq!(mapper.prg_addr(0x6000), 0x12000);
    }

    #[test]
    fn test_chr_addr() {
        let mut mapper = Mapper004::new(&mk_info());
        assert_eq!(mapper.chr_addr(0x0000), 0x0000);
        assert_eq!(mapper.chr_addr(0x0600), 0x0200);

        /*
         * 0x0000..=0x03FF => bank 0
         * 0x0400..=0x07FF => bank 1
         * 0x0800..=0x0BFF => bank 2
         * 0x0C00..=0x0FFF => bank 3
         * 0x1000..=0x13FF => bank 4
         * 0x1400..=0x17FF => bank 5
         * 0x1800..=0x1BFF => bank 6
         * 0x1C00..=0x1FFF => bank 7
         *
         * chr_inversion off
         * bank 0 => register 0 & 0xFE
         * bank 1 => register 0 | 0x01
         * bank 2 => register 1 & 0xFE
         * bank 3 => register 1 | 0x01
         * bank 4 => register 2
         * bank 5 => register 3
         * bank 6 => register 4
         * bank 7 => register 5
         *
         * chr_inversion on
         * bank 0 => register 2
         * bank 1 => register 3
         * bank 2 => register 4
         * bank 3 => register 5
         * bank 4 => register 0 & 0xFE
         * bank 5 => register 0 | 0x01
         * bank 6 => register 1 & 0xFE
         * bank 7 => register 1 | 0x01
         */

        mapper.configure(0x0000, 0b0000_0000); // set selected_reg to 0 invert off
        mapper.configure(0x0001, 0b0000_0011); // set registers[0] to 3 (b0 = 0b10 and b1 = 0b11)
        mapper.configure(0x0000, 0b0000_0001); // set selected_reg to 1 invert off
        mapper.configure(0x0001, 0b0000_0101); // set registers[1] to 5 (b2 = 0b100 and b3 = 0b101)
        mapper.configure(0x0000, 0b0000_0010); // set selected_reg to 2 invert off
        mapper.configure(0x0001, 0b0000_0111); // set registers[2] to 7 (b4 = 0b111)
        mapper.configure(0x0000, 0b0000_0011); // set selected_reg to 3 invert off
        mapper.configure(0x0001, 0b0000_1001); // set registers[3] to 9 (b5 = 0b1001)
        mapper.configure(0x0000, 0b0000_0100); // set selected_reg to 4 invert off
        mapper.configure(0x0001, 0b0000_1011); // set registers[4] to 11 (b6 = 0b1011)
        mapper.configure(0x0000, 0b0000_0101); // set selected_reg to 5 invert off
        mapper.configure(0x0001, 0b0000_0000); // set registers[5] to 13 (b7 = 0b0)
        assert_eq!(mapper.chr_addr(0x0000), 0x0800);
        assert_eq!(mapper.chr_addr(0x0400), 0x0C00);
        assert_eq!(mapper.chr_addr(0x0800), 0x1000);
        assert_eq!(mapper.chr_addr(0x0C00), 0x1400);
        assert_eq!(mapper.chr_addr(0x1000), 0x1C00);
        assert_eq!(mapper.chr_addr(0x1400), 0x2400);
        assert_eq!(mapper.chr_addr(0x1800), 0x2C00);
        assert_eq!(mapper.chr_addr(0x1C00), 0x0000);

        mapper.configure(0x0000, 0b1000_0000); // set selected_reg to 0 invert off
        mapper.configure(0x0001, 0b0000_0011); // set registers[0] to 3 (b0 = 0b10 and b1 = 0b11)
        mapper.configure(0x0000, 0b1000_0001); // set selected_reg to 1 invert off
        mapper.configure(0x0001, 0b0000_0101); // set registers[1] to 5 (b2 = 0b100 and b3 = 0b101)
        mapper.configure(0x0000, 0b1000_0010); // set selected_reg to 2 invert off
        mapper.configure(0x0001, 0b0000_0111); // set registers[2] to 7 (b4 = 0b111)
        mapper.configure(0x0000, 0b1000_0011); // set selected_reg to 3 invert off
        mapper.configure(0x0001, 0b0000_1001); // set registers[3] to 9 (b5 = 0b1001)
        mapper.configure(0x0000, 0b1000_0100); // set selected_reg to 4 invert off
        mapper.configure(0x0001, 0b0000_1011); // set registers[4] to 11 (b6 = 0b1011)
        mapper.configure(0x0000, 0b1000_0101); // set selected_reg to 5 invert off
        mapper.configure(0x0001, 0b0000_0000); // set registers[5] to 13 (b7 = 0b0)
        assert_eq!(mapper.chr_addr(0x0000), 0x1C00);
        assert_eq!(mapper.chr_addr(0x0400), 0x2400);
        assert_eq!(mapper.chr_addr(0x0800), 0x2C00);
        assert_eq!(mapper.chr_addr(0x0C00), 0x0000);
        assert_eq!(mapper.chr_addr(0x1000), 0x0800);
        assert_eq!(mapper.chr_addr(0x1400), 0x0C00);
        assert_eq!(mapper.chr_addr(0x1800), 0x1000);
        assert_eq!(mapper.chr_addr(0x1C00), 0x1400);
    }
}
