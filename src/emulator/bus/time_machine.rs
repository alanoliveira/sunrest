use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    wram: wram::Wram,
    sram: sram::Sram, // in reality this is on the cartridge
    oam_dma_page: Option<u8>,
    input_latch: u8,
}

impl TimeMachine {
    pub fn save(bus: &Bus) -> Self {
        Self {
            wram: bus.wram.clone(),
            sram: bus.sram.clone(),
            oam_dma_page: bus.oam_dma_page,
            input_latch: bus.input_latch,
        }
    }

    pub fn load(&self, bus: &mut Bus) {
        bus.wram = self.wram.clone();
        bus.sram = self.sram.clone();
        bus.oam_dma_page = self.oam_dma_page;
        bus.input_latch = self.input_latch;
    }
}
