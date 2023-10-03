#[derive(Debug, Default, Clone, Copy)]
pub struct Scroll {
    pub x: Axis,
    pub y: Axis,
}

impl Scroll {
    pub fn set_x(&mut self, val: impl Into<Axis>) {
        self.x = val.into();
    }

    pub fn set_y(&mut self, val: impl Into<Axis>) {
        self.y = val.into();
    }
}

#[derive(Default, Clone, Copy)]
pub struct Axis {
    pub raw: u8,
}

impl std::fmt::Debug for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:08b} (coarse: {}, fine: {})",
                self.raw,
                self.coarse(),
                self.fine(),
            )
        } else {
            write!(f, "{:02X}", self.raw)
        }
    }
}

impl Axis {
    const COARSE: u8 = 0b1111_1000;
    const FINE: u8 = 0b0000_0111;

    pub fn coarse(&self) -> u8 {
        (self.raw & Self::COARSE) >> Self::COARSE.trailing_zeros()
    }

    pub fn fine(&self) -> u8 {
        self.raw & Self::FINE
    }

    pub fn set_coarse(&mut self, val: u8) {
        self.raw = (self.raw & !Self::COARSE) | (val << Self::COARSE.trailing_zeros());
    }

    pub fn set_fine(&mut self, val: u8) {
        self.raw = (self.raw & !Self::FINE) | (val & Self::FINE);
    }
}

impl From<u8> for Axis {
    fn from(val: u8) -> Self {
        Self { raw: val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis() {
        let mut axis = Axis::from(0b1010_1010);
        assert_eq!(axis.coarse(), 0b0001_0101);
        assert_eq!(axis.fine(), 0b0000_0010);
        axis.set_coarse(0b1111_1100);
        assert_eq!(axis.coarse(), 0b0001_1100);
        axis.set_fine(0b1111_1100);
        assert_eq!(axis.fine(), 0b0000_0100);
    }
}
