#[derive(Debug, Default, Clone, Copy)]
pub enum SprHeight {
    #[default]
    Eight,
    Sixteen,
}

impl From<u8> for SprHeight {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Eight,
            1 => Self::Sixteen,
            _ => unreachable!(),
        }
    }
}
