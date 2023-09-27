#[derive(Debug, Default, Clone, Copy)]
pub enum PatternTable {
    #[default]
    Zero,
    One,
}

impl From<u8> for PatternTable {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Zero,
            1 => Self::One,
            _ => unreachable!(),
        }
    }
}
