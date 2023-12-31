use super::*;

pub struct Debugger<'a, M: Memory>(pub &'a Ppu<M>);

#[allow(dead_code)]
impl<M: Memory> Debugger<'_, M> {
    pub fn print_nametable(&self, table_num: u8) {
        println!("Name table {}:", table_num);
        for row in 0..30 {
            for col in 0..32 {
                let tile = self.0.mem.read_nametable(table_num, col, row);
                print!("{:02X} ", tile);
            }
            println!();
        }
        println!();
    }

    pub fn print_attribute_table(&self, table_num: u8) {
        println!("Attribute table {}:", table_num);
        for row in 0..8 {
            for col in 0..8 {
                let attr = self.0.mem.read_attribute(table_num, col, row);
                print!("{:02X} ", attr);
            }
            println!();
        }
        println!();
    }

    pub fn print_palette_table(&self) {
        println!("Palette table:");
        for row in 0..2 {
            for col in 0..16 {
                let color = self.0.mem.read(0x3F00 + row * 16 + col);
                print!("{:02X} ", color);
                if col % 4 == 3 {
                    print!("  ");
                }
            }
            println!();
        }
        println!();
    }

    pub fn print_oam(&self) {
        println!("OAM:");
        for row in 0..8 {
            for col in 0..32 {
                let sprite = self.0.oam.read(row * 32 + col);
                print!("{:02X} ", sprite);
            }
            println!();
        }
        println!();
    }

    pub fn print_pattern_row_by_tile_addr(&self, tile_addr: u16, row: usize) {
        let row = row as u16;
        let mut pattern_hi = self.0.mem.read(tile_addr + row);
        let mut pattern_lo = self.0.mem.read(tile_addr + row + 8);

        print!("|");
        for _ in 0..8 {
            let color = ((pattern_hi & 0x80) >> 7) | ((pattern_lo & 0x80) >> 6);
            pattern_hi <<= 1;
            pattern_lo <<= 1;
            match color {
                0 => print!("  "),
                1 => print!("░░"),
                2 => print!("▒▒"),
                3 => print!("▓▓"),
                _ => unreachable!(),
            }
        }
        print!("|");
        println!();
    }

    pub fn print_pattern_row(&self, table: usize, pattern: (usize, usize), row: usize) {
        let mut pattern_addr = 0x1000 * table;
        pattern_addr += pattern.1 * 16;
        pattern_addr += pattern.0 * 16 * 16;
        self.print_pattern_row_by_tile_addr(pattern_addr as u16, row);
    }
}
