#[derive(Debug, Default, Clone, Copy)]
pub enum NameTable {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl NameTable {
    pub fn h(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 0,
            Self::Three => 1,
        }
    }

    pub fn v(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 1,
        }
    }
}

impl From<u8> for NameTable {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => unreachable!(),
        }
    }
}
