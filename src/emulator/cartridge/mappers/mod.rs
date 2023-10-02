mod m000;

use super::*;

pub use m000::Mapper000;

pub trait Mapper {
    fn prg_addr(&self, addr: u16) -> usize;
    fn chr_addr(&self, addr: u16) -> usize;
    fn mirror_mode(&self) -> MirrorMode;
}

#[derive(Clone, Copy)]
struct Bank<const SIZE: usize>(usize);

impl<const SIZE: usize> Bank<SIZE> {
    fn select(&mut self, value: usize) {
        self.0 = value;
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
