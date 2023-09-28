#[derive(Debug, Default, Clone, Copy)]
pub struct RawSprite {
    pub attr: Attributes,
    pub x: u8,
    pub y: u8,
    pub tile: u8,
}

impl RawSprite {
    pub fn new(y: u8, tile: u8, attr: impl Into<Attributes>, x: u8) -> Self {
        Self {
            attr: attr.into(),
            x,
            y,
            tile,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Attributes {
    pub palette: u8,
    pub behind: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl From<u8> for Attributes {
    fn from(val: u8) -> Self {
        Self {
            palette: val & 0b11,
            behind: val & 0b0010_0000 != 0,
            flip_h: val & 0b0100_0000 != 0,
            flip_v: val & 0b1000_0000 != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attrs() {
        let attrs = Attributes::from(0b1010_0001);
        assert_eq!(attrs.palette, 0b01);
        assert!(attrs.behind);
        assert!(!attrs.flip_h);
        assert!(attrs.flip_v);
    }
}
