use super::*;

pub struct Dmc {
    enabled: bool,
    repeat: bool,
    address: u16,
    length: u16,

    irq_enabled: bool,
    irq: Option<()>,

    // MemoryReader
    current_address: u16,
    bytes_remaining: u16,
    buffer: Option<u8>,

    // OutputUnit
    shift_register: u8,
    bits_remaining: u8,
    output_level: u8,
    silence: bool,

    timer: Timer,
}

impl Dmc {
    pub fn new() -> Self {
        Self {
            enabled: false,
            repeat: false,
            address: 0,
            length: 0,

            irq_enabled: false,
            irq: None,

            // MemoryReader
            current_address: 0,
            bytes_remaining: 0,
            buffer: Some(0),

            // OutputUnit
            shift_register: 0,
            bits_remaining: 0,
            output_level: 0,
            silence: true,

            timer: Timer::new(0),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => {
                self.irq_enabled = value & 0x80 != 0;
                self.repeat = value & 0x40 != 0;
                self.timer.set_period(TIMER_PERIOD[(value & 0x0F) as usize]);
            }
            0x01 => {
                self.output_level = value & 0x7F;
            }
            0x02 => {
                // Sample address = %11AAAAAA.AA000000 = $C000 + (A * 64)
                self.address = 0xC000 | ((value as u16) << 6);
            }
            0x03 => {
                // Sample length = %LLLL.LLLL0001 = (L * 16) + 1 bytes
                if self.enabled {
                    self.length = ((value as u16) << 4) | 1;
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn take_irq(&mut self) -> bool {
        self.irq.take().is_some()
    }

    pub fn enabled(&self) -> bool {
        self.bytes_remaining > 0
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.bytes_remaining = 0;
        } else if self.bytes_remaining == 0 {
            self.restart();
        }
    }

    pub fn is_waiting(&self) -> Option<u16> {
        if self.enabled() && self.buffer.is_none() {
            Some(self.current_address)
        } else {
            None
        }
    }

    pub fn load_sample_buffer(&mut self, val: u8) {
        if self.buffer.is_some() {
            panic!("DMC buffer is full");
        }

        self.buffer = Some(val);
        self.current_address = self.current_address.checked_add(1).unwrap_or(0x8000);
        self.bytes_remaining -= 1;
        if self.bytes_remaining == 0 {
            if self.repeat {
                self.restart();
            } else if self.irq_enabled {
                self.irq = Some(());
            }
        }
    }

    pub fn output(&self) -> u8 {
        self.output_level
    }

    pub fn clock_timer(&mut self) {
        if !self.enabled {
            return;
        }

        if self.timer.is_zero() {
            self.update_shift_register();
            self.bits_remaining -= 1;

            if self.bits_remaining == 0 {
                self.start_output_cycle();
            }
        }

        self.timer.clock();
    }

    fn update_shift_register(&mut self) {
        if self.silence {
            return;
        }

        self.output_level = if self.shift_register & 1 == 1 {
            (self.output_level as i8).saturating_add(2) as u8
        } else {
            self.output_level.saturating_sub(2)
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

    fn restart(&mut self) {
        self.bytes_remaining = self.length;
        self.current_address = self.address;
    }
}

const TIMER_PERIOD: [u16; 16] = [
    0xD6, 0xBE, 0xAA, 0xA0, 0x8F, 0x7F, 0x71, 0x6B, 0x5F, 0x50, 0x47, 0x40, 0x35, 0x2A, 0x24, 0x1B,
];
