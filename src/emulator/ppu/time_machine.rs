use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    nmi: nmi::Nmi,
    oam: oam::Oam,
    sprites: Vec<RawSprite>,
    regs: registers::Registers,
    background: background::Background,
    foreground: foreground::Foreground,
    odd_frame: bool,
    color_idx: usize,
    dot: usize,
    scanline: usize,
    frame: usize,
    cycle: usize,
}

impl TimeMachine {
    pub fn save<M: Memory>(ppu: &Ppu<M>) -> Self {
        Self {
            nmi: ppu.nmi.clone(),
            oam: ppu.oam.clone(),
            sprites: ppu.sprites.clone(),
            regs: ppu.regs.clone(),
            background: ppu.background.clone(),
            foreground: ppu.foreground.clone(),
            odd_frame: ppu.odd_frame,
            color_idx: ppu.color_idx,
            dot: ppu.dot,
            scanline: ppu.scanline,
            frame: ppu.frame,
            cycle: ppu.cycle,
        }
    }

    pub fn load<M: Memory>(self, ppu: &mut Ppu<M>) {
        ppu.nmi = self.nmi;
        ppu.oam = self.oam;
        ppu.sprites = self.sprites;
        ppu.regs = self.regs;
        ppu.background = self.background;
        ppu.foreground = self.foreground;
        ppu.odd_frame = self.odd_frame;
        ppu.color_idx = self.color_idx;
        ppu.dot = self.dot;
        ppu.scanline = self.scanline;
        ppu.frame = self.frame;
        ppu.cycle = self.cycle;
    }
}
