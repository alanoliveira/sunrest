pub trait CartridgeIO {
    fn read(&self, addr: u16) -> u8;
}
