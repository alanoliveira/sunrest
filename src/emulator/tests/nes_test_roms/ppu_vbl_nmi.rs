use super::*;

fn extract_error(emulator: &Emulator) -> String {
    (0x6004..)
        .map_while(|addr| match emulator.cpu.mem.read(addr) {
            0 => None,
            val => Some(char::from(val)),
        })
        .collect()
}

#[test]
fn basics() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/01-vbl_basics.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn vbl_set_time() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/02-vbl_set_time.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn vbl_clear_time() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/03-vbl_clear_time.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn nmi_control() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/04-nmi_control.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn nmi_timing() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/05-nmi_timing.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn suppressmemn() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/06-suppression.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn nmi_on_timing() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/07-nmi_on_timing.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn nmi_off_timing() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/08-nmi_off_timing.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn even_odd_frames() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/09-even_odd_frames.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}

#[test]
fn even_odd_timing() {
    let mut emulator = build_emulator("ppu_vbl_nmi/rom_singles/10-even_odd_timing.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE8D5);
    match emulator.cpu.mem.read(0x0A) {
        0 => {}
        _ => {
            panic!("Error: {}", extract_error(&emulator))
        }
    }
}
