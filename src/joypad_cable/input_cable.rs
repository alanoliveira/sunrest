use super::JoypadCable;
use std::io::Read;

pub struct InputCable<C: JoypadCable, R: Read> {
    cable: C,
    reader: R,
}

impl<C: JoypadCable, R: Read> InputCable<C, R> {
    pub fn new(cable: C, reader: R) -> Self {
        Self { cable, reader }
    }
}

impl<C: JoypadCable, R: Read> JoypadCable for InputCable<C, R> {
    fn write(&mut self, state: u8) {
        let mut buf = [0];
        if self.reader.read_exact(&mut buf).is_err() {
            buf[0] = state;
        }
        self.cable.write(buf[0]);
    }
}
