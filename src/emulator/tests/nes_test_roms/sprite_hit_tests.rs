use super::*;

#[test]
fn basics() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/01.basics.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Sprite hit isn't working at all"),
        3 => panic!("3) Should hit even when completely behind background"),
        4 => panic!("4) Should miss when background rendering is off"),
        5 => panic!("5) Should miss when sprite rendering is off"),
        6 => panic!("6) Should miss when all rendering is off"),
        7 => panic!("7) All-transparent sprite should miss"),
        8 => panic!("8) Only low two palette index bits are relevant"),
        9 => panic!("9) Any non-zero palette index should hit with any other"),
        10 => panic!("10) Should miss when background is all transparent"),
        11 => panic!("11) Should always miss other sprites"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn alignment() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/02.alignment.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Basic sprite-background alignment is way off"),
        3 => panic!("3) Sprite should miss left side of bg tile"),
        4 => panic!("4) Sprite should hit left side of bg tile"),
        5 => panic!("5) Sprite should miss right side of bg tile"),
        6 => panic!("6) Sprite should hit right side of bg tile"),
        7 => panic!("7) Sprite should miss top of bg tile"),
        8 => panic!("8) Sprite should hit top of bg tile"),
        9 => panic!("9) Sprite should miss bottom of bg tile"),
        10 => panic!("1)0 Sprite should hit bottom of bg tile"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn corners() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/03.corners.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Lower-right pixel should hit"),
        3 => panic!("3) Lower-left pixel should hit"),
        4 => panic!("4) Upper-right pixel should hit"),
        5 => panic!("5) Upper-left pixel should hit"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn flip() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/04.flip.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE5B6);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Horizontal flipping doesn't work"),
        3 => panic!("3) Vertical flipping doesn't work"),
        4 => panic!("4) Horizontal + Vertical flipping doesn't work"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn left_clip() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/05.left_clip.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Should miss when entirely in left-edge clipping"),
        3 => panic!("3) Left-edge clipping occurs when $2001 is not $1e"),
        4 => panic!("4) Left-edge clipping is off when $2001 = $1e"),
        5 => panic!("5) Left-edge clipping blocks all hits only when X = 0"),
        6 => panic!("6) Should miss; sprite pixel covered by left-edge clip"),
        7 => panic!("7) Should hit; sprite pixel outside left-edge clip"),
        8 => panic!("8) Should hit; sprite pixel outside left-edge clip"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn right_edge() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/06.right_edge.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Should always miss when X = 255"),
        3 => panic!("3) Should hit; sprite has pixels < 255"),
        4 => panic!("4) Should miss; sprite pixel is at 255"),
        5 => panic!("5) Should hit; sprite pixel is at 254"),
        6 => panic!("6) Should also hit; sprite pixel is at 254"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn screen_bottom() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/07.screen_bottom.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Should always miss when Y >= 239"),
        3 => panic!("3) Can hit when Y < 239"),
        4 => panic!("4) Should always miss when Y = 255"),
        5 => panic!("5) Should hit; sprite pixel is at 238"),
        6 => panic!("6) Should miss; sprite pixel is at 239"),
        7 => panic!("7) Should hit; sprite pixel is at 238"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn double_height() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/08.double_height.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Lower sprite tile should miss bottom of bg tile"),
        3 => panic!("3) Lower sprite tile should hit bottom of bg tile and miss top of bg tile"),
        4 => panic!("4) Lower sprite tile should hit top of bg tile"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn timing_basics() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/09.timing_basics.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE64C);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Upper-left corner too soon"),
        3 => panic!("3) Upper-left corner too late"),
        4 => panic!("4) Upper-right corner too soon"),
        5 => panic!("5) Upper-right corner too late"),
        6 => panic!("6) Lower-left corner too soon"),
        7 => panic!("7) Lower-left corner too late"),
        8 => panic!("8) Cleared at end of VBL too soon"),
        9 => panic!("9) Cleared at end of VBL too late"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn timing_order() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/10.timing_order.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Upper-left corner too soon"),
        3 => panic!("3) Upper-left corner too late"),
        4 => panic!("4) Upper-right corner too soon"),
        5 => panic!("5) Upper-right corner too late"),
        6 => panic!("6) Lower-left corner too soon"),
        7 => panic!("7) Lower-left corner too late"),
        8 => panic!("8) Lower-right corner too soon"),
        9 => panic!("9) Lower-right corner too late"),
        err => panic!("Unknown error {err:02X}"),
    }
}

#[test]
fn edge_timing() {
    let mut emulator = build_emulator("sprite_hit_tests_2005.10.05/11.edge_timing.nes");
    clock_until(&mut emulator, |c| c.cpu.pc == 0xE635);
    match emulator.cpu.io.read(0xF8) {
        1 => {}
        2 => panic!("2) Hit time shouldn't be based on pixels under left clip"),
        3 => panic!("3) Hit time shouldn't be based on pixels at X=255"),
        4 => panic!("4) Hit time shouldn't be based on pixels off right edge"),
        err => panic!("Unknown error {err:02X}"),
    }
}
