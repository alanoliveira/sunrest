mod i_nes;

const CHR_RAM_SIZE: usize = 0x2000;

#[derive(Debug, Clone, Copy)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

pub struct Cartridge {
    prg_data: Vec<u8>,
    chr_data: Vec<u8>,
    chr_ram: Box<[u8; CHR_RAM_SIZE]>,
}

impl Cartridge {
    pub fn new(prg_data: &[u8], chr_data: &[u8]) -> Self {
        Self {
            prg_data: prg_data.to_vec(),
            chr_data: chr_data.to_vec(),
            chr_ram: Box::new([0; CHR_RAM_SIZE]),
        }
    }

    pub fn read_prg(&self, addr: u16) -> u8 {
        let len = self.prg_data.len();
        self.prg_data[(addr as usize) % len]
    }

    pub fn read_chr(&self, addr: u16) -> u8 {
        let len = self.chr_data.len();
        if len == 0 {
            self.chr_ram[addr as usize]
        } else {
            self.chr_data[(addr as usize) % len]
        }
    }

    pub fn write_chr(&mut self, addr: u16, value: u8) {
        self.chr_ram[addr as usize] = value;
    }
}

pub fn open_rom(path: &std::path::Path) -> Cartridge {
    log!("Loading ROM file: {:?}", path);
    let rom_data = std::fs::read(path).expect("Failed to read ROM file.");

    i_nes::INesRomBuilder::build(&rom_data)
}
