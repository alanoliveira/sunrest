pub struct Signal {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }
}

pub static DEFAULT_PALETTE: [Color; 64] = [
    Color(0x62, 0x62, 0x62),
    Color(0x0D, 0x22, 0x6B),
    Color(0x24, 0x14, 0x76),
    Color(0x3B, 0x0A, 0x6B),
    Color(0x4C, 0x07, 0x4D),
    Color(0x52, 0x0C, 0x24),
    Color(0x4C, 0x17, 0x00),
    Color(0x3B, 0x26, 0x00),
    Color(0x24, 0x34, 0x00),
    Color(0x0D, 0x3D, 0x00),
    Color(0x00, 0x40, 0x00),
    Color(0x00, 0x3B, 0x24),
    Color(0x00, 0x30, 0x4D),
    Color(0x00, 0x00, 0x00),
    Color(0x00, 0x00, 0x00),
    Color(0x00, 0x00, 0x00),
    Color(0xAB, 0xAB, 0xAB),
    Color(0x31, 0x56, 0xB1),
    Color(0x50, 0x43, 0xC5),
    Color(0x70, 0x34, 0xBB),
    Color(0x89, 0x2F, 0x95),
    Color(0x94, 0x34, 0x5F),
    Color(0x8E, 0x42, 0x26),
    Color(0x79, 0x55, 0x00),
    Color(0x5B, 0x68, 0x00),
    Color(0x3B, 0x77, 0x00),
    Color(0x22, 0x7C, 0x15),
    Color(0x17, 0x77, 0x4C),
    Color(0x1D, 0x69, 0x85),
    Color(0x00, 0x00, 0x00),
    Color(0x00, 0x00, 0x00),
    Color(0x00, 0x00, 0x00),
    Color(0xFF, 0xFF, 0xFF),
    Color(0x7C, 0xAA, 0xFF),
    Color(0x9B, 0x96, 0xFF),
    Color(0xBD, 0x86, 0xFF),
    Color(0xD8, 0x7E, 0xF1),
    Color(0xE6, 0x82, 0xBA),
    Color(0xE3, 0x8F, 0x7F),
    Color(0xD0, 0xA2, 0x4E),
    Color(0xB2, 0xB7, 0x34),
    Color(0x90, 0xC7, 0x39),
    Color(0x74, 0xCE, 0x5C),
    Color(0x66, 0xCB, 0x92),
    Color(0x69, 0xBE, 0xCE),
    Color(0x4E, 0x4E, 0x4E),
    Color(0x00, 0x00, 0x00),
    Color(0x00, 0x00, 0x00),
    Color(0xFF, 0xFF, 0xFF),
    Color(0xC9, 0xDE, 0xFC),
    Color(0xD5, 0xD6, 0xFF),
    Color(0xE2, 0xCF, 0xFF),
    Color(0xEE, 0xCC, 0xFC),
    Color(0xF5, 0xCC, 0xE7),
    Color(0xF5, 0xD1, 0xCF),
    Color(0xEE, 0xD8, 0xBB),
    Color(0xE2, 0xE1, 0xAE),
    Color(0xD5, 0xE8, 0xAE),
    Color(0xC9, 0xEB, 0xBB),
    Color(0xC2, 0xEB, 0xCF),
    Color(0xC2, 0xE6, 0xE7),
    Color(0xB8, 0xB8, 0xB8),
    Color(0x00, 0x00, 0x00),
    Color(0x00, 0x00, 0x00),
];
