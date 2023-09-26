pub mod bus;

const DOTS_PER_LINE: usize = 341;
const LINES_PER_FRAME: usize = 262;

pub struct Ppu {
    pub bus: bus::Bus,

    pub color_idx: usize,
    pub dot: usize,
    pub scanline: usize,
    pub frame: usize,
    pub cycle: usize,
}

impl Ppu {
    pub fn new(bus: bus::Bus) -> Self {
        Self {
            bus,

            color_idx: 0,
            dot: 0,
            scanline: 0,
            frame: 0,
            cycle: 0,
        }
    }

    pub fn clock(&mut self) {
        self.dot += 1;
        if self.dot == DOTS_PER_LINE {
            self.dot = 0;
            self.scanline += 1;
            if self.scanline == LINES_PER_FRAME {
                self.scanline = 0;
                self.frame += 1;
            }
        }

        self.color_idx = (self.dot * self.frame + self.scanline) % 64;

        self.cycle += 1;
    }
}
