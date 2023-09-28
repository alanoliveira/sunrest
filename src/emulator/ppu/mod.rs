mod background;
mod debugger;
mod io;
mod pixel;
mod registers;

pub mod bus;

use pixel::{Kind as PixelKind, Pixel};

const DOTS_PER_LINE: usize = 341;
const LINES_PER_FRAME: usize = 262;

pub struct Ppu {
    pub bus: bus::Bus,
    regs: registers::Registers,
    background: background::Background,

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
            background: background::Background::new(),

            color_idx: 0,
            dot: 0,
            scanline: 0,
            frame: 0,
            cycle: 0,
        }
    }

    pub fn debugger(&self) -> debugger::Debugger {
        debugger::Debugger(self)
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
        if (self.dot > 0 && self.dot <= 256) || (self.dot > 320 && self.dot <= 336) {
            let x = self.dot - 1;
            let shift = self.regs.scroll.x.fine();
            let bg_pixel = self.background.next_pixel(shift as usize);

            let show_bg = self.regs.show_bg && (!self.regs.clip_bg || x > 7);
            let pixel = if bg_pixel.is_visible() && show_bg {
                bg_pixel
            } else {
                Pixel::default()
            };
            self.color_idx = self.bus.read_palette(pixel.address()) as usize;
        }

        self.rendering_preparation();
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

        if matches!(self.dot, 1..=256 | 321..=336) {
            match self.dot % 8 {
                0 => {
                    self.background.load();
                    self.regs.vram_addr.increment_x();
                }
                1 => {
                    let tile_addr = self.regs.vram_addr.tile();
                    self.background.tmp_tile_idx = self.bus.read_nametable(tile_addr)
                }
                3 => {
                    let attribute = self.bus.read_attribute(self.regs.vram_addr.attribute());
                    let palette_shift = self.regs.vram_addr.palette_shift();
                    self.background.tmp_palette = (attribute >> palette_shift) & 0x03;
                }
                num @ (5 | 7) => {
                    let addr = (self.regs.bg_pattern_table as u16) << 12
                        | (self.background.tmp_tile_idx as u16) << 4
                        | (self.regs.vram_addr.fine_y() as u16);
                    if num == 5 {
                        // the byte containing the lo bits is stored first
                        self.background.tmp_pattern_lo = self.bus.read(addr);
                    } else {
                        self.background.tmp_pattern_hi = self.bus.read(addr + 8);
                    }
                }
                _ => {}
            }
        }

        match self.dot {
            256 => self.regs.vram_addr.increment_y(),
            257 => self.regs.update_vram_address_x(),
            _ => (),
        }
    }
}
