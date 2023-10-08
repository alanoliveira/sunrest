use std::{cell, rc};

use super::*;

pub struct StandardJoypadHandler {
    pub joypad: rc::Rc<cell::RefCell<input_devices::StandardJoypad>>,
}

impl StandardJoypadHandler {
    pub fn new(joypad: input_devices::StandardJoypad) -> Self {
        Self {
            joypad: rc::Rc::new(cell::RefCell::new(joypad)),
        }
    }
}

impl Clone for StandardJoypadHandler {
    fn clone(&self) -> Self {
        Self {
            joypad: self.joypad.clone(),
        }
    }
}

impl emulator::InputPort for StandardJoypadHandler {
    fn read(&self) -> u8 {
        self.joypad.borrow_mut().serial_read()
    }

    fn write(&mut self, val: u8) {
        self.joypad.borrow_mut().set_strobe(val & 0x01 != 0);
    }
}

impl JoypadHandler for StandardJoypadHandler {
    fn set_state(&mut self, val: u8) {
        let mut pad = self.joypad.borrow_mut();
        let state: JoypadState = val.into();
        pad.a = state.a;
        pad.b = state.b;
        pad.select = state.select;
        pad.start = state.start;
        pad.up = state.up;
        pad.down = state.down;
        pad.left = state.left;
        pad.right = state.right;
    }
}
