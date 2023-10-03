#[derive(Debug, Default, Clone, Copy)]
pub struct Length {
    pub(super) val: usize,
    pub(super) enabled: bool,
    pub(super) halted: bool,
}

impl Length {
    pub fn enabled(&self) -> bool {
        self.val > 0
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.reset();
        }
    }

    pub(super) fn clock(&mut self) {
        if self.val > 0 && !self.halted {
            self.val = self.val.wrapping_sub(1); // @TODO: what if underflow?
        }
    }

    pub(super) fn set(&mut self, val: usize) {
        if self.enabled {
            self.val = val;
        }
    }

    pub(super) fn set_by_index(&mut self, val: u8) {
        self.set(LENGTH_TABLE[val as usize]);
    }

    pub(super) fn reset(&mut self) {
        self.val = 0;
    }
}

const LENGTH_TABLE: [usize; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];
