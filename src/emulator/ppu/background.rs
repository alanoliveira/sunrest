use super::*;

const BUFFER_SIZE: usize = 16;

pub struct Background {
    pixels: [Pixel; BUFFER_SIZE],
    head: usize,
    tail: usize,

    pub tmp_tile_idx: u8,
    pub tmp_palette: u8,
    pub tmp_pattern_hi: u8,
    pub tmp_pattern_lo: u8,
}

impl Background {
    pub fn new() -> Self {
        Self {
            pixels: [Pixel::default(); BUFFER_SIZE],
            head: 0,
            tail: 0,

            tmp_tile_idx: 0,
            tmp_palette: 0,
            tmp_pattern_hi: 0,
            tmp_pattern_lo: 0,
        }
    }

    pub fn load(&mut self) {
        let mut pattern_hi = self.tmp_pattern_hi;
        let mut pattern_lo = self.tmp_pattern_lo;

        self.tail = (self.head + 8) % BUFFER_SIZE;
        for _ in 0..8 {
            let color = ((pattern_hi & 0x80) >> 6) | ((pattern_lo & 0x80) >> 7);
            pattern_hi <<= 1;
            pattern_lo <<= 1;
            let pixel = Pixel::new(PixelKind::Background, self.tmp_palette, color, false);
            self.add_pixel(pixel);
        }
    }

    pub fn next_pixel(&mut self, offset: usize) -> Pixel {
        let index = (self.head + offset) % BUFFER_SIZE;
        self.head = (self.head + 1) % BUFFER_SIZE;
        self.pixels[index]
    }

    fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels[self.tail] = pixel;
        self.tail = (self.tail + 1) % BUFFER_SIZE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bg_pixels() {
        let mut bg_pixels = Background::new();

        bg_pixels.tmp_pattern_hi = 0b1010_1010;
        bg_pixels.tmp_pattern_lo = 0b1111_0000;
        bg_pixels.tmp_palette = 0b0000_0011;
        bg_pixels.load();

        for _ in 0..8 {
            assert_eq!(bg_pixels.next_pixel(0), Pixel::default());
        }
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 3, 3, false));
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 3, 1, false));
        assert_eq!(bg_pixels.next_pixel(2), Pixel::new(PixelKind::Background, 3, 2, false));
        assert_eq!(bg_pixels.next_pixel(2), Pixel::new(PixelKind::Background, 3, 0, false));
        bg_pixels.tmp_pattern_hi = 0b1111_0000;
        bg_pixels.tmp_pattern_lo = 0b1010_1010;
        bg_pixels.tmp_palette = 0b0000_0010;
        bg_pixels.load();
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 3, 2, false));
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 3, 0, false));
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 3, 2, false));
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 3, 0, false));
        for _ in 0..4 {
            assert_eq!(bg_pixels.next_pixel(0), Pixel::default());
        }
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 2, 3, false));
        assert_eq!(bg_pixels.next_pixel(0), Pixel::new(PixelKind::Background, 2, 2, false));
    }
}
