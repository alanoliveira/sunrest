mod io;
mod vblank;

pub mod bus;
pub mod tests;

const DOTS_PER_LINE: usize = 341;
const LINES_PER_FRAME: usize = 262;

pub struct Ppu {
    pub bus: bus::Bus,

    pub vblank: vblank::VblankHandler,

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

            vblank: vblank::VblankHandler::default(),

            color_idx: 0,
            dot: 0,
            scanline: 0,
            frame: 0,
            cycle: 0,
        }
    }

    pub fn take_nmi(&mut self) -> bool {
        self.vblank.take_nmi()
    }

    pub fn io(&mut self) -> io::IO {
        io::IO::new(self)
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

        match self.scanline {
            241 => self.vblank_start_line(),
            261 => self.pre_render_line(),
            _ => (),
        }

        self.color_idx = (self.dot * self.frame + self.scanline) % 64;

        self.cycle += 1;
    }

    fn vblank_start_line(&mut self) {
        if self.dot == 1 {
            self.vblank.start();
        }
    }

    fn pre_render_line(&mut self) {
        if self.dot == 1 {
            self.vblank.stop();
        }
    }
}
