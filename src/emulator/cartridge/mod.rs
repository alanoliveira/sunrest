mod i_nes;

pub mod mappers;

const CHR_RAM_SIZE: usize = 0x2000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorMode {
    Horizontal,
    Vertical,
    SingleScreen0,
    SingleScreen1,
}

pub struct Cartridge {
    mapper: Box<dyn mappers::Mapper>,
    prg_data: Vec<u8>,
    chr_data: Vec<u8>,
    chr_ram: Box<[u8; CHR_RAM_SIZE]>,
}

impl Cartridge {
    pub fn new(mapper: Box<dyn mappers::Mapper>, prg_data: &[u8], chr_data: &[u8]) -> Self {
        Self {
            mapper,
            prg_data: prg_data.to_vec(),
            chr_data: chr_data.to_vec(),
            chr_ram: Box::new([0; CHR_RAM_SIZE]),
        }
    }

    pub fn read_prg(&self, addr: u16) -> u8 {
        self.prg_data[self.mapper.prg_addr(addr)]
    }

    pub fn read_chr(&self, addr: u16) -> u8 {
        let addr = self.mapper.chr_addr(addr);
        if self.chr_data.len() == 0 {
            self.chr_ram[addr]
        } else {
            self.chr_data[addr]
        }
    }

    pub fn write_chr(&mut self, addr: u16, value: u8) {
        self.chr_ram[addr as usize] = value;
    }

    pub fn mirror_mode(&self) -> MirrorMode {
        self.mapper.mirror_mode()
    }
}

pub fn open_rom(path: &std::path::Path) -> Cartridge {
    log!("Loading ROM file: {:?}", path);
    let rom_data = std::fs::read(path).expect("Failed to read ROM file.");

    i_nes::INesRomBuilder::build(&rom_data)
}
