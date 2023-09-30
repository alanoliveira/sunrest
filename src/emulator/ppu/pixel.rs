#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    #[default]
    Background,
    Sprite,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Pixel {
    pub kind: Kind,
    pub palette: u8,
    pub color: u8,
    pub behind: bool,
}

impl Pixel {
    pub fn new(kind: Kind, palette: u8, color: u8, behind: bool) -> Self {
        Self {
            kind,
            palette,
            color,
            behind,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.color != 0
    }

    pub fn table(&self) -> u8 {
        if self.kind == Kind::Background {
            0
        } else {
            1
        }
    }
}
