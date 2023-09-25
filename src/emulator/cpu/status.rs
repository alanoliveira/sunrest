#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub raw: u8,
}

impl Status {
    pub const C: u8 = 0b0000_0001; // carry
    pub const Z: u8 = 0b0000_0010; // zero
    pub const I: u8 = 0b0000_0100; // interrupt disable
    pub const D: u8 = 0b0000_1000; // decimal mode
    pub const B: u8 = 0b0001_0000; // break
    pub const U: u8 = 0b0010_0000; // unused
    pub const V: u8 = 0b0100_0000; // overflow
    pub const N: u8 = 0b1000_0000; // negative

    pub fn set(&mut self, flag: u8, val: bool) {
        if val {
            self.raw |= flag;
        } else {
            self.raw &= !flag;
        }
    }

    pub fn get(&self, flag: u8) -> bool {
        self.raw & flag != 0
    }

    pub(super) fn set_zn(&mut self, val: u8) {
        self.set(Self::Z, val == 0);
        self.set(Self::N, val & 0x80 != 0);
    }
}

impl From<u8> for Status {
    fn from(val: u8) -> Self {
        Self { raw: val }
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:02X} ", self.raw)?;
        }

        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            if self.get(Status::N) { "N" } else { "_" },
            if self.get(Status::V) { "V" } else { "_" },
            if self.get(Status::U) { "U" } else { "_" },
            if self.get(Status::B) { "B" } else { "_" },
            if self.get(Status::D) { "D" } else { "_" },
            if self.get(Status::I) { "I" } else { "_" },
            if self.get(Status::Z) { "Z" } else { "_" },
            if self.get(Status::C) { "C" } else { "_" },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status() {
        let status: Status = 0b1010_1010.into();

        assert!(status.get(Status::N));
        assert!(!status.get(Status::V));
        assert!(status.get(Status::U));
        assert!(!status.get(Status::B));
        assert!(status.get(Status::D));
        assert!(!status.get(Status::I));
        assert!(status.get(Status::Z));
        assert!(!status.get(Status::C));
    }

    #[test]
    fn test_set_status() {
        let mut status: Status = 0b0000_0000.into();

        assert!(!status.get(Status::C));
        assert!(!status.get(Status::Z));
        status.set(Status::C, true);
        status.set(Status::Z, true);
        assert!(status.get(Status::C));
        assert!(status.get(Status::Z));
        assert!(status.get(Status::Z | Status::C));
    }

    #[test]
    fn test_set_p_zn() {
        let mut status: Status = 0b0000_0000.into();

        status.set_zn(0);
        assert!(status.get(Status::Z));
        assert!(!status.get(Status::N));

        status.set_zn(0x80);
        assert!(!status.get(Status::Z));
        assert!(status.get(Status::N));

        status.set_zn(0x7F);
        assert!(!status.get(Status::Z));
        assert!(!status.get(Status::N));
    }
}
