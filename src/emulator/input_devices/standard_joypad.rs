#[derive(Default, Debug, Clone, Copy)]
pub struct State {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl From<State> for u8 {
    fn from(val: State) -> Self {
        val.a as u8
            | (val.b as u8) << 1
            | (val.select as u8) << 2
            | (val.start as u8) << 3
            | (val.up as u8) << 4
            | (val.down as u8) << 5
            | (val.left as u8) << 6
            | (val.right as u8) << 7
    }
}

impl From<u8> for State {
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

#[derive(Default, Debug)]
pub struct StandardJoypad {
    pub state: State,
    strobe: bool,
    buffer: u8,
}

impl StandardJoypad {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_strobe(&mut self, val: bool) {
        if self.strobe && !val {
            self.buffer = self.collect_buttons();
        }
        self.strobe = val;
    }

    pub fn serial_read(&mut self) -> u8 {
        if self.strobe {
            // while strobe is high, return the current state of the A button
            self.state.a as u8
        } else {
            let state = self.buffer & 0x01;
            self.buffer = self.buffer.rotate_right(1);
            state
        }
    }

    fn collect_buttons(&self) -> u8 {
        self.state.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! mk_joy {
        ($($button:ident),*) => {
            StandardJoypad {
                state: State {
                    $( $button: true ),*,
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }

    #[test]
    fn test_serial_read() {
        let mut joy = mk_joy!(a, down, right, start);
        assert_eq!(joy.serial_read(), 0);
        joy.set_strobe(true);
        assert_eq!(joy.serial_read(), 1, "read A");
        assert_eq!(
            joy.serial_read(),
            1,
            "keep reading A button when strobe is high"
        );
        joy.set_strobe(false);
        assert_eq!(joy.serial_read(), 1, "read A");
        assert_eq!(joy.serial_read(), 0, "read B");
        assert_eq!(joy.serial_read(), 0, "read Select");
        assert_eq!(joy.serial_read(), 1, "read Start");
        assert_eq!(joy.serial_read(), 0, "read Up");
        assert_eq!(joy.serial_read(), 1, "read Down");
        assert_eq!(joy.serial_read(), 0, "read Left");
        assert_eq!(joy.serial_read(), 1, "read Right");
    }
}
