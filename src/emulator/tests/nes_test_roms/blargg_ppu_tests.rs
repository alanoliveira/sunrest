use super::*;

#[test]
fn basics() {
    let mut console = build_emulator("blargg_ppu_tests_2005.09.15b/palette_ram.nes");
    clock_until(&mut console, |c| c.cpu.pc == 0xE412);
    match console.cpu.io.read(0xF0) {
        1 => {}
        2 => panic!("2) Palette read shouldn't be buffered like other VRAM"),
        3 => panic!("3) Palette write/read doesn't work"),
        4 => panic!("4) Palette should be mirrored within $3f00-$3fff"),
        5 => panic!("5) Write to $10 should be mirrored at $00"),
        6 => panic!("6) Write to $00 should be mirrored at $10"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn power_up_palette() {
    let mut console = build_emulator("blargg_ppu_tests_2005.09.15b/power_up_palette.nes");
    clock_until(&mut console, |c| c.cpu.pc == 0xE3AC);
    match console.cpu.io.read(0xF0) {
        1 => {}
        2 => panic!("2) Palette differs from table"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn sprite_ram() {
    let mut console = build_emulator("blargg_ppu_tests_2005.09.15b/sprite_ram.nes");
    clock_until(&mut console, |c| c.cpu.pc == 0xE467);
    match console.cpu.io.read(0xF0) {
        1 => {}
        2 => panic!("2) Basic read/write doesn't work"),
        3 => panic!("3) Address should increment on $2004 write"),
        4 => panic!("4) Address should not increment on $2004 read"),
        5 => panic!("5) Third sprite bytes should be masked with $e3 on read"),
        6 => panic!("6) $4014 DMA copy doesn't work at all"),
        7 => panic!("7) $4014 DMA copy should start at value in $2003 and wrap"),
        8 => panic!("8) $4014 DMA copy should leave value in $2003 intact"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn vbl_clear_timing() {
    let mut console = build_emulator("blargg_ppu_tests_2005.09.15b/vbl_clear_time.nes");
    clock_until(&mut console, |c| c.cpu.pc == 0xE3B3);
    match console.cpu.io.read(0xF0) {
        1 => {}
        2 => panic!("2) VBL flag cleared too soon"),
        3 => panic!("3) VBL flag cleared too late"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn vram_access() {
    let mut console = build_emulator("blargg_ppu_tests_2005.09.15b/vram_access.nes");
    clock_until(&mut console, |c| c.cpu.pc == 0xE48D);
    match console.cpu.io.read(0xF0) {
        1 => {}
        2 => panic!("2) VRAM reads should be delayed in a buffer"),
        3 => panic!("3) Basic Write/read doesn't work"),
        4 => panic!("4) Read buffer shouldn't be affected by VRAM write"),
        5 => panic!("5) Read buffer shouldn't be affected by palette write"),
        6 => panic!("6) Palette read should also read VRAM into read buffer"),
        7 => panic!("7) \"Shadow\" VRAM read unaffected by palette transparent color mirroring"),
        err => panic!("Unknown error {err:02X}"),
    }
}
