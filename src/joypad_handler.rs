use std::{cell, rc};

use super::*;
use input_devices::standard_joypad::StandardJoypad;

pub struct JoypadHandler {
    joypad: rc::Rc<cell::RefCell<StandardJoypad>>,
}

impl JoypadHandler {
    pub fn new() -> Self {
        let joypad = StandardJoypad::new();
        Self {
            joypad: rc::Rc::new(cell::RefCell::new(joypad)),
        }
    }

    fn joypad_mut(&self) -> cell::RefMut<StandardJoypad> {
        self.joypad.try_borrow_mut().unwrap()
    }
}

impl Clone for JoypadHandler {
    fn clone(&self) -> Self {
        Self {
            joypad: self.joypad.clone(),
        }
    }
}

impl emulator::InputPort for JoypadHandler {
    fn read(&self) -> u8 {
        self.joypad_mut().serial_read()
    }

    fn write(&mut self, val: u8) {
        self.joypad_mut().set_strobe(val & 0x01 != 0);
    }
}

impl joypad_cable::JoypadCable for JoypadHandler {
    fn write(&mut self, state: u8) {
        self.joypad_mut().state = state.into();
    }
}
