use super::*;

const CYCLES_PER_FRAME: usize = DOTS_PER_LINE * LINES_PER_FRAME;

impl Ppu {
    fn sync_vblank(&mut self) {
        let mut count = 0;
        loop {
            self.clock();
            if self.io().read_status().vertical_blank {
                break;
            }
            count += 1;
            assert!(count < self.cycles_per_frame(), "VBLANK not reached");
        }
    }

    fn sync_vblank_delay(&mut self, delay: usize) {
        self.sync_vblank();
        self.clock_n(self.cycles_per_frame().checked_sub(delay).unwrap());
    }

    fn clock_n(&mut self, n: usize) {
        for _ in 0..n {
            self.clock();
        }
    }

    fn cycles_per_frame(&self) -> usize {
        CYCLES_PER_FRAME
    }
}

struct DummyCartridgeIO;

impl bus::CartridgeIO for DummyCartridgeIO {
    fn read(&self, addr: u16) -> u8 {
        addr as u8
    }
}

fn mk_ppu() -> Ppu {
    let bus = bus::Bus::new(Box::new(DummyCartridgeIO));
    Ppu::new(bus)
}

#[test]
fn test_vblank_period() {
    let mut ppu = mk_ppu();

    ppu.sync_vblank_delay(1);
    assert!(!ppu.io().read_status().vertical_blank, "VBLANK too early");

    ppu.clock();
    assert!(ppu.io().read_status().vertical_blank, "VBLANK not reached");
}

#[test]
fn test_nmi_activation() {
    let mut ppu = mk_ppu();

    ppu.io().write_ctrl(io::Control { nmi_enabled: false });
    ppu.sync_vblank();
    assert!(!ppu.take_nmi(), "NMI should not happen when disabled");

    ppu.io().write_ctrl(io::Control { nmi_enabled: true });
    ppu.sync_vblank();
    assert!(ppu.take_nmi());
}
