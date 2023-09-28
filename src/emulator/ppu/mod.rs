mod background;
mod debugger;
mod foreground;
mod io;
mod oam;
mod pixel;
mod registers;
mod sprite;

pub mod bus;

use pixel::{Kind as PixelKind, Pixel};
use sprite::RawSprite;

const DOTS_PER_LINE: usize = 341;
const LINES_PER_FRAME: usize = 262;
const OAM_SIZE: usize = 0x100;
const MAX_VISIBLE_SPRITES: usize = 8;

pub struct Ppu {
    pub bus: bus::Bus,
    oam: oam::Oam,
    sprites: Vec<RawSprite>,
    regs: registers::Registers,
    background: background::Background,
    foreground: foreground::Foreground,

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
            oam: oam::Oam::new(),
            sprites: Vec::with_capacity(MAX_VISIBLE_SPRITES),
            regs: registers::Registers::default(),
            background: background::Background::new(),
            foreground: foreground::Foreground::new(),

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
        if self.dot > 0 && self.dot <= 256 {
            let bg_pixel = self
                .background
                .next_pixel(self.regs.scroll.x.fine() as usize);
            let spr_pixel = self.foreground.next_pixel().unwrap_or_default();

            let x = self.dot - 1;
            let show_bg =
                self.regs.show_bg && (!self.regs.clip_bg || x > 7) && bg_pixel.is_visible();
            let show_spr =
                self.regs.show_spr && (!self.regs.clip_spr || x > 7) && spr_pixel.is_visible();

            let pixel = if show_spr && !(show_bg && spr_pixel.behind) {
                spr_pixel
            } else if show_bg {
                bg_pixel
            } else {
                Pixel::default()
            };
            self.color_idx = self.bus.read_palette(pixel.address()) as usize;
        }

        if self.dot > 320 && self.dot <= 336 {
            self.background
                .next_pixel(self.regs.scroll.x.fine() as usize);
        }

        self.background_preparation();
        self.sprites_preparation();
    }

    fn vblank_start_line(&mut self) {
        if self.dot == 1 {
            self.regs.start_vblank();
        }
    }

    fn pre_render_line(&mut self) {
        self.background_preparation();

        match self.dot {
            1 => self.regs.stop_vblank(),
            _ => (),
        }

        if self.regs.render_enabled() && matches!(self.dot, 280..=304) {
            self.regs.update_vram_address_y();
        }
    }

    fn background_preparation(&mut self) {
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

    fn sprites_preparation(&mut self) {
        match self.dot {
            64 => self.sprites.clear(),
            256 => {
                for spr in self.oam.sprites_iter() {
                    if !self.is_spr_visible(spr) {
                        continue;
                    }

                    if self.sprites.len() == MAX_VISIBLE_SPRITES {
                        self.regs.spr_overflow = true;
                        break;
                    }
                    self.sprites.push(spr);
                }
            }
            263 => self.foreground.clear(),
            264..=320 if self.dot % 8 == 0 => {
                let index = (self.dot - 257) / 8;
                if let Some(spr) = self.sprites.get(index).copied() {
                    let pattern_addr = self.spr_pattern_addr(spr);
                    let hi = self.bus.read(pattern_addr + 8);
                    let lo = self.bus.read(pattern_addr);
                    self.foreground.load(spr, hi, lo);
                }
            }
            _ => (),
        }
    }

    fn is_spr_visible(&self, sprite: RawSprite) -> bool {
        let diff = self.scanline as isize - sprite.y as isize;
        diff >= 0 && diff < self.regs.spr_height as isize
    }

    pub fn spr_pattern_addr(&self, sprite: RawSprite) -> u16 {
        let diff = self.scanline as isize - sprite.y as isize;
        let row = if sprite.attr.flip_v {
            (self.regs.spr_height as isize) - 1 - diff
        } else {
            diff
        } as u16;

        match self.regs.spr_height {
            registers::SprHeight::Eight => {
                (self.regs.spr_pattern_table as u16) << 12 | (sprite.tile as u16) << 4 | row
            }
            registers::SprHeight::Sixteen => {
                // 8x16 mode uses the first 16 bytes for the top and the next 16 for the bottom
                // also, the used pattern table is based on the LSB of the tile number
                let tile = u16::from_le_bytes([sprite.tile & 0xFE, sprite.tile & 0x01]) << 4;
                tile | (row & 8) << 1 | row & 7
            }
        }
    }
}
