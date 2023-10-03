const NAMETABLE_BASE_ADDR: u16 = 0x2000;
const ATTRIBUTE_BASE_ADDR: u16 = 0x23C0;
const PALETTE_BASE_ADDR: u16 = 0x3F00;

pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);

    fn read_nametable(&self, table: u8, row: u8, col: u8) -> u8 {
        let addr =
            NAMETABLE_BASE_ADDR | ((table as u16) << 10) | ((row as u16) << 5) | (col as u16);
        self.read(addr)
    }

    fn read_attribute(&self, table: u8, row: u8, col: u8) -> u8 {
        let addr = ATTRIBUTE_BASE_ADDR
            | (table as u16) << 10
            | (col as u16) >> 2
            | ((row as u16) & !0x3) << 1;
        self.read(addr)
    }

    fn read_attribute_palette(&self, table: u8, row: u8, col: u8) -> u8 {
        let attr = self.read_attribute(table, row, col);
        let palette_shift = ((row & 0x02) << 1) | (col & 0x02);
        (attr >> palette_shift) & 0x03
    }

    fn read_palette(&self, table: u8, palette: u8, color: u8) -> u8 {
        let addr =
            PALETTE_BASE_ADDR | (((table as u16) << 4) + ((palette as u16) << 2)) | (color as u16);
        self.read(addr)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestMemory(u16, u8);

    impl Memory for TestMemory {
        fn read(&self, addr: u16) -> u8 {
            if addr == self.0 {
                self.1
            } else {
                0xFF
            }
        }

        fn write(&mut self, _: u16, _: u8) {}
    }

    #[test]
    fn test_read_nametable() {
        assert_eq!(TestMemory(0x2000, 1).read_nametable(0, 0, 0), 1);
        assert_eq!(TestMemory(0x2005, 1).read_nametable(0, 0, 5), 1);
        assert_eq!(TestMemory(0x20A0, 1).read_nametable(0, 5, 0), 1);
        assert_eq!(TestMemory(0x23BF, 1).read_nametable(0, 29, 31), 1);
        assert_eq!(TestMemory(0x2400, 1).read_nametable(1, 0, 0), 1);
        assert_eq!(TestMemory(0x2576, 1).read_nametable(1, 11, 22), 1);
        assert_eq!(TestMemory(0x2FBF, 1).read_nametable(3, 29, 31), 1);
    }

    #[test]
    fn test_read_attribute() {
        assert_eq!(TestMemory(0x23C0, 1).read_attribute(0, 0, 0), 1);
        assert_eq!(TestMemory(0x23FF, 1).read_attribute(0, 31, 29), 1);
        assert_eq!(TestMemory(0x27C0, 1).read_attribute(1, 0, 0), 1);
        assert_eq!(TestMemory(0x27FF, 1).read_attribute(1, 31, 29), 1);
        assert_eq!(TestMemory(0x2BC0, 1).read_attribute(2, 0, 0), 1);
        assert_eq!(TestMemory(0x2BFF, 1).read_attribute(2, 31, 29), 1);
        assert_eq!(TestMemory(0x2FC0, 1).read_attribute(3, 0, 0), 1);
        assert_eq!(TestMemory(0x2FFF, 1).read_attribute(3, 31, 29), 1);

        macro_rules! test_attr_group {
            ($addr:expr, $t:expr, $y:expr, $x:expr) => {
                for row in 0..4 {
                    for col in 0..4 {
                        assert_eq!(
                            TestMemory($addr, 1).read_attribute($t, $y + row, $x + col),
                            1
                        );
                    }
                }
            };
        }

        test_attr_group!(0x23C0, 0, 0, 0);
        test_attr_group!(0x27D9, 1, 12, 4);
        test_attr_group!(0x2BC1, 2, 0, 4);
        test_attr_group!(0x2FEB, 3, 20, 12);
    }

    #[test]
    fn test_read_attribute_palette() {
        macro_rules! test_pal_group {
            ($addr:expr, $t:expr, $y:expr, $x:expr, $want:expr) => {
                for row in 0..2 {
                    for col in 0..2 {
                        assert_eq!(
                            TestMemory($addr, 0b11_10_01_00).read_attribute_palette(
                                $t,
                                $y + row,
                                $x + col
                            ),
                            $want
                        );
                    }
                }
            };
        }

        test_pal_group!(0x23C0, 0, 0, 0, 0);
        test_pal_group!(0x23C0, 0, 0, 2, 1);
        test_pal_group!(0x23C0, 0, 2, 0, 2);
        test_pal_group!(0x23C0, 0, 2, 2, 3);
    }
}
