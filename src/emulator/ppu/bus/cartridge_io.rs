pub trait CartridgeIO {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, val: u8);
}
