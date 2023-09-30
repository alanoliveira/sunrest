#[macro_use]
mod util {
    use super::*;

    macro_rules! mk_io {
        ($($mem_addr:literal: $mem_val:expr),*) => {{
            let mut io = TestIO(vec![0; 0x10000]);
            io.0[0x0100..=0x01FF].copy_from_slice((0x00..=0xFF).collect::<Vec<_>>().as_slice()); // stack

            $( io.0[$mem_addr] = $mem_val; )*
            io
        }};
    }

    macro_rules! mk_cpu {
        ($io:expr, $($reg:ident: $val:expr),*) => {{
            let mut cpu = Cpu::new($io);
            $(
                cpu.$reg = $val.try_into().unwrap_or_else(|_| panic!("Invalid value for register {}", stringify!($reg)));
            )*
            cpu
        }};
    }

    macro_rules! assert_cpu {
        ($cpu:expr, {$($want_reg:ident: $want_val:expr),*}) => {{
            let cpu = &mut $cpu;

            loop {
                cpu.clock();

                if cpu.busy_cycles == 0 {
                    break;
                }
            }

            $( assert_eq!(cpu.$want_reg, $want_val.try_into().unwrap(), "cpu.{} failed", stringify!($want_reg));)*
        }};
    }

    pub fn opcode_lookup(inst: Instruction, addr_mode: AddressingMode) -> u8 {
        OPCODES
            .iter()
            .enumerate()
            .find_map(|(idx, opcode)| {
                if opcode.0 == inst && opcode.1 == addr_mode {
                    Some(idx as u8)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Invalid opcode: {:?} {:?}", inst, addr_mode))
    }
}

mod disasm;
mod instructions;
mod nestest;

use super::{AddressingMode as AM, Instruction as IN, Status as S, *};

#[derive(Clone)]
struct TestIO(Vec<u8>);

impl Memory for TestIO {
    fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.0[addr as usize] = data;
    }
}

#[test]
fn test_detour() {
    let mut cpu = mk_cpu!(mk_io!(0x3000: 0xBB, 0x3001: 0xAA), pc: 0x8000);
    cpu.detour(0x3000);
    assert_eq!(cpu.pc, 0xAABB);
}

#[test]
fn test_reset() {
    let mut cpu = mk_cpu!(mk_io!(0xFFFC: 0xBB, 0xFFFD: 0xAA), pc: 0x8000);
    cpu.reset();
    assert_eq!(cpu.pc, 0xAABB);
    assert_eq!(cpu.sp, 0xFD);
    assert_eq!(cpu.p, (S::U | S::I).into());
}

#[test]
fn test_rst_signal() {
    let mut cpu = mk_cpu!(mk_io!(0xAABB: util::opcode_lookup(IN::Nop, AM::Imp), 0xFFFC: 0xBB, 0xFFFD: 0xAA), pc: 0x8000, sp: 0x40, p: S::U | S::B | S::N);
    cpu.set_signal(Signal::Rst);
    assert_cpu!(cpu, {pc: 0xAABB, sp: 0xFD, p: S::U | S::I, cycle: 7});
    assert_cpu!(cpu, {pc: 0xAABC, sp: 0xFD, p: S::U | S::I, cycle: 9});
}

#[test]
fn test_irq_signal() {
    let nop = util::opcode_lookup(IN::Nop, AM::Imp);
    let mut cpu = mk_cpu!(mk_io!(0xAABB: nop, 0xFFFE: 0xBB, 0xFFFF: 0xAA), pc: 0x8000, sp: 0x40, p: S::U | S::Z | S::N);
    cpu.set_signal(Signal::Irq);
    assert_cpu!(cpu, {pc: 0xAABB, sp: 0x3D, p: S::U | S::Z | S::N, cycle: 6});
    assert_cpu!(cpu, {pc: 0xAABC, sp: 0x3D, p: S::U | S::Z | S::N, cycle: 8});

    let mut cpu = mk_cpu!(mk_io!(0x8000: nop, 0xFFFE: 0xBB, 0xFFFF: 0xAA), pc: 0x8000, sp: 0x40, p: S::U | S::Z | S::N | S::I);
    cpu.set_signal(Signal::Irq);
    assert_cpu!(cpu, {pc: 0x8001, sp: 0x40, p: S::U | S::Z | S::N | S::I, cycle: 2});
}

#[test]
fn test_nmi_signal() {
    let nop = util::opcode_lookup(IN::Nop, AM::Imp);
    let mut cpu = mk_cpu!(mk_io!(0xAABB: nop, 0xFFFA: 0xBB, 0xFFFB: 0xAA), pc: 0x8000, sp: 0x40, p: S::U | S::Z | S::N);
    cpu.set_signal(Signal::Nmi);
    assert_cpu!(cpu, {pc: 0xAABB, sp: 0x3D, p: S::U | S::Z | S::N, cycle: 7});
    assert_cpu!(cpu, {pc: 0xAABC, sp: 0x3D, p: S::U | S::Z | S::N, cycle: 9});

    let mut cpu = mk_cpu!(mk_io!(0xAABB: nop, 0xFFFA: 0xBB, 0xFFFB: 0xAA), pc: 0x8000, sp: 0x40, p: S::U | S::Z | S::N | S::I);
    cpu.set_signal(Signal::Nmi);
    assert_cpu!(cpu, {pc: 0xAABB, sp: 0x3D, p: S::U | S::Z | S::N | S::I, cycle: 7});
    assert_cpu!(cpu, {pc: 0xAABC, sp: 0x3D, p: S::U | S::Z | S::N | S::I, cycle: 9});
}
