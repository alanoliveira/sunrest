mod debugger;
mod io;
mod registers;

pub mod bus;

const DOTS_PER_LINE: usize = 341;
const LINES_PER_FRAME: usize = 262;

pub struct Ppu {
    pub bus: bus::Bus,
    regs: registers::Registers,

    pub color_idx: usize,
    pub dot: usize,
    pub scanline: usize,
    pub frame: usize,
    pub cycle: usize,
}

impl std::fmt::Debug for Ppu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ppu {{ ({:03},{:03}) [{}] }}",
            self.scanline, self.dot, self.frame,
        )
    }
}

impl Ppu {
    pub fn new(bus: bus::Bus) -> Self {
        Self {
            bus,
            regs: registers::Registers::default(),

            color_idx: 0,
            dot: 0,
            scanline: 0,
            frame: 0,
            cycle: 0,
        }
    }

    pub fn take_nmi(&mut self) -> bool {
        self.regs.nmi.take().is_some()
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
            0..=239 => self.visible_line(),
            241 => self.vblank_start_line(),
            261 => self.pre_render_line(),
            _ => (),
        }

        self.cycle += 1;
    }

    fn visible_line(&mut self) {
        self.rendering_preparation();
        self.color_idx = (self.dot * self.frame + self.scanline) % 64;
    }

    fn vblank_start_line(&mut self) {
        if self.dot == 1 {
            self.regs.start_vblank();
        }
    }

    fn pre_render_line(&mut self) {
        self.rendering_preparation();

        match self.dot {
            1 => self.regs.stop_vblank(),
            _ => (),
        }

        if self.regs.render_enabled() && matches!(self.dot, 280..=304) {
            self.regs.update_vram_address_y();
        }
    }

    fn rendering_preparation(&mut self) {
        if !self.regs.render_enabled() {
            return;
        }

        match self.dot {
            1..=256 | 321..=336 if self.dot % 8 == 0 => self.regs.vram_addr.increment_x(),
            256 => self.regs.vram_addr.increment_y(),
            257 => self.regs.update_vram_address_x(),
            _ => (),
        }
    }
}
