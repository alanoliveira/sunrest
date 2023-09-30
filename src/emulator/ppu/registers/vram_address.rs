#[derive(Default, Clone, Copy)]
pub struct VramAddress(pub u16);

impl std::fmt::Debug for VramAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:016b} (coarse_x: {}, coarse_y: {}, nametable_h: {}, nametable_v: {}, fine_y: {})",
                self.0,
                self.coarse_x(),
                self.coarse_y(),
                self.nametable_h(),
                self.nametable_v(),
                self.fine_y(),
            )
        } else {
            write!(f, "{:04X}", self.0)
        }
    }
}

impl VramAddress {
    const COARSE_X: u16 = 0b0000_0000_0001_1111;
    const COARSE_Y: u16 = 0b0000_0011_1110_0000;
    const NAMETABLE_H: u16 = 0b0000_0100_0000_0000;
    const NAMETABLE_V: u16 = 0b0000_1000_0000_0000;
    const FINE_Y: u16 = 0b0111_0000_0000_0000;

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, val: u16) {
        self.0 = val;
    }

    pub fn coarse_x(&self) -> u8 {
        get_val(self.0, Self::COARSE_X) as u8
    }

    pub fn coarse_y(&self) -> u8 {
        get_val(self.0, Self::COARSE_Y) as u8
    }

    pub fn nametable_h(&self) -> u8 {
        get_val(self.0, Self::NAMETABLE_H) as u8
    }

    pub fn nametable_v(&self) -> u8 {
        get_val(self.0, Self::NAMETABLE_V) as u8
    }

    pub fn nametable(&self) -> u8 {
        get_val(self.0, Self::NAMETABLE_H | Self::NAMETABLE_V) as u8
    }

    pub fn fine_y(&self) -> u8 {
        get_val(self.0, Self::FINE_Y) as u8
    }

    pub fn set_coarse_x(&mut self, val: u8) {
        self.0 = set_val(self.0, Self::COARSE_X, val as u16);
    }

    pub fn set_coarse_y(&mut self, val: u8) {
        self.0 = set_val(self.0, Self::COARSE_Y, val as u16);
    }

    pub fn set_nametable_h(&mut self, val: u8) {
        self.0 = set_val(self.0, Self::NAMETABLE_H, val as u16);
    }

    pub fn set_nametable_v(&mut self, val: u8) {
        self.0 = set_val(self.0, Self::NAMETABLE_V, val as u16);
    }

    pub fn set_nametable(&mut self, val: u8) {
        self.0 = set_val(self.0, Self::NAMETABLE_H | Self::NAMETABLE_V, val as u16);
    }

    pub fn set_fine_y(&mut self, val: u8) {
        self.0 = set_val(self.0, Self::FINE_Y, val as u16);
    }

    pub fn set_lb(&mut self, val: u8) {
        self.0 = set_val(self.0, 0xFF, val as u16);
    }

    pub fn set_hb(&mut self, val: u8) {
        self.0 = set_val(self.0, 0xFF00, val as u16);
    }

    pub fn increment(&mut self, val: u16) {
        self.0 = self.0.wrapping_add(val);
    }

    pub fn increment_x(&mut self) {
        let mut coarse_x = self.coarse_x() + 1;
        if coarse_x == 32 {
            coarse_x = 0;
            self.set_nametable_h(self.nametable_h() ^ 1);
        }
        self.set_coarse_x(coarse_x);
    }

    pub fn increment_y(&mut self) {
        let mut fine_y = self.fine_y() + 1;
        if fine_y > 7 {
            fine_y = 0;
            let mut coarse_y = self.coarse_y() + 1;
            if coarse_y == 30 {
                coarse_y = 0;
                self.set_nametable_v(self.nametable_v() ^ 1);
            } else if coarse_y == 32 {
                coarse_y = 0;
            }
            self.set_coarse_y(coarse_y);
        }
        self.set_fine_y(fine_y);
    }
}

impl From<u16> for VramAddress {
    fn from(val: u16) -> Self {
        Self(val)
    }
}

fn get_val(val: u16, mask: u16) -> u16 {
    (val & mask) >> mask.trailing_zeros()
}

fn set_val(val: u16, mask: u16, new_val: u16) -> u16 {
    (val & !mask) | (new_val << mask.trailing_zeros())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw() {
        assert_eq!(VramAddress(0b1111_1111_1110_0000).coarse_x(), 0);
        assert_eq!(VramAddress(0b0000_0000_0001_0000).coarse_x(), 16);
        assert_eq!(VramAddress(0b0000_0000_0001_1111).coarse_x(), 31);
    }

    #[test]
    fn test_coarse_x() {
        assert_eq!(VramAddress(0b1111_1111_1110_0000).coarse_x(), 0);
        assert_eq!(VramAddress(0b0000_0000_0001_0000).coarse_x(), 16);
        assert_eq!(VramAddress(0b0000_0000_0001_1111).coarse_x(), 31);
    }

    #[test]
    fn test_coarse_y() {
        assert_eq!(VramAddress(0b1111_1100_0001_1111).coarse_y(), 0);
        assert_eq!(VramAddress(0b0000_0000_0010_0000).coarse_y(), 1);
        assert_eq!(VramAddress(0b0000_0011_1110_0000).coarse_y(), 31);
    }

    #[test]
    fn test_nametable_h() {
        assert_eq!(VramAddress(0b1111_1011_1111_1111).nametable_h(), 0);
        assert_eq!(VramAddress(0b0000_0100_0000_0000).nametable_h(), 1);
    }

    #[test]
    fn test_nametable_v() {
        assert_eq!(VramAddress(0b1111_0111_1111_1111).nametable_v(), 0);
        assert_eq!(VramAddress(0b0000_1000_0000_0000).nametable_v(), 1);
    }

    #[test]
    fn test_nametable() {
        assert_eq!(VramAddress(0b1111_0011_1111_1111).nametable(), 0);
        assert_eq!(VramAddress(0b0000_1100_0000_0000).nametable(), 3);
    }

    #[test]
    fn test_fine_y() {
        assert_eq!(VramAddress(0b1000_1111_1111_1111).fine_y(), 0);
        assert_eq!(VramAddress(0b0111_0000_0000_0000).fine_y(), 7);
    }

    macro_rules! assert_change {
        ($ini_val:expr, $f:ident($($args:expr),*), $want:expr) => {
            let mut vram_addr = VramAddress($ini_val);
            vram_addr.$f($($args),*);
            assert_eq!(vram_addr.0, $want, "{:016b} != {:016b}", vram_addr.0, $want);
        };
    }

    #[test]
    fn test_set_coarse_x() {
        assert_change!(
            0b1111_1111_1111_1111,
            set_coarse_x(0),
            0b1111_1111_1110_0000
        );
        assert_change!(
            0b0000_0000_0000_0000,
            set_coarse_x(16),
            0b0000_0000_0001_0000
        );
    }

    #[test]
    fn test_set_coarse_y() {
        assert_change!(
            0b1111_1111_1111_1111,
            set_coarse_y(0),
            0b1111_1100_0001_1111
        );
        assert_change!(
            0b0000_0000_0000_0000,
            set_coarse_y(1),
            0b0000_0000_0010_0000
        );
    }

    #[test]
    fn test_set_nametable_h() {
        assert_change!(
            0b1111_1111_1111_1111,
            set_nametable_h(0),
            0b1111_1011_1111_1111
        );
        assert_change!(
            0b0000_0000_0000_0000,
            set_nametable_h(1),
            0b0000_0100_0000_0000
        );
    }

    #[test]
    fn test_set_nametable_v() {
        assert_change!(
            0b1111_1111_1111_1111,
            set_nametable_v(0),
            0b1111_0111_1111_1111
        );
        assert_change!(
            0b0000_0000_0000_0000,
            set_nametable_v(1),
            0b0000_1000_0000_0000
        );
    }

    #[test]
    fn test_set_nametable() {
        assert_change!(
            0b1111_1111_1111_1111,
            set_nametable(0),
            0b1111_0011_1111_1111
        );
        assert_change!(
            0b0000_0000_0000_0000,
            set_nametable(3),
            0b0000_1100_0000_0000
        );
    }

    #[test]
    fn test_set_fine_y() {
        assert_change!(0b1111_1111_1111_1111, set_fine_y(0), 0b1000_1111_1111_1111);
        assert_change!(0b0000_0000_0000_0000, set_fine_y(7), 0b0111_0000_0000_0000);
    }

    #[test]
    fn test_set_lb() {
        assert_change!(0b1111_1111_1111_1111, set_lb(0), 0b1111_1111_0000_0000);
        assert_change!(0b0000_0000_0000_0000, set_lb(0), 0b0000_0000_0000_0000);
    }

    #[test]
    fn test_set_hb() {
        assert_change!(0b1111_1111_1111_1111, set_hb(0), 0b0000_0000_1111_1111);
        assert_change!(0b0000_0000_0000_0000, set_hb(0), 0b0000_0000_0000_0000);
    }

    #[test]
    fn test_increment() {
        assert_change!(0b1111_1111_1111_1111, increment(0), 0b1111_1111_1111_1111);
        assert_change!(0b1111_1111_1111_1111, increment(1), 0b0000_0000_0000_0000);
        assert_change!(0b0000_0000_0000_0000, increment(1), 0b0000_0000_0000_0001);
        assert_change!(0b0000_0000_0001_0000, increment(16), 0b0000_0000_0010_0000);
    }

    #[test]
    fn test_increment_x() {
        // coarse_x < 31
        assert_change!(0b0000_0100_0001_1110, increment_x(), 0b0000_0100_0001_1111);
        // coarse_x == 31
        assert_change!(0b0000_0100_0001_1111, increment_x(), 0b0000_0000_0000_0000);
    }

    #[test]
    fn test_increment_y() {
        // fine_y <= 7
        assert_change!(0b0110_0011_1110_0000, increment_y(), 0b0111_0011_1110_0000);
        // fine_y > 7 && coarse_y < 29
        assert_change!(0b0111_0011_1000_0000, increment_y(), 0b0000_0011_1010_0000);
        // fine_y > 7 && coarse_y == 29
        assert_change!(0b0111_0011_1010_0000, increment_y(), 0b0000_1000_0000_0000);
        // fine_y > 7 && coarse_y == 31
        assert_change!(0b0111_0011_1110_0000, increment_y(), 0b0000_0000_0000_0000);
    }
}
