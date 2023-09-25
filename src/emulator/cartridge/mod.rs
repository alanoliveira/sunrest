mod i_nes;

#[derive(Debug, Clone, Copy)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

pub struct Cartridge {
    prg_data: Vec<u8>,
    chr_data: Vec<u8>,
}

impl Cartridge {
    pub fn read_prg(&self, addr: u16) -> u8 {
        let len = self.prg_data.len();
        self.prg_data[(addr as usize) % len]
    }

    pub fn read_chr(&self, addr: u16) -> u8 {
        let len = self.chr_data.len();
        self.chr_data[(addr as usize) % len]
    }
}

pub fn open_rom(path: &std::path::Path) -> Cartridge {
    log!("Loading ROM file: {:?}", path);
    let rom_data = std::fs::read(path).expect("Failed to read ROM file.");

    i_nes::INesRomBuilder::build(&rom_data)
}
