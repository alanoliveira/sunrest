mod i_nes;
mod mappers;
mod time_machine;

pub use time_machine::TimeMachine;

const CHR_RAM_SIZE: usize = 0x2000;

#[derive(Debug, Default, Clone)]
pub struct CartridgeData {
    pub mapper_code: u8,
    pub prg_banks: usize,
    pub chr_banks: usize,
    pub mirror_mode: MirrorMode,
    pub has_persistent_memory: bool,
    pub has_trainer: bool,
    pub prg_data: Vec<u8>,
    pub chr_data: Vec<u8>,
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
    data: CartridgeData,
    chr_ram: Box<[u8; CHR_RAM_SIZE]>,
    mapper: mappers::Mapper,
}

impl Cartridge {
    pub fn new(data: CartridgeData) -> Self {
        let mapper = mappers::Mapper::build(&data);
        Self {
            data,
            chr_ram: Box::new([0; CHR_RAM_SIZE]),
            mapper,
        }
    }

    pub fn read_prg(&self, addr: u16) -> u8 {
        self.data.prg_data[self.mapper.as_ref().prg_addr(addr)]
    }

    pub fn read_chr(&self, addr: u16) -> u8 {
        if self.data.chr_banks == 0 {
            self.chr_ram[addr as usize]
        } else {
            let addr = self.mapper.as_ref().chr_addr(addr);
            self.data.chr_data[addr]
        }
    }

    pub fn write_prg(&mut self, addr: u16, val: u8) {
        self.mapper.as_mut().configure(addr, val);
    }

    pub fn write_chr(&mut self, addr: u16, val: u8) {
        self.chr_ram[addr as usize] = val;
    }

    pub fn mirror_mode(&self) -> MirrorMode {
        self.mapper.as_ref().mirror_mode()
    }

    pub fn take_irq(&mut self) -> bool {
        self.mapper.as_mut().take_irq()
    }
}

pub fn open_rom(path: &std::path::Path) -> Cartridge {
    log!("Loading ROM file: {:?}", path);
    let rom_data = std::fs::read(path).expect("Failed to read ROM file.");

    let cartridge_data = i_nes::INesRomBuilder::build(&rom_data);

    Cartridge::new(cartridge_data)
}
