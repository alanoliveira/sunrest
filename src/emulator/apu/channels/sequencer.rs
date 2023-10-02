#[derive(Debug, Default, Clone, Copy)]
pub struct Sequencer<const LEN: u8>(u8);

impl<const LEN: u8> Sequencer<LEN> {
    pub fn clock(&mut self) {
        self.0 = (self.0 + 1) % LEN;
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}
