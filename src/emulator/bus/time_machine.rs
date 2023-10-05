use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    wram: wram::Wram,
    sram: sram::Sram, // in reality this is on the cartridge
    oam_dma_page: Option<u8>,
    input_ctrl_write: Option<u8>,
    device1_state: Option<u8>,
    device2_state: Option<u8>,
}

impl TimeMachine {
    pub fn save(bus: &Bus) -> Self {
        Self {
            wram: bus.wram.clone(),
            sram: bus.sram.clone(),
            oam_dma_page: bus.oam_dma_page,
            input_ctrl_write: bus.input_ctrl_write,
            device1_state: bus.device1_state.get(),
            device2_state: bus.device2_state.get(),
        }
    }

    pub fn load(&self, bus: &mut Bus) {
        bus.wram = self.wram.clone();
        bus.sram = self.sram.clone();
        bus.oam_dma_page = self.oam_dma_page;
        bus.input_ctrl_write = self.input_ctrl_write;
        bus.device1_state.set(self.device1_state);
        bus.device2_state.set(self.device2_state);
    }
}
