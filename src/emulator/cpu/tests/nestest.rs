use super::{Status as S, *};

struct NestestIO([u8; 0x10000]);
impl Memory for NestestIO {
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
#[ignore]
fn nestest_test() {
    let mut io = NestestIO([0; 0x10000]);

    let nes_test_roms_path =
        std::env::var("NES_TEST_ROMS_PATH").expect("NES_TEST_ROMS_PATH not set");
    let path = std::path::PathBuf::from(nes_test_roms_path).join("other/nestest.nes");
    let nestest_rom_data = std::fs::read(path).unwrap();
    io.0[0x8000..0xC000].copy_from_slice(&nestest_rom_data[0x0010..0x4010]);
    let mut cpu = Cpu::new(io);
    cpu.pc = 0x8000;
    cpu.p = (S::U | S::I).into();
    cpu.cycle = 7;

    for _ in 0..=14571 {
        cpu.clock();
    }

    assert_eq!(cpu.pc, 0xC6BD, "CPU instruction timing is off");

    let ret1 = cpu.mem.read(0x00);
    let ret2 = cpu.mem.read(0x02);
    if ret1 | ret2 != 0x00 {
        panic!("Nestest failed: {:02X} {:02X}", ret1, ret2);
    }
}
