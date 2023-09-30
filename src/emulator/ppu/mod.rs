mod background;
mod debugger;
mod foreground;
mod io_ports;
mod memory;
mod oam;
mod pixel;
mod registers;
mod sprite;

pub mod bus;

pub use memory::*;
use pixel::{Kind as PixelKind, Pixel};
use sprite::RawSprite;

const DOTS_PER_LINE: usize = 341;
const LINES_PER_FRAME: usize = 262;
const OAM_SIZE: usize = 0x100;
const MAX_VISIBLE_SPRITES: usize = 8;

pub struct Ppu<M: Memory> {
    pub mem: M,
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

impl<M: Memory> std::fmt::Debug for Ppu<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ppu {{ ({:03},{:03}) [{}] }}",
            self.scanline, self.dot, self.frame,
        )
    }
}

impl<M: Memory> Ppu<M> {
    pub fn new(mem: M) -> Self {
        Self {
            mem,
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

    pub fn debugger(&self) -> debugger::Debugger<M> {
        debugger::Debugger(self)
    }

    pub fn take_nmi(&mut self) -> bool {
        self.regs.nmi.take().is_some()
    }

    pub fn io_ports(&mut self) -> io_ports::IOPorts<M> {
        io_ports::IOPorts::new(self)
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
            self.pixel_preparation();
        }

        if self.regs.render_enabled() {
            self.background_preparation();
            self.sprites_preparation();
        }
    }

    fn vblank_start_line(&mut self) {
        if self.dot == 1 {
            self.regs.start_vblank();
        }
    }

    fn pre_render_line(&mut self) {
        if self.regs.render_enabled() {
            self.background_preparation();
        }

        match self.dot {
            1 => {
                self.regs.stop_vblank();
                self.regs.spr0_hit = false;
                self.regs.spr_overflow = false;
            }
            66 => self.regs.spr0_found = false,
            _ => (),
        }

        if self.regs.render_enabled() && matches!(self.dot, 280..=304) {
            self.regs.update_vram_address_y();
        }
    }

    fn pixel_preparation(&mut self) {
        let x = self.dot - 1;

        let bg_pixel = self.background.pixel_at(self.regs.scroll.x.fine());
        let spr_pixel = self.foreground.pixel_at(self.dot - 1).unwrap_or_default();

        let show_bg = self.regs.show_bg && (!self.regs.clip_bg || x > 7);
        let show_spr = self.regs.show_spr && (!self.regs.clip_spr || x > 7);
        let bg_visible = show_bg && bg_pixel.is_visible();
        let spr_visible = show_spr && spr_pixel.is_visible();

        let pix = if spr_visible && !(bg_visible && spr_pixel.behind) {
            spr_pixel
        } else if bg_visible {
            bg_pixel
        } else {
            Pixel::default()
        };
        self.color_idx = self.mem.read_palette(pix.table(), pix.palette, pix.color) as usize;

        if !self.regs.spr0_hit
            && self.regs.spr0_found
            && self.foreground.zero_fetch
            && bg_visible
            && spr_visible
            && x != 255
        {
            self.regs.spr0_hit = true;
        }
    }

    fn background_preparation(&mut self) {
        if self.dot > 0 && self.dot <= 256 || self.dot > 320 && self.dot <= 336 {
            self.background.shift();
        }

        if matches!(self.dot, 1..=256 | 321..=336) {
            match self.dot % 8 {
                0 => {
                    self.background.load();
                    self.regs.vram_addr.increment_x();
                }
                1 => {
                    self.background.tmp_tile_idx = self.mem.read_nametable(
                        self.regs.vram_addr.nametable(),
                        self.regs.vram_addr.coarse_y(),
                        self.regs.vram_addr.coarse_x(),
                    );
                }
                3 => {
                    self.background.tmp_palette = self.mem.read_attribute_palette(
                        self.regs.vram_addr.nametable(),
                        self.regs.vram_addr.coarse_y(),
                        self.regs.vram_addr.coarse_x(),
                    );
                }
                num @ (5 | 7) => {
                    let addr = (self.regs.bg_pattern_table as u16) << 12
                        | (self.background.tmp_tile_idx as u16) << 4
                        | (self.regs.vram_addr.fine_y() as u16);
                    if num == 5 {
                        // the byte containing the lo bits is stored first
                        self.background.tmp_pattern_lo = self.mem.read(addr);
                    } else {
                        self.background.tmp_pattern_hi = self.mem.read(addr + 8);
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
                self.regs.spr0_found = false;
                for (i, spr) in self.oam.sprites_iter().enumerate() {
                    if !self.is_spr_visible(spr) {
                        continue;
                    }

                    if self.sprites.len() == MAX_VISIBLE_SPRITES {
                        self.regs.spr_overflow = true;
                        break;
                    }

                    self.sprites.push(spr);
                    self.regs.spr0_found |= i == 0;
                }
            }
            263 => self.foreground.clear(),
            264..=320 if self.dot % 8 == 0 => {
                let index = (self.dot - 257) / 8;
                if let Some(spr) = self.sprites.get(index).copied() {
                    let pattern_addr = self.spr_pattern_addr(spr);
                    let hi = self.mem.read(pattern_addr + 8);
                    let lo = self.mem.read(pattern_addr);
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
