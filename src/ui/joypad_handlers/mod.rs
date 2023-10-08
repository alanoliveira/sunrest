use super::*;
mod standard_joypad;
pub use standard_joypad::StandardJoypadHandler;

pub trait JoypadHandler {
    fn set_state(&mut self, val: u8);
}
