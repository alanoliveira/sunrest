mod disasm;
mod nestest;

use super::{AddressingMode as AM, Instruction as IN, Status as S, *};

#[derive(Clone)]
struct TestIO(Vec<u8>);

impl IO for TestIO {
    fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.0[addr as usize] = data;
    }
}

fn opcode_lookup(inst: Instruction, addr_mode: AddressingMode) -> u8 {
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

macro_rules! test_instruction {
    ($ins:expr, $mode:expr, [$($args:expr),*], $cpu:expr, {$($want_reg:ident: $want_val:expr),*}) => {{
        let mut cpu = $cpu;
        let prg = &[opcode_lookup($ins, $mode), $($args,)*];
        prg.iter().enumerate().for_each(|(idx, val)| {
            cpu.io.write(cpu.pc + idx as u16, *val);
        });

        loop {
            cpu.clock();

            if cpu.busy_cycles == 0 {
                break;
            }
        }

        $( assert_eq!(cpu.$want_reg, $want_val.try_into().unwrap(), "cpu.{} failed", stringify!($want_reg));)*
    }};
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
fn test_imp_instructions() {
    test_instruction!(IN::Nop, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000), {pc: 0x8001, cycle: 2});
    test_instruction!(IN::Tax, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10), {pc: 0x8001, x: 0x10, cycle: 2});
    test_instruction!(IN::Txa, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x20), {pc: 0x8001, a: 0x20, cycle: 2});
    test_instruction!(IN::Dex, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x20), {pc: 0x8001, x: 0x1F, cycle: 2});
    test_instruction!(IN::Inx, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x20), {pc: 0x8001, x: 0x21, cycle: 2});
    test_instruction!(IN::Tay, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10), {pc: 0x8001, y: 0x10, cycle: 2});
    test_instruction!(IN::Tya, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x20), {pc: 0x8001, a: 0x20, cycle: 2});
    test_instruction!(IN::Tsx, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x40), {pc: 0x8001, x: 0x40, cycle: 2});
    test_instruction!(IN::Txs, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x20), {pc: 0x8001, sp: 0x20, cycle: 2});
    test_instruction!(IN::Dey, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x20), {pc: 0x8001, y: 0x1F, cycle: 2});
    test_instruction!(IN::Iny, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x20), {pc: 0x8001, y: 0x21, cycle: 2});
    test_instruction!(IN::Pha, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x40, sp: 0x3F), {pc: 0x8001, sp: 0x3E, cycle: 3});
    test_instruction!(IN::Php, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x3F), {pc: 0x8001, sp: 0x3E, cycle: 3});
    test_instruction!(IN::Pla, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x80, a: 0x10, p: S::U), {pc: 0x8001, sp: 0x81, a: 0x81, p: S::U | S::N, cycle: 4});
    test_instruction!(IN::Plp, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x80, p: S::U), {pc: 0x8001, sp: 0x81, p: S::U | 0x81, cycle: 4});
    test_instruction!(IN::Rti, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x40, p: S::U), {pc: 0x4342, sp: 0x43, p: S::U | S::V | S::C, cycle: 6});
    test_instruction!(IN::Rts, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x40), {pc: 0x4242, sp: 0x42, cycle: 6});
    test_instruction!(IN::Brk, AM::Imp, [], mk_cpu!(mk_io!(0xFFFE: 0xCD, 0xFFFF: 0xAB), pc: 0x8000, sp: 0x40, p: S::U), {pc: 0xABCD, sp: 0x3B, p: S::U | S::B | S::I, cycle: 7});
    test_instruction!(IN::Clc, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U | S::C), {pc: 0x8001, p: S::U, cycle: 2});
    test_instruction!(IN::Cld, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U | S::D), {pc: 0x8001, p: S::U, cycle: 2});
    test_instruction!(IN::Cli, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U | S::I), {pc: 0x8001, p: S::U, cycle: 2});
    test_instruction!(IN::Clv, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U | S::V), {pc: 0x8001, p: S::U, cycle: 2});
    test_instruction!(IN::Sec, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U), {pc: 0x8001, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Sed, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U), {pc: 0x8001, p: S::U | S::D, cycle: 2});
    test_instruction!(IN::Sei, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U), {pc: 0x8001, p: S::U | S::I, cycle: 2});
    test_instruction!(IN::Asl, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x40, p: S::U), {pc: 0x8001, a: 0x80, p: S::U | S::N, cycle: 2});
    test_instruction!(IN::Asl, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x80, p: S::U), {pc: 0x8001, a: 0x00, p: S::U | S::C | S::Z, cycle: 2});
    test_instruction!(IN::Lsr, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x40, p: S::U), {pc: 0x8001, a: 0x20, p: S::U, cycle: 2});
    test_instruction!(IN::Lsr, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x01, p: S::U), {pc: 0x8001, a: 0x00, p: S::U | S::C | S::Z, cycle: 2});
    test_instruction!(IN::Rol, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x40, p: S::U), {pc: 0x8001, a: 0x80, p: S::U | S::N, cycle: 2});
    test_instruction!(IN::Rol, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x80, p: S::U), {pc: 0x8001, a: 0x00, p: S::U | S::C | S::Z, cycle: 2});
    test_instruction!(IN::Ror, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x40, p: S::U), {pc: 0x8001, a: 0x20, p: S::U, cycle: 2});
    test_instruction!(IN::Ror, AM::Imp, [], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x01, p: S::U), {pc: 0x8001, a: 0x00, p: S::U | S::C | S::Z, cycle: 2});
}

#[test]
fn test_imm_instructions() {
    test_instruction!(IN::Adc, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Adc, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U | S::C), {pc: 0x8002, a: 0x16, p: S::U, cycle: 2});
    test_instruction!(IN::Adc, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0xFF, p: S::U | S::C), {pc: 0x8002, a: 0x05, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Adc, AM::Imm, [0x00], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x00, p: S::U), {pc: 0x8002, a: 0x00, p: S::U | S::Z, cycle: 2});
    test_instruction!(IN::Sbc, AM::Imm, [0x01], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U), {pc: 0x8002, a: 0x0E, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Sbc, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U), {pc: 0x8002, a: 0x0A, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Sbc, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x00, p: S::U), {pc: 0x8002, a: 0xFA, p: S::U | S::N, cycle: 2});
    test_instruction!(IN::And, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U), {pc: 0x8002, a: 0x00, p: S::U | S::Z, cycle: 2});
    test_instruction!(IN::Ora, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U), {pc: 0x8002, a: 0x15, p: S::U, cycle: 2});
    test_instruction!(IN::Eor, AM::Imm, [0x01], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U), {pc: 0x8002, a: 0x11, p: S::U, cycle: 2});
    test_instruction!(IN::Cmp, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x10, p: S::U), {pc: 0x8002, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Cpx, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x10, p: S::U), {pc: 0x8002, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Cpy, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x10, p: S::U), {pc: 0x8002, p: S::U | S::C, cycle: 2});
    test_instruction!(IN::Lda, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U), {pc: 0x8002, a: 0x05, p: S::U, cycle: 2});
    test_instruction!(IN::Ldx, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U), {pc: 0x8002, x: 0x05, p: S::U, cycle: 2});
    test_instruction!(IN::Ldy, AM::Imm, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: S::U), {pc: 0x8002, y: 0x05, p: S::U, cycle: 2});
}

#[test]
fn test_zp0_instructions() {
    test_instruction!(IN::Adc, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8002, a: 0x11, cycle: 3});
    test_instruction!(IN::Sbc, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8002, a: 0x0E, cycle: 3});
    test_instruction!(IN::And, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8002, a: 0x00, cycle: 3});
    test_instruction!(IN::Ora, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8002, a: 0x11, cycle: 3});
    test_instruction!(IN::Eor, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8002, a: 0x11, cycle: 3});
    test_instruction!(IN::Cmp, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 3});
    test_instruction!(IN::Cpx, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 3});
    test_instruction!(IN::Cpy, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 3});
    test_instruction!(IN::Lda, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, a: 0x01, cycle: 3});
    test_instruction!(IN::Ldx, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, x: 0x01, cycle: 3});
    test_instruction!(IN::Ldy, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, y: 0x01, cycle: 3});
    test_instruction!(IN::Sta, AM::Zp0, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x01), {pc: 0x8002, cycle: 3});
    test_instruction!(IN::Stx, AM::Zp0, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x01), {pc: 0x8002, cycle: 3});
    test_instruction!(IN::Sty, AM::Zp0, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x01), {pc: 0x8002, cycle: 3});
    test_instruction!(IN::Inc, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 5});
    test_instruction!(IN::Dec, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 5});
    test_instruction!(IN::Asl, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x80), pc: 0x8000), {pc: 0x8002, cycle: 5});
    test_instruction!(IN::Lsr, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 5});
    test_instruction!(IN::Rol, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x80), pc: 0x8000), {pc: 0x8002, cycle: 5});
    test_instruction!(IN::Ror, AM::Zp0, [0x05], mk_cpu!(mk_io!(0x05: 0x01), pc: 0x8000), {pc: 0x8002, cycle: 5});
}

#[test]
fn test_zpx_instructions() {
    test_instruction!(IN::Adc, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8002, a: 0x11, cycle: 4});
    test_instruction!(IN::Sbc, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8002, a: 0x0E, cycle: 4});
    test_instruction!(IN::And, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8002, a: 0x00, cycle: 4});
    test_instruction!(IN::Ora, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8002, a: 0x11, cycle: 4});
    test_instruction!(IN::Eor, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8002, a: 0x11, cycle: 4});
    test_instruction!(IN::Cmp, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 4});
    test_instruction!(IN::Lda, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, a: 0x01, cycle: 4});
    test_instruction!(IN::Sta, AM::Zpx, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x04, a: 0x01), {pc: 0x8002, cycle: 4});
    test_instruction!(IN::Inc, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Dec, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Asl, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x80), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Lsr, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Rol, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x80), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Ror, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Ldy, AM::Zpx, [0x05], mk_cpu!(mk_io!(0x09: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8002, y: 0x01, cycle: 4});
    test_instruction!(IN::Sty, AM::Zpx, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x04, y: 0x01), {pc: 0x8002, cycle: 4});
}

#[test]
fn test_abs_instructions() {
    test_instruction!(IN::Adc, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Sbc, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8003, a: 0x0E, cycle: 4});
    test_instruction!(IN::And, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8003, a: 0x00, cycle: 4});
    test_instruction!(IN::Ora, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Eor, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000, a: 0x10), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Cmp, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Cpx, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Cpy, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Lda, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, a: 0x01, cycle: 4});
    test_instruction!(IN::Ldx, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, x: 0x01, cycle: 4});
    test_instruction!(IN::Ldy, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, y: 0x01, cycle: 4});
    test_instruction!(IN::Sta, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000, a: 0x01), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Stx, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x01), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Sty, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x01), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Inc, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 6});
    test_instruction!(IN::Dec, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 6});
    test_instruction!(IN::Asl, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x80), pc: 0x8000), {pc: 0x8003, cycle: 6});
    test_instruction!(IN::Lsr, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 6});
    test_instruction!(IN::Rol, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x80), pc: 0x8000), {pc: 0x8003, cycle: 6});
    test_instruction!(IN::Ror, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01), pc: 0x8000), {pc: 0x8003, cycle: 6});
    test_instruction!(IN::Jmp, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000), {pc: 0x1005, cycle: 3});
    test_instruction!(IN::Jsr, AM::Abs, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000, sp: 0x40), {pc: 0x1005, sp: 0x3E, cycle: 6});
}

#[test]
fn test_abx_instructions() {
    test_instruction!(IN::Adc, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Sbc, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x0E, cycle: 4});
    test_instruction!(IN::And, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x00, cycle: 4});
    test_instruction!(IN::Ora, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Eor, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Cmp, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 4});
    test_instruction!(IN::Lda, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, a: 0x01, cycle: 4});
    test_instruction!(IN::Sta, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x04, a: 0x01), {pc: 0x8003, cycle: 5});
    test_instruction!(IN::Inc, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 7});
    test_instruction!(IN::Dec, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 7});
    test_instruction!(IN::Asl, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x80), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 7});
    test_instruction!(IN::Lsr, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 7});
    test_instruction!(IN::Rol, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x80), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 7});
    test_instruction!(IN::Ror, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 7});
    test_instruction!(IN::Ldy, AM::Abx, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, y: 0x01, cycle: 4});

    // page cross
    test_instruction!(IN::Adc, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x11, cycle: 5});
    test_instruction!(IN::Sbc, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x0E, cycle: 5});
    test_instruction!(IN::And, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x00, cycle: 5});
    test_instruction!(IN::Ora, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x11, cycle: 5});
    test_instruction!(IN::Eor, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, a: 0x10, x: 0x04), {pc: 0x8003, a: 0x11, cycle: 5});
    test_instruction!(IN::Cmp, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, cycle: 5});
    test_instruction!(IN::Lda, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, a: 0x01, cycle: 5});
    test_instruction!(IN::Ldy, AM::Abx, [0xFF, 0x10], mk_cpu!(mk_io!(0x1103: 0x01), pc: 0x8000, x: 0x04), {pc: 0x8003, y: 0x01, cycle: 5});
}

#[test]
fn test_aby_instructions() {
    test_instruction!(IN::Ldx, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, y: 0x04), {pc: 0x8003, x: 0x01, cycle: 4});
    test_instruction!(IN::Sta, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x04, a: 0x01), {pc: 0x8003, cycle: 5});
    test_instruction!(IN::Adc, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, y: 0x04), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Sbc, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, y: 0x04), {pc: 0x8003, a: 0x0E, cycle: 4});
    test_instruction!(IN::And, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, y: 0x04), {pc: 0x8003, a: 0x00, cycle: 4});
    test_instruction!(IN::Ora, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, y: 0x04), {pc: 0x8003, a: 0x11, cycle: 4});
    test_instruction!(IN::Eor, AM::Aby, [0x05, 0x10], mk_cpu!(mk_io!(0x1009: 0x01), pc: 0x8000, a: 0x10, y: 0x04), {pc: 0x8003, a: 0x11, cycle: 4});
}
#[test]
fn test_aby_instructions_with_page_cross() {
    test_instruction!(IN::Ldx, AM::Aby, [0xFF, 0x10], mk_cpu!(mk_io!(0x1102: 0x01), pc: 0x8000, y: 0x03), {pc: 0x8003, x: 0x01, cycle: 5});
    test_instruction!(IN::Adc, AM::Aby, [0xFF, 0x10], mk_cpu!(mk_io!(0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8003, a: 0x11, cycle: 5});
    test_instruction!(IN::Sbc, AM::Aby, [0xFF, 0x10], mk_cpu!(mk_io!(0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8003, a: 0x0E, cycle: 5});
    test_instruction!(IN::And, AM::Aby, [0xFF, 0x10], mk_cpu!(mk_io!(0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8003, a: 0x00, cycle: 5});
    test_instruction!(IN::Ora, AM::Aby, [0xFF, 0x10], mk_cpu!(mk_io!(0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8003, a: 0x11, cycle: 5});
    test_instruction!(IN::Eor, AM::Aby, [0xFF, 0x10], mk_cpu!(mk_io!(0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8003, a: 0x11, cycle: 5});
}

#[test]
fn test_rel_instructions() {
    test_instruction!(IN::Bcc, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Bcc, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x01), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bcs, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bcs, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x01), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Beq, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Beq, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x02), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Bmi, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bmi, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x80), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Bne, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Bne, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x02), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bpl, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Bpl, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x80), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bvc, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8007, cycle: 2});
    test_instruction!(IN::Bvc, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x40), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bvs, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x8002, cycle: 2});
    test_instruction!(IN::Bvs, AM::Rel, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x40), {pc: 0x8007, cycle: 2});

    // branch instructions take 1 additional cycle when branching to a new page (only on success)
    test_instruction!(IN::Bcc, AM::Rel, [-0x05i8 as u8], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x00), {pc: 0x7FFD, cycle: 3});
    test_instruction!(IN::Bcc, AM::Rel, [-0x05i8 as u8], mk_cpu!(mk_io!(), pc: 0x8000, p: 0x01), {pc: 0x8002, cycle: 2});
}

#[test]
fn test_ind_instructions() {
    test_instruction!(IN::Jmp, AM::Ind, [0x05, 0x10], mk_cpu!(mk_io!(0x1005: 0x01, 0x1006: 0x80), pc: 0x8000), {pc: 0x8001, cycle: 5});
    // boundary bug
    test_instruction!(IN::Jmp, AM::Ind, [0xFF, 0x10], mk_cpu!(mk_io!(0x1000: 0x80, 0x10FF: 0x01, 0x1100: 0x11), pc: 0x8000), {pc: 0x8001, cycle: 5});
}

#[test]
fn test_izx_instructions() {
    test_instruction!(IN::Adc, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, a: 0x10, x: 0x03), {pc: 0x8002, a: 0x11, cycle: 6});
    test_instruction!(IN::Sbc, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, a: 0x10, x: 0x03), {pc: 0x8002, a: 0x0E, cycle: 6});
    test_instruction!(IN::And, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, a: 0x10, x: 0x03), {pc: 0x8002, a: 0x00, cycle: 6});
    test_instruction!(IN::Ora, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, a: 0x10, x: 0x03), {pc: 0x8002, a: 0x11, cycle: 6});
    test_instruction!(IN::Eor, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, a: 0x10, x: 0x03), {pc: 0x8002, a: 0x11, cycle: 6});
    test_instruction!(IN::Cmp, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, x: 0x03), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Lda, AM::Izx, [0x05], mk_cpu!(mk_io!(0x08: 0x05, 0x09: 0x10, 0x1005: 0x01), pc: 0x8000, x: 0x03), {pc: 0x8002, a: 0x01, cycle: 6});
    test_instruction!(IN::Sta, AM::Izx, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, x: 0x03, a: 0x01), {pc: 0x8002, cycle: 6});
}

#[test]
fn test_izy_instructions() {
    test_instruction!(IN::Adc, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x11, cycle: 5});
    test_instruction!(IN::Sbc, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x0E, cycle: 5});
    test_instruction!(IN::And, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x00, cycle: 5});
    test_instruction!(IN::Ora, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x11, cycle: 5});
    test_instruction!(IN::Eor, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x11, cycle: 5});
    test_instruction!(IN::Cmp, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, y: 0x03), {pc: 0x8002, cycle: 5});
    test_instruction!(IN::Lda, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0x05, 0x06: 0x10, 0x1008: 0x01), pc: 0x8000, y: 0x03), {pc: 0x8002, a: 0x01, cycle: 5});
    test_instruction!(IN::Sta, AM::Izy, [0x05], mk_cpu!(mk_io!(), pc: 0x8000, y: 0x03, a: 0x01), {pc: 0x8002, cycle: 6});

    // page cross
    test_instruction!(IN::Adc, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x11, cycle: 6});
    test_instruction!(IN::Sbc, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x0E, cycle: 6});
    test_instruction!(IN::And, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x00, cycle: 6});
    test_instruction!(IN::Ora, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x11, cycle: 6});
    test_instruction!(IN::Eor, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, a: 0x10, y: 0x03), {pc: 0x8002, a: 0x11, cycle: 6});
    test_instruction!(IN::Cmp, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, y: 0x03), {pc: 0x8002, cycle: 6});
    test_instruction!(IN::Lda, AM::Izy, [0x05], mk_cpu!(mk_io!(0x05: 0xFF, 0x06: 0x10, 0x1102: 0x01), pc: 0x8000, y: 0x03), {pc: 0x8002, a: 0x01, cycle: 6});
}
