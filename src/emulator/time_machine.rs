use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    cpu_mem: bus::TimeMachine,
    cpu: cpu::TimeMachine,
    ppu_mem: ppu::bus::TimeMachine,
    ppu: ppu::TimeMachine,
    apu: apu::TimeMachine,
    cartridge: cartridge::TimeMachine,
    oam_dma: oam_dma::OamDma,
    dmc_dma: dmc_dma::DmcDma,
}

impl TimeMachine {
    pub fn save(emu: &Emulator) -> Self {
        Self {
            cpu_mem: bus::TimeMachine::save(&emu.cpu.mem),
            cpu: cpu::TimeMachine::save(&emu.cpu),
            ppu_mem: ppu::bus::TimeMachine::save(&emu.ppu.as_ref().mem),
            ppu: ppu::TimeMachine::save(&emu.ppu.as_ref()),
            apu: apu::TimeMachine::save(&emu.apu.as_ref()),
            cartridge: cartridge::TimeMachine::save(&emu.cartridge.borrow()),
            oam_dma: emu.oam_dma.clone(),
            dmc_dma: emu.dmc_dma.clone(),
        }
    }

    pub fn load(self, emu: &mut Emulator) {
        self.cpu_mem.load(&mut emu.cpu.mem);
        self.cpu.load(&mut emu.cpu);
        self.ppu_mem.load(&mut emu.ppu.as_mut().mem);
        self.ppu.load(&mut emu.ppu.as_mut());
        self.apu.load(&mut emu.apu.as_mut());
        self.cartridge.load(&mut emu.cartridge.borrow_mut());
        emu.oam_dma = self.oam_dma;
        emu.dmc_dma = self.dmc_dma;
    }
}
