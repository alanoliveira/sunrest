use super::{Status as S, *};

struct NestestIO([u8; 0x10000]);
impl IO for NestestIO {
    fn read(&self, addr: u16) -> u8 {
        let addr = match addr {
            0xC000..=0xFFFF => addr - 0x4000,
            _ => addr,
        };
        self.0[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.0[addr as usize] = data;
    }
}

#[test]
fn nestest_test() {
    let mut io = NestestIO([0; 0x10000]);
    let nestest_prg: &[u8] = include!("nestest.in");
    io.0[0x8000..0x8000 + nestest_prg.len()].copy_from_slice(nestest_prg);
    let mut cpu = Cpu::new(io);
    cpu.pc = 0x8000;
    cpu.p = (S::U | S::I).into();
    cpu.cycle = 7;

    loop {
        cpu.clock();

        // after this address it will start to test unofficial opcodes
        if cpu.pc == 0xC6BD {
            break;
        }

        if cpu.cycle > 20000 {
            panic!("Nestest is taking too much cycles");
        }
    }

    let ret1 = cpu.io.read(0x00);
    let ret2 = cpu.io.read(0x02);
    if ret1 | ret2 != 0x00 {
        panic!("Nestest failed: {:02X} {:02X}", ret1, ret2);
    }
}
