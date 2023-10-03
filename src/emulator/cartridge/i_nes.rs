use super::*;

const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;
const TRAINER_SIZE: usize = 0x200;
const HEADER_SIZE: usize = 16;

struct Flags6 {
    mirroring: Mirroring,
    has_persistent_memory: bool,
    has_trainer: bool,
    ignore_mirroring: bool,
    mapper_lo: u8,
}

impl From<u8> for Flags6 {
    fn from(value: u8) -> Self {
        Self {
            mirroring: if value & 0b0000_0001 != 0 {
                Mirroring::Vertical
            } else {
                Mirroring::Horizontal
            },
            has_persistent_memory: value & 0b0000_0010 != 0,
            has_trainer: value & 0b0000_0100 != 0,
            ignore_mirroring: value & 0b0000_1000 != 0,
            mapper_lo: (value & 0b1111_0000) >> 4,
        }
    }
}

struct Flags7 {
    mapper_hi: u8,
}

impl From<u8> for Flags7 {
    fn from(value: u8) -> Self {
        Self {
            mapper_hi: (value & 0b1111_0000) >> 4,
        }
    }
}

pub struct INesRomBuilder;

impl INesRomBuilder {
    pub fn build(data: &[u8]) -> Cartridge {
        if data[0..=3].ne(&[0x4E, 0x45, 0x53, 0x1A]) {
            panic!("Invalid iNES header");
        }

        let prg_banks = data[4] as usize;
        let chr_banks = data[5] as usize;
        let flags6 = Flags6::from(data[6]);
        let flags7 = Flags7::from(data[7]);

        let mut data_iter = if flags6.has_trainer {
            data.iter().skip(TRAINER_SIZE)
        } else {
            data.iter().skip(16)
        };
        let mapper_code = (flags7.mapper_hi << 4) | flags6.mapper_lo;

        log!("PRG ROM banks: {}", prg_banks);
        log!("CHR ROM banks: {}", chr_banks);
        log!("Hardware Mirroring: {:?}", flags6.mirroring);
        log!("Persistent memory: {}", flags6.has_persistent_memory);
        log!("Mapper: {}", mapper_code);

        let prg_size = prg_banks * PRG_ROM_PAGE_SIZE;
        let prg_data = data_iter.by_ref().take(prg_size).copied().collect();

        let chr_size = chr_banks * CHR_ROM_PAGE_SIZE;
        let chr_data = data_iter.by_ref().take(chr_size).copied().collect();

        if mapper_code != 0 {
            panic!("Mapper {} not supported", mapper_code)
        }

        Cartridge {
            prg_data,
            chr_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartridge_build() {
        let mut data = vec![0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x00, 0x00];
        (0..HEADER_SIZE - data.len()).for_each(|_| data.push(0xFF));
        (0..PRG_ROM_PAGE_SIZE * 2).for_each(|_| data.push(0x42));
        (0..CHR_ROM_PAGE_SIZE).for_each(|_| data.push(0x42));
        let cartridge = INesRomBuilder::build(&data);

        assert_eq!(cartridge.prg_data.len(), PRG_ROM_PAGE_SIZE * 2);
        assert_eq!(cartridge.chr_data.len(), CHR_ROM_PAGE_SIZE);
        assert_eq!(cartridge.prg_data[0], 0x42);
    }

    #[test]
    fn test_skip_trainer() {
        let mut data = vec![0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x04, 0x00];
        (0..TRAINER_SIZE - data.len()).for_each(|_| data.push(0xFF));
        data.push(0x42);

        let cartridge = INesRomBuilder::build(&data);
        assert_eq!(cartridge.prg_data[0], 0x42);
    }

    #[test]
    #[should_panic]
    fn test_invalid_header() {
        let data = [0x4E, 0x45, 0x53, 0x1B, 0x02, 0x01, 0x00, 0x00];
        INesRomBuilder::build(&data);
    }
}
