use super::*;

struct SpritePixels {
    hi: u8,
    lo: u8,
    palette: u8,
    x: usize,
    behind: bool,
}

impl SpritePixels {
    fn color(&self) -> u8 {
        ((self.hi & 0x80) >> 6) | ((self.lo & 0x80) >> 7)
    }
}

pub struct Foreground {
    spr_pixels: Vec<SpritePixels>,
}

impl Foreground {
    pub fn new() -> Self {
        Self {
            spr_pixels: Vec::with_capacity(MAX_VISIBLE_SPRITES),
        }
    }

    pub fn clear(&mut self) {
        self.spr_pixels.clear();
    }

    pub fn load(&mut self, sprite: RawSprite, pattern_hi: u8, pattern_lo: u8) {
        let mut pattern_hi = pattern_hi;
        let mut pattern_lo = pattern_lo;

        if sprite.attr.flip_h {
            pattern_hi = pattern_hi.reverse_bits();
            pattern_lo = pattern_lo.reverse_bits();
        }

        self.spr_pixels.push(SpritePixels {
            hi: pattern_hi,
            lo: pattern_lo,
            palette: sprite.attr.palette,
            x: sprite.x as usize,
            behind: sprite.attr.behind,
        });
    }

    pub fn next_pixel(&mut self) -> Option<Pixel> {
        let pixel = self
            .spr_pixels
            .iter()
            .filter(|p| p.x == 0)
            .map(|p| Pixel::new(PixelKind::Sprite, p.palette, p.color(), p.behind))
            .find(|p| p.is_visible());
        self.shift();
        pixel
    }

    fn shift(&mut self) {
        self.spr_pixels.iter_mut().for_each(|p| {
            if p.x > 0 {
                p.x -= 1;
            } else {
                p.hi <<= 1;
                p.lo <<= 1;
            }
        });
    }
}

fn generate_pixels(mut hi: u8, mut lo: u8, spr: RawSprite) -> [Pixel; 8] {
    let mut pixels = [Pixel::default(); 8];
    for i in 0..8 {
        let color = ((hi & 0x80) >> 6) | ((lo & 0x80) >> 7);
        hi <<= 1;
        lo <<= 1;
        pixels[i] = Pixel::new(PixelKind::Sprite, spr.attr.palette, color, spr.attr.behind);
    }
    pixels
}
