pub mod bus;

const LAST_DOT: usize = 340;
const LAST_SCANLINE: usize = 261;

pub struct Ppu {
    bus: bus::Bus,
    dot: usize,
    scanline: usize,
    frame: usize,
    cycle: usize,
}

impl Ppu {
    pub fn new(bus: bus::Bus) -> Self {
        Self {
            bus,
            dot: 0,
            scanline: 0,
            frame: 0,
            cycle: 0,
        }
    }

    pub fn clock(&mut self) {

    }
}
