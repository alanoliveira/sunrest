#[derive(Debug, Default, Clone, Copy)]
pub struct Vblank(bool);

impl Vblank {
    pub fn set(&mut self, val: bool) {
        self.0 = val;
    }

    pub fn get(&mut self) -> bool {
        let ret = self.0;
        self.0 = false;
        ret
    }
}
