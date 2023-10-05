use super::*;

const OAM_SIZE: usize = 0x100;

#[derive(Clone)]
pub struct Oam {
    mem: Box<[u8; OAM_SIZE]>,
}

impl Oam {
    pub fn new() -> Self {
        Self {
            mem: Box::new([0; OAM_SIZE]),
        }
    }

    pub fn read(&self, addr: u8) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u8, val: u8) {
        self.mem[addr as usize] = val;
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = RawSprite> + '_ {
        self.mem.chunks_exact(4).map(|chunk| {
            let y = chunk[0];
            let tile = chunk[1];
            let attr = chunk[2];
            let x = chunk[3];

            RawSprite::new(y, tile, attr, x)
        })
    }
}
