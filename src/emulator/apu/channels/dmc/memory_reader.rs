#[derive(Debug, Default)]
pub struct MemoryReader {
    length: u16,
    address: u16,
    bytes_remaining: u16,
    current_address: u16,
    repeat: bool,
}

pub struct NoMoreBytes;

impl MemoryReader {
    pub fn current_address(&self) -> u16 {
        self.current_address
    }

    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    pub fn enabled(&self) -> bool {
        self.bytes_remaining > 0
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        if !enabled {
            self.bytes_remaining = 0;
        } else if self.bytes_remaining == 0 {
            self.reset();
        }
    }

    pub fn set_address(&mut self, val: u8) {
        // Sample address = %11AAAAAA.AA000000 = $C000 + (A * 64)
        self.address = 0xC000 | ((val as u16) << 6);
    }

    pub fn set_length(&mut self, value: u8) {
        // Sample length = %LLLL.LLLL0001 = (L * 16) + 1 bytes
        self.length = ((value as u16) << 4) | 1;
    }

    pub fn increment_address(&mut self) -> Result<(), NoMoreBytes> {
        self.current_address = self.current_address.checked_add(1).unwrap_or(0x8000);
        self.bytes_remaining -= 1;
        if self.bytes_remaining == 0 {
            if !self.repeat {
                return Err(NoMoreBytes);
            }
            self.reset();
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.bytes_remaining = self.length;
        self.current_address = self.address;
    }
}
