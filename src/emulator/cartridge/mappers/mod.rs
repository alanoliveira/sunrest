mod m000;
mod m001;
mod m002;
mod m003;
mod m004;

use super::*;

pub use m000::Mapper000;
pub use m001::Mapper001;
pub use m002::Mapper002;
pub use m003::Mapper003;
pub use m004::Mapper004;

pub trait Mappable {
    fn prg_addr(&self, addr: u16) -> usize;
    fn chr_addr(&self, addr: u16) -> usize;
    fn mirror_mode(&self) -> MirrorMode;
    fn configure(&mut self, addr: u16, val: u8);
    fn take_irq(&mut self) -> bool {
        false
    }
}

#[derive(Clone)]
pub enum Mapper {
    M000(Mapper000),
    M001(Mapper001),
    M002(Mapper002),
    M003(Mapper003),
    M004(Mapper004),
}

/*
 * @TODO: is this the best way to do this?
 * The mapper is decided in runtime. As it must to be clonable/serializable
 * (because of savestates) I can't use a trait object.
 */
impl Mapper {
    pub fn build(info: &CartridgeData) -> Self {
        match info.mapper_code {
            0 => Self::M000(Mapper000::new(info)),
            1 => Self::M001(Mapper001::new(info)),
            2 => Self::M002(Mapper002::new(info)),
            3 => Self::M003(Mapper003::new(info)),
            4 => Self::M004(Mapper004::new(info)),
            _ => panic!("Unsupported mapper: {}", info.mapper_code),
        }
    }

    pub fn as_mut(&mut self) -> &mut dyn Mappable {
        match self {
            Self::M000(m) => m,
            Self::M001(m) => m,
            Self::M002(m) => m,
            Self::M003(m) => m,
            Self::M004(m) => m,
        }
    }

    pub fn as_ref(&self) -> &dyn Mappable {
        match self {
            Self::M000(m) => m,
            Self::M001(m) => m,
            Self::M002(m) => m,
            Self::M003(m) => m,
            Self::M004(m) => m,
        }
    }
}

#[derive(Clone, Copy)]
struct Bank<const SIZE: usize>(usize);

impl<const SIZE: usize> std::fmt::Debug for Bank<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bank")
            .field("bank", &format_args!("{:04X}", self.0))
            .field("SIZE", &format_args!("{:04X}", SIZE))
            .finish()
    }
}

impl<const SIZE: usize> Bank<SIZE> {
    fn select(&mut self, val: usize) {
        self.0 = val;
    }

    fn resolve_address(&self, address: u16) -> usize {
        self.0 * SIZE + (address as usize & (SIZE - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank() {
        let mut bank = Bank::<0x4000>(0);
        assert_eq!(bank.resolve_address(0x0000), 0x0000);
        assert_eq!(bank.resolve_address(0x3fff), 0x3fff);
        assert_eq!(bank.resolve_address(0x4010), 0x0010);
        bank.select(1);
        assert_eq!(bank.resolve_address(0x0000), 0x4000);
        assert_eq!(bank.resolve_address(0x3fff), 0x7fff);
        assert_eq!(bank.resolve_address(0x4010), 0x4010);
    }
}
