use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    vram: vram::Vram,
    palette_ram: palette_ram::PaletteRam,
}

impl TimeMachine {
    pub fn save(ppu: &Bus) -> Self {
        Self {
            vram: ppu.vram.clone(),
            palette_ram: ppu.palette_ram.clone(),
        }
    }

    pub fn load(self, ppu: &mut Bus) {
        ppu.vram = self.vram;
        ppu.palette_ram = self.palette_ram;
    }
}
