use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    chr_ram: Vec<u8>,
    mapper: mappers::Mapper,
}

impl TimeMachine {
    pub fn save(cartridge: &Cartridge) -> Self {
        Self {
            chr_ram: cartridge.chr_ram.clone(),
            mapper: cartridge.mapper.clone(),
        }
    }

    pub fn load(self, cartridge: &mut Cartridge) {
        cartridge.chr_ram = self.chr_ram;
        cartridge.mapper = self.mapper;
    }
}
