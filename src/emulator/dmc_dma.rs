use super::*;

#[derive(PartialEq, Eq)]
enum DmaState {
    Idle,
    Ready,
    Aligning,
    Running,
}

pub struct DmcDma {
    state: DmaState,
    address: u16,
    pub buffer: u8,
}

impl DmcDma {
    pub fn new() -> Self {
        Self {
            state: DmaState::Idle,
            address: 0,
            buffer: 0,
        }
    }

    pub fn prepare(&mut self, address: u16) {
        if self.state != DmaState::Idle {
            log!("Attempted to start DMC DMA while it is already running");
        }

        self.address = address;
        self.state = DmaState::Ready;
    }

    pub fn is_active(&self) -> bool {
        self.state != DmaState::Idle
    }

    pub fn dummy(&mut self) {
        match self.state {
            DmaState::Ready | DmaState::Aligning => {
                self.state = DmaState::Running;
            }
            _ => {}
        }
    }

    pub fn read(&mut self, bus: &bus::Bus) {
        match self.state {
            DmaState::Ready => {
                self.state = DmaState::Aligning;
            }
            DmaState::Running => {
                self.buffer = bus.read(self.address);
                self.state = DmaState::Idle;
            }
            _ => {}
        }
    }
}
