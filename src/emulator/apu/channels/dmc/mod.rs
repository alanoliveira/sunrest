mod memory_reader;
mod output_unit;

use super::*;
use memory_reader::*;
use output_unit::*;

#[derive(Clone)]
pub struct Dmc {
    output_unit: OutputUnit,
    pub memory_reader: MemoryReader,

    timer: Timer,
    irq_enabled: bool,
}

impl Dmc {
    pub fn new() -> Self {
        Self {
            output_unit: OutputUnit::default(),
            memory_reader: MemoryReader::default(),
            irq_enabled: false,

            timer: Timer::new(0),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x00 => {
                self.irq_enabled = value & 0x80 != 0;
                self.memory_reader.set_repeat(value & 0x40 != 0);
                self.timer.period = TIMER_PERIOD[(value & 0x0F) as usize];
            }
            0x01 => self.output_unit.set_level(value & 0x7F),
            0x02 => self.memory_reader.set_address(value),
            0x03 => self.memory_reader.set_length(value),
            _ => unreachable!(),
        }
    }

    pub fn is_waiting(&self) -> Option<u16> {
        if self.memory_reader.enabled() && self.output_unit.starved() {
            Some(self.memory_reader.current_address())
        } else {
            None
        }
    }

    pub fn load_sample_buffer(&mut self, val: u8) {
        self.output_unit.feed(val);
        if self.memory_reader.increment_address().is_err() && self.irq_enabled {
            // IRQ
        }
    }

    pub fn output(&self) -> u8 {
        self.output_unit.level()
    }

    pub fn clock_timer(&mut self) {
        if !self.memory_reader.enabled() {
            return;
        }

        if self.timer.is_zero() {
            self.output_unit.clock();
        }

        self.timer.clock();
    }
}

const TIMER_PERIOD: [u16; 16] = [
    0xD6, 0xBE, 0xAA, 0xA0, 0x8F, 0x7F, 0x71, 0x6B, 0x5F, 0x50, 0x47, 0x40, 0x35, 0x2A, 0x24, 0x1B,
];
