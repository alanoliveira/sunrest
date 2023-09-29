mod standard_joypad;

pub use standard_joypad::StandardJoypad;

pub trait InputDevice {
    fn read(&mut self) -> u8;
    fn write(&mut self, val: u8);
}
