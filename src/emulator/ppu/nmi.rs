pub struct Nmi {
    occurred: Option<()>,
    countdown: usize,
}

/*
 * According to the documentation, the NMI is triggered at scanline 241 dot 1, but if the signal
 * is sent to the CPU at this moment, the emulation will fail in all nmi timing tests.
 * It seems the signal should be delayed for some clocks (I don't know the reason yet).
 * 14 is the value that passes on ppu_vbl_nmi 05-nmi_timing
 */

impl Nmi {
    pub fn new() -> Self {
        Self {
            occurred: None,
            countdown: 0,
        }
    }

    pub fn schedule(&mut self) {
        self.countdown = 14; 
    }

    pub fn clock(&mut self) {
        if self.countdown > 0 {
            self.countdown -= 1;
            if self.countdown == 0 {
                self.occurred = Some(());
            }
        }
    }

    pub fn abort(&mut self) {
        self.countdown = 0;
    }

    pub fn take(&mut self) -> bool {
        self.occurred.take().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nmi() {
        let mut nmi = Nmi::new();
        assert!(!nmi.take());
        nmi.schedule();
        assert!(!nmi.take());
        for _ in 0..13 {
            nmi.clock();
            assert!(!nmi.take());
        }
        nmi.clock();
        assert!(nmi.take());
        assert!(!nmi.take());
        nmi.clock();
        assert!(!nmi.take());

        nmi.schedule();
        for _ in 0..13 {
            nmi.clock();
            assert!(!nmi.take());
        }
        nmi.abort();
        nmi.clock();
        assert!(!nmi.take());
    }
}
