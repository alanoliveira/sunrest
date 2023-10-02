mod nes_test_roms;

use super::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct ClockState {
    cpu: isize,
    ppu: isize,
    emu: isize,
}

impl ClockState {
    fn diff(self, other: Self) -> Self {
        Self {
            cpu: self.cpu - other.cpu,
            ppu: self.ppu - other.ppu,
            emu: self.emu - other.emu,
        }
    }
}

impl Emulator {
    fn current_clock_state(&self) -> ClockState {
        ClockState {
            cpu: self.cpu.cycle as isize,
            ppu: self.ppu.as_ref().cycle as isize,
            emu: self.cycle as isize,
        }
    }

    fn clock_to_next_frame(&mut self) -> ClockState {
        let current_frame = self.ppu.as_ref().frame;
        self.clock_until(move |emu| emu.ppu.as_ref().frame > current_frame)
    }

    const MAX_CYCLES: isize = 100_000;
    fn clock_until<F>(&mut self, f: F) -> ClockState
    where
        F: Fn(&Emulator) -> bool,
    {
        let clock_state = self.current_clock_state();
        while !f(self) {
            self.clock();
            if clock_state.emu > clock_state.emu + Self::MAX_CYCLES {
                panic!("Exceeded max cycles at PC = {:04X}", self.cpu.pc);
            }
        }

        self.current_clock_state().diff(clock_state)
    }
}

#[test]
fn test_frame_timing() {
    let cartridge = cartridge::Cartridge::new(
        cartridge::CartridgeInfo {
            prg_banks: 1,
            chr_banks: 1,
            ..Default::default()
        },
        &vec![0xEA; 0x8000],
        &vec![],
    );
    let mut emulator = Emulator::new(cartridge);

    for _ in 0..5 {
        emulator.ppu.as_mut().io_ports().write(0x01, 0x00); // disable rendering
        let ini_state = emulator.current_clock_state();
        for _ in 0..3 {
            emulator.clock_to_next_frame();
        }
        let diff_state = emulator.current_clock_state().diff(ini_state);
        assert_eq!(
            diff_state.cpu, 89342,
            "a ppu frame should take 29780⅔ cpu cycles when rendering is disabled"
        );

        emulator.ppu.as_mut().io_ports().write(0x01, 0x18); // enable rendering
        let ini_state = emulator.current_clock_state();
        for _ in 0..2 {
            emulator.clock_to_next_frame();
        }
        let diff_state = emulator.current_clock_state().diff(ini_state);
        assert_eq!(
            diff_state.cpu, 59561,
            "a ppu frame should take 29780½ cpu cycles when rendering is disabled"
        );
    }
}
