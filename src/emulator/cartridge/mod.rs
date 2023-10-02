mod i_nes;

pub mod mappers;

const CHR_RAM_SIZE: usize = 0x2000;

#[derive(Debug, Default, Copy, Clone)]
pub struct CartridgeInfo {
    pub mapper_code: u8,
    pub prg_banks: usize,
    pub chr_banks: usize,
    pub mirror_mode: MirrorMode,
    pub has_persistent_memory: bool,
    pub has_trainer: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MirrorMode {
    #[default]
    Horizontal,
    Vertical,
    SingleScreen0,
    SingleScreen1,
}

pub struct Cartridge {
    prg_data: Vec<u8>,
    chr_data: Vec<u8>,
    chr_ram: Box<[u8; CHR_RAM_SIZE]>,
    mapper: Box<dyn mappers::Mapper>,
}

impl Cartridge {
    pub fn new(info: CartridgeInfo, prg_data: &[u8], chr_data: &[u8]) -> Self {
        Self {
            prg_data: prg_data.to_vec(),
            chr_data: chr_data.to_vec(),
            chr_ram: Box::new([0; CHR_RAM_SIZE]),
            mapper: mappers::build(info),
        }
    }

    pub fn read_prg(&self, addr: u16) -> u8 {
        self.prg_data[self.mapper.prg_addr(addr)]
    }

    pub fn read_chr(&self, addr: u16) -> u8 {
        if self.chr_data.len() == 0 {
            self.chr_ram[addr as usize]
        } else {
            let addr = self.mapper.chr_addr(addr);
            self.chr_data[addr]
        }
    }

    pub fn write_prg(&mut self, addr: u16, val: u8) {
        self.mapper.configure(addr, val);
    }

    pub fn write_chr(&mut self, addr: u16, val: u8) {
        self.chr_ram[addr as usize] = val;
    }

    pub fn mirror_mode(&self) -> MirrorMode {
        self.mapper.mirror_mode()
    }

    pub fn take_irq(&mut self) -> bool {
        self.mapper.take_irq()
    }
}

pub fn open_rom(path: &std::path::Path) -> Cartridge {
    log!("Loading ROM file: {:?}", path);
    let rom_data = std::fs::read(path).expect("Failed to read ROM file.");

    i_nes::INesRomBuilder::build(&rom_data)
}
