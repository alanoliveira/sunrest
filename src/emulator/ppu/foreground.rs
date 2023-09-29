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
        ((self.hi & 0x01) << 1) | (self.lo & 0x01)
    }

    fn is_visible(&self) -> bool {
        self.x == 0 && self.color() != 0
    }
}

pub struct Foreground {
    spr_pixels: Vec<SpritePixels>,
    pub zero_fetch: bool,
}

impl Foreground {
    pub fn new() -> Self {
        Self {
            spr_pixels: Vec::with_capacity(MAX_VISIBLE_SPRITES),
            zero_fetch: false,
        }
    }

    pub fn clear(&mut self) {
        self.spr_pixels.clear();
        self.zero_fetch = false;
    }

    pub fn load(&mut self, sprite: RawSprite, pattern_hi: u8, pattern_lo: u8) {
        let mut pattern_hi = pattern_hi;
        let mut pattern_lo = pattern_lo;

        if !sprite.attr.flip_h {
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

    pub fn cur_pixel(&mut self) -> Option<Pixel> {
        self.zero_fetch = false;
        self.spr_pixels.iter().enumerate().find_map(|(i, p)| {
            if p.x == 0 && p.color() != 0 {
                self.zero_fetch = i == 0;
                Some(Pixel::new(
                    PixelKind::Sprite,
                    p.palette,
                    p.color(),
                    p.behind,
                ))
            } else {
                None
            }
        })
    }

    pub fn shift(&mut self) {
        self.spr_pixels.iter_mut().for_each(|p| {
            if p.x > 0 {
                p.x -= 1;
            } else {
                p.hi >>= 1;
                p.lo >>= 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_pixel(pal: u8, col: u8, bhd: bool) -> Pixel {
        Pixel::new(PixelKind::Sprite, pal, col, bhd)
    }

    fn mk_spr(x: u8, bhd: bool, flip: bool, pal: u8) -> RawSprite {
        RawSprite {
            y: 0,
            x,
            tile: 0,
            attr: sprite::Attributes {
                behind: bhd,
                flip_h: flip,
                flip_v: false,
                palette: pal,
            },
        }
    }

    #[test]
    fn test_spr_pixels() {
        let mut spr_pixels = Foreground::new();

        spr_pixels.load(mk_spr(0, true, false, 1), 0b10101010, 0b11101110);
        spr_pixels.load(mk_spr(1, false, false, 0), 0b00001010, 0b00001110);
        spr_pixels.load(mk_spr(12, false, false, 2), 0b11010101, 0b01110111);

        let mut get_n_shift = || {
            let pix = spr_pixels.cur_pixel();
            spr_pixels.shift();
            pix
        };

        assert_eq!(get_n_shift(), Some(mk_pixel(1, 3, true)));
        assert_eq!(get_n_shift(), Some(mk_pixel(1, 1, true)));
        assert_eq!(get_n_shift(), Some(mk_pixel(1, 3, true)));
        assert_eq!(get_n_shift(), None);
        assert_eq!(get_n_shift(), Some(mk_pixel(1, 3, true)));
        assert_eq!(get_n_shift(), Some(mk_pixel(1, 1, true)));
        assert_eq!(get_n_shift(), Some(mk_pixel(1, 3, true)));
        assert_eq!(get_n_shift(), Some(mk_pixel(0, 3, false)));
        for _ in 0..4 {
            assert_eq!(get_n_shift(), None);
        }
        assert_eq!(get_n_shift(), Some(mk_pixel(2, 2, false)));
        assert_eq!(get_n_shift(), Some(mk_pixel(2, 3, false)));
    }
}
