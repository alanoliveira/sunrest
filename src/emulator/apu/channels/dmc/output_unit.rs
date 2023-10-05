#[derive(Debug, Default, Clone)]
pub struct OutputUnit {
    shift_register: u8,
    bits_remaining: u8,
    level: u8,
    silence: bool,
    buffer: Option<u8>,
}

impl OutputUnit {
    pub fn starved(&self) -> bool {
        self.buffer.is_none()
    }

    pub fn feed(&mut self, val: u8) {
        if self.buffer.is_some() {
            panic!("DMC buffer is full");
        }

        self.buffer = Some(val);
    }

    pub fn set_level(&mut self, val: u8) {
        self.level = val & 0x7F;
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn clock(&mut self) {
        self.update_shift_register();
        self.bits_remaining -= 1;

        if self.bits_remaining == 0 {
            self.start_output_cycle();
        }
    }

    fn update_shift_register(&mut self) {
        if self.silence {
            return;
        }

        self.level = if self.shift_register & 1 == 1 {
            (self.level as i8).saturating_add(2) as u8
        } else {
            self.level.saturating_sub(2)
        };

        self.shift_register >>= 1;
    }

    fn start_output_cycle(&mut self) {
        self.bits_remaining = 8;
        match self.buffer.take() {
            Some(value) => {
                self.silence = false;
                self.shift_register = value;
            }
            None => {
                self.silence = true;
            }
        }
    }
}
