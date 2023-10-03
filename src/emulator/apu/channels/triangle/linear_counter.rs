#[derive(Debug, Default, Clone, Copy)]
pub struct LinearCounter {
    load: u8,
    current: u8,
    reload_flag: bool,
    control_flag: bool,
}

impl LinearCounter {
    pub fn set_control_flag(&mut self, value: bool) {
        self.control_flag = value;
    }

    pub fn set_load(&mut self, value: u8) {
        self.load = value;
    }

    pub fn set_reload_flag(&mut self, value: bool) {
        self.reload_flag = value;
    }

    pub fn output(&self) -> u8 {
        self.current
    }

    pub fn ended(&self) -> bool {
        self.current == 0
    }

    pub fn clock(&mut self) {
        if self.reload_flag {
            self.current = self.load;
        } else if self.current > 0 {
            self.current -= 1;
        }

        if !self.control_flag {
            self.reload_flag = false;
        }
    }
}
