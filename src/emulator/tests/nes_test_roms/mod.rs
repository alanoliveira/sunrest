mod blargg_ppu_tests;
mod ppu_vbl_nmi;
mod sprite_hit_tests;

use super::*;

use std::path;

const MAX_CYCLES: u64 = 100_000_000;

fn build_emulator(rom_path: &str) -> Emulator {
    println!("Building console for {}", rom_path);
    let cartridge = cartridge::open_rom(&test_roms_path(rom_path));
    Emulator::new(cartridge)
}

fn clock_until(emulator: &mut Emulator, f: fn(&Emulator) -> bool) {
    let mut i = 0;
    while !f(emulator) {
        emulator.clock();
        i += 1;
        if i > MAX_CYCLES {
            panic!("Exceeded max cycles at PC = {:04X}", emulator.cpu.pc);
        }
    }
}

fn test_roms_path(rom_name: &str) -> path::PathBuf {
    let nes_test_roms_path =
        std::env::var("NES_TEST_ROMS_PATH").expect("NES_TEST_ROMS_PATH not set");
    path::PathBuf::from(nes_test_roms_path).join(rom_name)
}
