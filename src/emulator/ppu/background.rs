use super::*;

const BUFFER_SIZE: usize = 16;

pub struct Background {
    pattern_hi: u16,
    pattern_lo: u16,
    palette_hi: u16,
    palette_lo: u16,

    pub tmp_tile_idx: u8,
    pub tmp_palette: u8,
    pub tmp_pattern_hi: u8,
    pub tmp_pattern_lo: u8,
}

impl std::fmt::Debug for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "[ {:016b} {:016b} {:016b} {:016b} ]",
                self.pattern_hi, self.pattern_lo, self.palette_hi, self.palette_lo
            )
        } else {
            f.debug_struct("Background")
                .field("pattern_hi", &format_args!("{:#016b}", self.pattern_hi))
                .field("pattern_lo", &format_args!("{:#016b}", self.pattern_lo))
                .field("palette_hi", &format_args!("{:#016b}", self.palette_hi))
                .field("palette_lo", &format_args!("{:#016b}", self.palette_lo))
                .field("tmp_tile_idx", &format_args!("{:02X}", self.tmp_tile_idx))
                .field("tmp_palette", &format_args!("{:02X}", self.tmp_palette))
                .field(
                    "tmp_pattern_hi",
                    &format_args!("{:02X}", self.tmp_pattern_hi),
                )
                .field(
                    "tmp_pattern_lo",
                    &format_args!("{:02X}", self.tmp_pattern_lo),
                )
                .finish()
        }
    }
}

impl Background {
    pub fn new() -> Self {
        Self {
            pattern_hi: 0,
            pattern_lo: 0,
            palette_hi: 0,
            palette_lo: 0,

            tmp_tile_idx: 0,
            tmp_palette: 0,
            tmp_pattern_hi: 0,
            tmp_pattern_lo: 0,
        }
    }

    pub fn load(&mut self) {
        self.pattern_hi =
            (self.pattern_hi & 0x00FF) | (self.tmp_pattern_hi.reverse_bits() as u16) << 8;
        self.pattern_lo =
            (self.pattern_lo & 0x00FF) | (self.tmp_pattern_lo.reverse_bits() as u16) << 8;

        let pal = self.tmp_palette;
        self.palette_hi = (self.palette_hi & 0x00FF) | if pal & 0b10 != 0 { 0xFF00 } else { 0x00 };
        self.palette_lo = (self.palette_lo & 0x00FF) | if pal & 0b01 != 0 { 0xFF00 } else { 0x00 };
    }

    pub fn pixel_at(&mut self, offset: u8) -> Pixel {
        let offset = offset as u16;
        Pixel::new(
            PixelKind::Background,
            self.palette(offset),
            self.color(offset),
            false,
        )
    }

    fn color(&self, offset: u16) -> u8 {
        let hi = (self.pattern_hi >> offset) & 0x01;
        let lo = (self.pattern_lo >> offset) & 0x01;
        (hi as u8) << 1 | lo as u8
    }

    fn palette(&self, offset: u16) -> u8 {
        let hi = (self.palette_hi >> offset) & 0x01;
        let lo = (self.palette_lo >> offset) & 0x01;
        (hi as u8) << 1 | lo as u8
    }

    pub fn shift(&mut self) {
        self.pattern_hi >>= 1;
        self.pattern_lo >>= 1;
        self.palette_hi >>= 1;
        self.palette_lo >>= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_pixel(pal: u8, col: u8) -> Pixel {
        Pixel::new(PixelKind::Background, pal, col, false)
    }

    #[test]
    fn test_bg_pixels() {
        let mut bg_pixels = Background::new();

        bg_pixels.tmp_pattern_hi = 0b1010_1010;
        bg_pixels.tmp_pattern_lo = 0b1111_0000;
        bg_pixels.tmp_palette = 0b0000_0011;
        bg_pixels.load();

        for _ in 0..6 {
            bg_pixels.shift();
        }

        bg_pixels.tmp_pattern_hi = 0b1111_0000;
        bg_pixels.tmp_pattern_lo = 0b1010_1010;
        bg_pixels.tmp_palette = 0b0000_0010;
        bg_pixels.load();

        for _ in 0..2 {
            bg_pixels.shift();
        }

        assert_eq!(bg_pixels.pixel_at(0), mk_pixel(3, 3));
        assert_eq!(bg_pixels.pixel_at(1), mk_pixel(3, 1));
        assert_eq!(bg_pixels.pixel_at(2), mk_pixel(3, 3));
        assert_eq!(bg_pixels.pixel_at(3), mk_pixel(3, 1));
        assert_eq!(bg_pixels.pixel_at(4), mk_pixel(3, 2));
        assert_eq!(bg_pixels.pixel_at(5), mk_pixel(3, 0));
        assert_eq!(bg_pixels.pixel_at(6), mk_pixel(2, 3));
        assert_eq!(bg_pixels.pixel_at(7), mk_pixel(2, 2));
        assert_eq!(bg_pixels.pixel_at(8), mk_pixel(2, 3));
        assert_eq!(bg_pixels.pixel_at(9), mk_pixel(2, 2));
        assert_eq!(bg_pixels.pixel_at(10), mk_pixel(2, 1));
        assert_eq!(bg_pixels.pixel_at(11), mk_pixel(2, 0));
        assert_eq!(bg_pixels.pixel_at(12), mk_pixel(2, 1));
        assert_eq!(bg_pixels.pixel_at(13), mk_pixel(2, 0));
        assert_eq!(bg_pixels.pixel_at(14), mk_pixel(0, 0));
    }
}
