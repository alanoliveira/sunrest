pub trait IO {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, val: u8);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum DmaState {
    Idle,
    Ready,
    Aligning,
    Running,
}

#[derive(Clone)]
pub struct OamDma {
    page: u8,
    index: u16,
    buffer: u8,
    state: DmaState,
}

impl OamDma {
    pub fn new() -> Self {
        Self {
            page: 0,
            index: 0,
            buffer: 0,
            state: DmaState::Idle,
        }
    }

    pub fn prepare(&mut self, page: u8) {
        if self.state != DmaState::Idle {
            log!("Attempted to start OAM DMA while it is already running");
        }

        self.page = page;
        self.index = 0;
        self.state = DmaState::Ready;
    }

    pub fn is_active(&self) -> bool {
        self.state != DmaState::Idle
    }

    pub fn write(&mut self, bus: &mut dyn IO) {
        match self.state {
            DmaState::Ready => self.state = DmaState::Running,
            DmaState::Aligning => self.state = DmaState::Running,
            DmaState::Running => {
                bus.write(self.buffer);
                if self.index > 0xFF {
                    self.state = DmaState::Idle;
                }
            }
            _ => {}
        }
    }

    pub fn read(&mut self, bus: &dyn IO) {
        match self.state {
            DmaState::Ready => self.state = DmaState::Aligning,
            DmaState::Running => {
                let addr = (self.page as u16) << 8 | self.index;
                self.buffer = bus.read(addr);
                self.index += 1;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestIO(u8);

    impl IO for TestIO {
        fn read(&self, addr: u16) -> u8 {
            addr as u8
        }

        fn write(&mut self, val: u8) {
            self.0 = val;
        }
    }

    #[test]
    fn test_oam_dma_starting_on_a_read_cycle() {
        let mut io = TestIO(0);
        let mut dma = OamDma::new();
        dma.prepare(0x12);
        assert_eq!(dma.state, DmaState::Ready);
        dma.read(&io);
        assert_eq!(
            dma.state,
            DmaState::Aligning,
            "there is an alignment cycle if it started on a read cycle"
        );
        dma.write(&mut io);
        assert_eq!(dma.state, DmaState::Running);
    }

    #[test]
    fn test_oam_dma_starting_on_a_write_cycle() {
        let mut io = TestIO(0);
        let mut dma = OamDma::new();
        dma.prepare(0x12);
        assert_eq!(dma.state, DmaState::Ready);
        dma.write(&mut io);
        assert_eq!(dma.state, DmaState::Running);
    }

    #[test]
    fn test_oam_dma() {
        let mut io = TestIO(0);
        let mut dma = OamDma::new();
        dma.state = DmaState::Running;
        dma.read(&io);
        dma.write(&mut io);
        assert_eq!(io.0, 0x00);
        dma.read(&io);
        dma.write(&mut io);
        assert_eq!(io.0, 0x01);
    }

    #[test]
    fn test_oam_dma_finishing() {
        let mut io = TestIO(0);
        let mut dma = OamDma::new();
        dma.state = DmaState::Running;
        dma.index = 0xFF;
        dma.read(&io);
        dma.write(&mut io);
        assert_eq!(dma.state, DmaState::Idle);
    }
}
