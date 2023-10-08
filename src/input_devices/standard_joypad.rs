#[derive(Default, Debug)]
pub struct StandardJoypad {
    strobe: bool,
    buffer: u8,

    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
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
            self.a as u8
        } else {
            let state = self.buffer & 0x01;
            self.buffer = self.buffer.rotate_right(1);
            state
        }
    }

    fn collect_buttons(&self) -> u8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! mk_joy {
        ($($button:ident),*) => {
            StandardJoypad {
                $( $button: true ),*,
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
