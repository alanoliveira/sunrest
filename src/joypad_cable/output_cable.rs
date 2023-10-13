use super::JoypadCable;
use std::io::Write;

pub struct OutputCable<C: JoypadCable, W: Write> {
    cable: C,
    writer: W,
}

impl<C: JoypadCable, W: Write> OutputCable<C, W> {
    pub fn new(cable: C, writer: W) -> Self {
        Self { cable, writer }
    }
}

impl<C: JoypadCable, W: Write> JoypadCable for OutputCable<C, W> {
    fn write(&mut self, state: u8) {
        let buf = [state];
        self.cable.write(state);
        self.writer.write_all(&buf).unwrap();
    }
}
