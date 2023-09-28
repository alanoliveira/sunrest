#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    #[default]
    Background,
    Sprite,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Pixel {
    kind: Kind,
    palette: u8,
    color: u8,
    behind: bool,
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

    pub fn address(&self) -> u16 {
        let palette_table = if self.kind == Kind::Background {
            0x00
        } else {
            0x10
        };
        (self.palette as u16) * 4 + (self.color as u16) + palette_table
    }
}
