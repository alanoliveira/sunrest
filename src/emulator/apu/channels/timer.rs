#[derive(Debug, Default, Clone, Copy)]
pub struct Timer {
    pub period: u16,
    pub counter: u16,
}

impl Timer {
    pub fn new(period: u16) -> Self {
        Self {
            period,
            counter: period,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.counter == 0
    }

    pub fn set_period_hi(&mut self, period: u8) {
        self.period = (self.period & 0x00FF) | ((period as u16) << 8);
    }

    pub fn set_period_lo(&mut self, period: u8) {
        self.period = (self.period & 0xFF00) | (period as u16);
    }

    pub fn increment_period(&mut self, inc: u16) {
        self.period = self.period.wrapping_add(inc);
    }

    pub fn reset(&mut self) {
        self.counter = self.period;
    }

    pub fn clock(&mut self) -> bool {
        if self.counter > 0 {
            self.counter -= 1;
        } else {
            self.reset();
        }
        self.counter == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let mut timer = Timer::new(3);

        assert_eq!(timer.counter, 3);
        assert_eq!(timer.period, 3);

        assert!(!timer.clock());
        assert_eq!(timer.counter, 2);

        assert!(!timer.clock());
        assert_eq!(timer.counter, 1);

        assert!(timer.clock());
        assert_eq!(timer.counter, 0);

        assert!(!timer.clock());
        assert_eq!(timer.counter, 3);

        assert!(!timer.clock());
        assert_eq!(timer.counter, 2);

        timer.reset();
        assert_eq!(timer.counter, 3);

        timer.period = 5;
        assert_eq!(timer.period, 5);
        assert_eq!(timer.counter, 3);
    }

    #[test]
    fn test_timer_hi_lo() {
        let mut timer = Timer::new(0);

        timer.set_period_hi(0x12);
        assert_eq!(timer.period, 0x1200);

        timer.set_period_lo(0x34);
        assert_eq!(timer.period, 0x1234);
    }
}
