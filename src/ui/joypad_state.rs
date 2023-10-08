#[derive(Default, Clone, Copy)]
pub struct JoypadState {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Into<u8> for JoypadState {
    fn into(self) -> u8 {
        self.a as u8
            | (self.b as u8) << 1
            | (self.select as u8) << 2
            | (self.start as u8) << 3
            | (self.up as u8) << 4
            | (self.down as u8) << 5
            | (self.left as u8) << 6
            | (self.right as u8) << 7
    }
}

impl From<u8> for JoypadState {
    fn from(value: u8) -> Self {
        Self {
            a: value & 0x01 != 0,
            b: value & 0x02 != 0,
            select: value & 0x04 != 0,
            start: value & 0x08 != 0,
            up: value & 0x10 != 0,
            down: value & 0x20 != 0,
            left: value & 0x40 != 0,
            right: value & 0x80 != 0,
        }
    }
}
