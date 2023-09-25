#[cfg(test)]
mod tests;

mod disasm;
mod opcodes;
mod status;

pub use disasm::*;
pub use status::Status;

use opcodes::*;

const STACK_BASE_ADDR: u16 = 0x0100;
const INITIAL_SP: u8 = 0xFD;
const BREAK_VECTOR: u16 = 0xFFFE;

pub trait IO {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub struct Cpu<I: IO> {
    pub io: I,

    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub p: Status,

    pub cycle: usize,
    pub busy_cycles: usize,
}

impl<I: IO> Cpu<I> {
    pub fn new(io: I) -> Self {
        Self {
            io,

            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: INITIAL_SP,
            p: (Status::U | Status::I).into(),

            cycle: 0,
            busy_cycles: 0,
        }
    }

    pub fn detour(&mut self, vector: u16) {
        self.push_pc();
        let lo = self.io.read(vector);
        let hi = self.io.read(vector.wrapping_add(1));
        self.pc = u16::from_le_bytes([lo, hi]);
    }

    pub fn clock(&mut self) {
        if self.busy_cycles == 0 {
            self.run_instruction();
        }

        self.cycle += 1;
        self.busy_cycles -= 1;
    }

    fn run_instruction(&mut self) {
        let opcode = self.read_pc();
        self.busy_cycles += OPCODE_CYCLES[opcode as usize];
        let (instruction, addr_mode) = opcodes::OPCODES[opcode as usize];

        match instruction {
            Instruction::Nop => {}
            Instruction::Brk => self.brk(),
            Instruction::Pha => self.pha(),
            Instruction::Php => self.php(),
            Instruction::Pla => self.pla(),
            Instruction::Plp => self.plp(),
            Instruction::Rti => self.rti(),
            Instruction::Rts => self.rts(),
            Instruction::Dex => self.dex(),
            Instruction::Dey => self.dey(),
            Instruction::Inx => self.inx(),
            Instruction::Iny => self.iny(),
            Instruction::Tax => self.tax(),
            Instruction::Tay => self.tay(),
            Instruction::Tsx => self.tsx(),
            Instruction::Txa => self.txa(),
            Instruction::Txs => self.txs(),
            Instruction::Tya => self.tya(),
            Instruction::Clc => self.p.set(Status::C, false),
            Instruction::Cld => self.p.set(Status::D, false),
            Instruction::Cli => self.p.set(Status::I, false),
            Instruction::Clv => self.p.set(Status::V, false),
            Instruction::Sec => self.p.set(Status::C, true),
            Instruction::Sed => self.p.set(Status::D, true),
            Instruction::Sei => self.p.set(Status::I, true),
            _ => match addr_mode {
                AddressingMode::Imp => match instruction {
                    Instruction::Asl => self.asl_acc(),
                    Instruction::Lsr => self.lsr_acc(),
                    Instruction::Rol => self.rol_acc(),
                    Instruction::Ror => self.ror_acc(),
                    _ => panic!("Invalid instruction {:?} for IMP", instruction),
                },
                AddressingMode::Imm => {
                    let val = self.read_pc();
                    match instruction {
                        Instruction::Adc => self.adc(val),
                        Instruction::And => self.and(val),
                        Instruction::Cmp => self.cmp(val),
                        Instruction::Cpx => self.cpx(val),
                        Instruction::Cpy => self.cpy(val),
                        Instruction::Eor => self.eor(val),
                        Instruction::Lda => self.lda(val),
                        Instruction::Ldx => self.ldx(val),
                        Instruction::Ldy => self.ldy(val),
                        Instruction::Ora => self.ora(val),
                        Instruction::Sbc => self.sbc(val),
                        _ => panic!("Invalid instruction {:?} for IMM", instruction),
                    }
                }
                AddressingMode::Zp0 => {
                    let addr = self.zp0();
                    match instruction {
                        Instruction::Asl => self.asl_mem(addr),
                        Instruction::Lsr => self.lsr_mem(addr),
                        Instruction::Rol => self.rol_mem(addr),
                        Instruction::Ror => self.ror_mem(addr),
                        Instruction::Adc => self.adc(self.io.read(addr)),
                        Instruction::And => self.and(self.io.read(addr)),
                        Instruction::Bit => self.bit(self.io.read(addr)),
                        Instruction::Cmp => self.cmp(self.io.read(addr)),
                        Instruction::Cpx => self.cpx(self.io.read(addr)),
                        Instruction::Cpy => self.cpy(self.io.read(addr)),
                        Instruction::Eor => self.eor(self.io.read(addr)),
                        Instruction::Lda => self.lda(self.io.read(addr)),
                        Instruction::Ldx => self.ldx(self.io.read(addr)),
                        Instruction::Ldy => self.ldy(self.io.read(addr)),
                        Instruction::Ora => self.ora(self.io.read(addr)),
                        Instruction::Sbc => self.sbc(self.io.read(addr)),
                        Instruction::Dec => self.dec(addr),
                        Instruction::Inc => self.inc(addr),
                        Instruction::Sta => self.sta(addr),
                        Instruction::Stx => self.stx(addr),
                        Instruction::Sty => self.sty(addr),
                        _ => panic!("Invalid instruction {:?} for ZP0", instruction),
                    }
                }
                AddressingMode::Zpx => {
                    let addr = self.zpx();
                    match instruction {
                        Instruction::Asl => self.asl_mem(addr),
                        Instruction::Lsr => self.lsr_mem(addr),
                        Instruction::Rol => self.rol_mem(addr),
                        Instruction::Ror => self.ror_mem(addr),
                        Instruction::Adc => self.adc(self.io.read(addr)),
                        Instruction::Sbc => self.sbc(self.io.read(addr)),
                        Instruction::And => self.and(self.io.read(addr)),
                        Instruction::Eor => self.eor(self.io.read(addr)),
                        Instruction::Ora => self.ora(self.io.read(addr)),
                        Instruction::Cmp => self.cmp(self.io.read(addr)),
                        Instruction::Lda => self.lda(self.io.read(addr)),
                        Instruction::Ldy => self.ldy(self.io.read(addr)),
                        Instruction::Sty => self.sty(addr),
                        Instruction::Dec => self.dec(addr),
                        Instruction::Inc => self.inc(addr),
                        Instruction::Sta => self.sta(addr),
                        _ => panic!("Invalid instruction {:?} for ZPX", instruction),
                    }
                }
                AddressingMode::Zpy => {
                    let addr = self.zpy();
                    match instruction {
                        Instruction::Ldx => self.ldx(self.io.read(addr)),
                        Instruction::Stx => self.stx(addr),
                        _ => panic!("Invalid instruction {:?} for ZPY", instruction),
                    }
                }
                AddressingMode::Abs => {
                    let addr = self.abs();
                    match instruction {
                        Instruction::Asl => self.asl_mem(addr),
                        Instruction::Lsr => self.lsr_mem(addr),
                        Instruction::Rol => self.rol_mem(addr),
                        Instruction::Ror => self.ror_mem(addr),
                        Instruction::Adc => self.adc(self.io.read(addr)),
                        Instruction::And => self.and(self.io.read(addr)),
                        Instruction::Bit => self.bit(self.io.read(addr)),
                        Instruction::Cmp => self.cmp(self.io.read(addr)),
                        Instruction::Cpx => self.cpx(self.io.read(addr)),
                        Instruction::Cpy => self.cpy(self.io.read(addr)),
                        Instruction::Eor => self.eor(self.io.read(addr)),
                        Instruction::Lda => self.lda(self.io.read(addr)),
                        Instruction::Ldx => self.ldx(self.io.read(addr)),
                        Instruction::Ldy => self.ldy(self.io.read(addr)),
                        Instruction::Ora => self.ora(self.io.read(addr)),
                        Instruction::Sbc => self.sbc(self.io.read(addr)),
                        Instruction::Jmp => self.jmp(addr),
                        Instruction::Jsr => self.jsr(addr),
                        Instruction::Dec => self.dec(addr),
                        Instruction::Inc => self.inc(addr),
                        Instruction::Sta => self.sta(addr),
                        Instruction::Stx => self.stx(addr),
                        Instruction::Sty => self.sty(addr),
                        _ => panic!("Invalid instruction {:?} for ABS", instruction),
                    }
                }
                AddressingMode::Abx => {
                    let (addr, crossed) = self.abx();
                    match instruction {
                        Instruction::Asl => self.asl_mem(addr),
                        Instruction::Lsr => self.lsr_mem(addr),
                        Instruction::Rol => self.rol_mem(addr),
                        Instruction::Ror => self.ror_mem(addr),
                        Instruction::Dec => self.dec(addr),
                        Instruction::Inc => self.inc(addr),
                        Instruction::Sta => self.sta(addr),
                        _ => {
                            self.busy_cycles += crossed as usize;
                            match instruction {
                                Instruction::Adc => self.adc(self.io.read(addr)),
                                Instruction::Sbc => self.sbc(self.io.read(addr)),
                                Instruction::Lda => self.lda(self.io.read(addr)),
                                Instruction::Ldy => self.ldy(self.io.read(addr)),
                                Instruction::And => self.and(self.io.read(addr)),
                                Instruction::Cmp => self.cmp(self.io.read(addr)),
                                Instruction::Eor => self.eor(self.io.read(addr)),
                                Instruction::Ora => self.ora(self.io.read(addr)),
                                _ => panic!("Invalid instruction {:?} for ABX", instruction),
                            }
                        }
                    };
                }
                AddressingMode::Aby => {
                    let (addr, crossed) = self.aby();
                    match instruction {
                        Instruction::Sta => self.sta(addr),
                        _ => {
                            self.busy_cycles += crossed as usize;
                            match instruction {
                                Instruction::Adc => self.adc(self.io.read(addr)),
                                Instruction::Sbc => self.sbc(self.io.read(addr)),
                                Instruction::And => self.and(self.io.read(addr)),
                                Instruction::Eor => self.eor(self.io.read(addr)),
                                Instruction::Ora => self.ora(self.io.read(addr)),
                                Instruction::Cmp => self.cmp(self.io.read(addr)),
                                Instruction::Lda => self.lda(self.io.read(addr)),
                                Instruction::Ldx => self.ldx(self.io.read(addr)),
                                _ => panic!("Invalid instruction {:?} for ABY", instruction),
                            }
                        }
                    };
                }
                AddressingMode::Ind => {
                    let addr = self.ind();
                    match instruction {
                        Instruction::Jmp => self.jmp(addr),
                        _ => panic!("Invalid instruction {:?} for IND", instruction),
                    }
                }
                AddressingMode::Izx => {
                    let addr = self.izx();
                    match instruction {
                        Instruction::Adc => self.adc(self.io.read(addr)),
                        Instruction::And => self.and(self.io.read(addr)),
                        Instruction::Cmp => self.cmp(self.io.read(addr)),
                        Instruction::Eor => self.eor(self.io.read(addr)),
                        Instruction::Lda => self.lda(self.io.read(addr)),
                        Instruction::Ora => self.ora(self.io.read(addr)),
                        Instruction::Sbc => self.sbc(self.io.read(addr)),
                        Instruction::Sta => self.sta(addr),
                        _ => panic!("Invalid instruction {:?} for IZX", instruction),
                    }
                }
                AddressingMode::Izy => {
                    let (addr, crossed) = self.izy();
                    match instruction {
                        Instruction::Sta => self.sta(addr),
                        _ => {
                            self.busy_cycles += crossed as usize;
                            match instruction {
                                Instruction::Adc => self.adc(self.io.read(addr)),
                                Instruction::Sbc => self.sbc(self.io.read(addr)),
                                Instruction::And => self.and(self.io.read(addr)),
                                Instruction::Eor => self.eor(self.io.read(addr)),
                                Instruction::Ora => self.ora(self.io.read(addr)),
                                Instruction::Cmp => self.cmp(self.io.read(addr)),
                                Instruction::Lda => self.lda(self.io.read(addr)),
                                _ => panic!("Invalid instruction {:?} for IZY", instruction),
                            }
                        }
                    };
                }
                AddressingMode::Rel => {
                    let (address, crossed) = self.rel();

                    let cond = match instruction {
                        Instruction::Bcs => self.p.get(Status::C),
                        Instruction::Bcc => !self.p.get(Status::C),
                        Instruction::Beq => self.p.get(Status::Z),
                        Instruction::Bne => !self.p.get(Status::Z),
                        Instruction::Bmi => self.p.get(Status::N),
                        Instruction::Bpl => !self.p.get(Status::N),
                        Instruction::Bvs => self.p.get(Status::V),
                        Instruction::Bvc => !self.p.get(Status::V),
                        _ => panic!("Invalid instruction {:?} for REL", instruction),
                    };

                    if cond {
                        self.pc = address;
                        self.busy_cycles += crossed as usize;
                    }
                }
            },
        }
    }

    fn zp0(&mut self) -> u16 {
        self.read_pc() as u16
    }

    fn zpx(&mut self) -> u16 {
        self.read_pc().wrapping_add(self.x) as u16
    }

    fn zpy(&mut self) -> u16 {
        self.read_pc().wrapping_add(self.y) as u16
    }

    fn abs(&mut self) -> u16 {
        let lo = self.read_pc();
        let hi = self.read_pc();
        u16::from_le_bytes([lo, hi])
    }

    fn abx(&mut self) -> (u16, bool) {
        let lo = self.read_pc();
        let hi = self.read_pc();
        let base_address = u16::from_le_bytes([lo, hi]);
        let addr = base_address.wrapping_add(self.x as u16);
        let crossed = page_crossed(base_address, addr);
        (addr, crossed)
    }

    fn aby(&mut self) -> (u16, bool) {
        let lo = self.read_pc();
        let hi = self.read_pc();
        let base_address = u16::from_le_bytes([lo, hi]);
        let addr = base_address.wrapping_add(self.y as u16);
        let crossed = page_crossed(base_address, addr);
        (addr, crossed)
    }

    fn ind(&mut self) -> u16 {
        let lo = self.read_pc();
        let hi = self.read_pc();
        let base_address = u16::from_le_bytes([lo, hi]);
        if base_address & 0xFF == 0xFF {
            // emulate 6502 page boundary hardware bug
            let lo = self.io.read(base_address);
            let hi = self.io.read(base_address & 0xFF00);
            u16::from_le_bytes([lo, hi])
        } else {
            let lo = self.io.read(base_address);
            let hi = self.io.read(base_address.wrapping_add(1));
            u16::from_le_bytes([lo, hi])
        }
    }

    fn izx(&mut self) -> u16 {
        let ind_address = self.read_pc().wrapping_add(self.x);
        let lo = self.io.read(ind_address as u16);
        let hi = self.io.read(ind_address.wrapping_add(1) as u16);
        u16::from_le_bytes([lo, hi])
    }

    fn izy(&mut self) -> (u16, bool) {
        let ind_address = self.read_pc();
        let lo = self.io.read(ind_address as u16);
        let hi = self.io.read(ind_address.wrapping_add(1) as u16);
        let base_address = u16::from_le_bytes([lo, hi]);
        let addr = base_address.wrapping_add(self.y as u16);
        let crossed = page_crossed(base_address, addr);
        (addr, crossed)
    }

    fn rel(&mut self) -> (u16, bool) {
        let offset = self.read_pc() as i8 as u16;
        let addr = self.pc.wrapping_add(offset);
        let crossed = page_crossed(self.pc, addr);
        (addr, crossed)
    }

    fn read_pc(&mut self) -> u8 {
        let data = self.io.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        data
    }

    fn stack_addr(&self) -> u16 {
        STACK_BASE_ADDR.wrapping_add(self.sp as u16)
    }

    fn push(&mut self, val: u8) {
        self.io.write(self.stack_addr(), val);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.io.read(self.stack_addr())
    }

    fn push_pc(&mut self) {
        let pc_bytes = self.pc.to_be_bytes();
        self.push(pc_bytes[0]);
        self.push(pc_bytes[1]);
    }

    fn pop_pc(&mut self) {
        let hi = self.pop();
        let lo = self.pop();
        self.pc = u16::from_be_bytes([lo, hi]);
    }

    // Transfer Instructions

    /// load accumulator
    fn lda(&mut self, val: u8) {
        self.a = val;
        self.p.set_zn(val);
    }

    /// load X
    fn ldx(&mut self, val: u8) {
        self.x = val;
        self.p.set_zn(val);
    }

    /// load Y
    fn ldy(&mut self, val: u8) {
        self.y = val;
        self.p.set_zn(val);
    }

    /// store accumulator
    fn sta(&mut self, addr: u16) {
        self.io.write(addr, self.a);
    }

    /// store X
    fn stx(&mut self, addr: u16) {
        self.io.write(addr, self.x);
    }

    /// store Y
    fn sty(&mut self, addr: u16) {
        self.io.write(addr, self.y);
    }

    /// transfer accumulator to X
    fn tax(&mut self) {
        self.x = self.a;
        self.p.set_zn(self.x);
    }

    /// transfer accumulator to Y
    fn tay(&mut self) {
        self.y = self.a;
        self.p.set_zn(self.y);
    }

    /// transfer stack pointer to X
    fn tsx(&mut self) {
        self.x = self.sp;
        self.p.set_zn(self.x);
    }

    /// transfer X to accumulator
    fn txa(&mut self) {
        self.a = self.x;
        self.p.set_zn(self.a);
    }

    /// transfer X to stack pointer
    fn txs(&mut self) {
        self.sp = self.x;
    }

    /// transfer Y to accumulator
    fn tya(&mut self) {
        self.a = self.y;
        self.p.set_zn(self.a);
    }

    // Stack Instructions

    /// push accumulator
    fn pha(&mut self) {
        let val = self.a;
        self.push(val);
    }

    /// push processor status
    fn php(&mut self) {
        self.push(self.p.raw | Status::B | Status::U);
    }

    /// pull accumulator
    fn pla(&mut self) {
        let val = self.pop();
        self.a = val;
        self.p.set_zn(val);
    }

    /// pull processor status
    fn plp(&mut self) {
        let val = self.pop() & !(Status::B | Status::U);
        self.p.set(!(Status::B | Status::U), false);
        self.p.set(val, true);
    }

    // Decrements & Increments

    /// decrement (memory)
    fn dec(&mut self, addr: u16) {
        let val = self.io.read(addr).wrapping_sub(1);
        self.io.write(addr, val);
        self.p.set_zn(val);
    }

    /// decrement X
    fn dex(&mut self) {
        let val = self.x.wrapping_sub(1);
        self.x = val;
        self.p.set_zn(val);
    }

    /// decrement Y
    fn dey(&mut self) {
        let val = self.y.wrapping_sub(1);
        self.y = val;
        self.p.set_zn(val);
    }

    /// increment (memory)
    fn inc(&mut self, addr: u16) {
        let val = self.io.read(addr).wrapping_add(1);
        self.io.write(addr, val);
        self.p.set_zn(val);
    }

    /// increment X
    fn inx(&mut self) {
        let val = self.x.wrapping_add(1);
        self.x = val;
        self.p.set_zn(val);
    }

    /// increment Y
    fn iny(&mut self) {
        let val = self.y.wrapping_add(1);
        self.y = val;
        self.p.set_zn(val);
    }

    // Arithmetic Operations

    /// add with carry (prepare by CLC)
    fn adc(&mut self, val: u8) {
        let carry = self.p.get(Status::C) as u8;
        let (result, overflow_carry) = self.a.overflowing_add(carry);
        let (result, overflow_operand) = result.overflowing_add(val);
        let sig_overflow = (!(self.a ^ val) & (self.a ^ result)) & 0x80 != 0;
        let carry = overflow_operand || overflow_carry;
        self.a = result;
        self.p.set_zn(result);
        self.p.set(Status::C, carry);
        self.p.set(Status::V, sig_overflow);
    }

    /// subtract with carry (prepare by SEC)
    fn sbc(&mut self, val: u8) {
        let carry = 1 - self.p.get(Status::C) as u8;
        let (result, overflow_carry) = self.a.overflowing_sub(carry);
        let (result, overflow_operand) = result.overflowing_sub(val);
        let sig_overflow = ((self.a ^ val) & (self.a ^ result)) & 0x80 != 0;
        let carry = !overflow_operand && !overflow_carry;
        self.a = result;
        self.p.set_zn(result);
        self.p.set(Status::C, carry);
        self.p.set(Status::V, sig_overflow);
    }

    // Logical Operations

    /// and (with accumulator)
    fn and(&mut self, val: u8) {
        self.a &= val;
        let result = self.a;
        self.p.set_zn(result);
    }

    /// exclusive or (with accumulator)
    fn eor(&mut self, val: u8) {
        self.a ^= val;
        let result = self.a;
        self.p.set_zn(result);
    }

    /// or (with accumulator)
    fn ora(&mut self, val: u8) {
        self.a |= val;
        let result = self.a;
        self.p.set_zn(result);
    }

    // Shift & Rotate Instructions

    /// arithmetic shift left (shifts in a zero bit on the right)
    fn asl_acc(&mut self) {
        self.a = self.asl(self.a);
    }

    fn asl_mem(&mut self, addr: u16) {
        let res = self.asl(self.io.read(addr));
        self.io.write(addr, res);
    }

    fn asl(&mut self, val: u8) -> u8 {
        let result = val << 1;
        self.p.set_zn(result);
        self.p.set(Status::C, val & 0x80 != 0);
        result
    }

    /// logical shift right (shifts in a zero bit on the left)
    fn lsr_acc(&mut self) {
        self.a = self.lsr(self.a);
    }

    fn lsr_mem(&mut self, addr: u16) {
        let res = self.lsr(self.io.read(addr));
        self.io.write(addr, res);
    }

    fn lsr(&mut self, val: u8) -> u8 {
        let result = val >> 1;
        self.p.set_zn(result);
        self.p.set(Status::C, val & 0x01 != 0);
        result
    }

    /// rotate left (shifts in carry bit on the right)
    fn rol_acc(&mut self) {
        self.a = self.rol(self.a);
    }

    fn rol_mem(&mut self, addr: u16) {
        let res = self.rol(self.io.read(addr));
        self.io.write(addr, res);
    }

    fn rol(&mut self, val: u8) -> u8 {
        let carry = self.p.get(Status::C) as u8;
        let result = (val << 1) | carry;
        self.p.set_zn(result);
        self.p.set(Status::C, val & 0x80 != 0);
        result
    }

    /// rotate right (shifts in zero bit on the left)
    fn ror_acc(&mut self) {
        self.a = self.ror(self.a);
    }

    fn ror_mem(&mut self, addr: u16) {
        let res = self.ror(self.io.read(addr));
        self.io.write(addr, res);
    }

    fn ror(&mut self, val: u8) -> u8 {
        let carry = ((self.p.get(Status::C)) as u8) << 7;
        let result = (val >> 1) | carry;
        self.p.set_zn(result);
        self.p.set(Status::C, val & 0x01 != 0);
        result
    }

    // Jump & Call Instructions

    /// jump
    fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// jump to subroutine
    fn jsr(&mut self, addr: u16) {
        self.pc = self.pc.wrapping_sub(1);
        self.push_pc();
        self.pc = addr;
    }

    /// return from subroutine
    fn rts(&mut self) {
        self.pop_pc();
        self.pc = self.pc.wrapping_add(1);
    }

    // Compare Instructions

    /// compare (with accumulator)
    fn cmp(&mut self, val: u8) {
        let result = self.a.wrapping_sub(val);
        self.p.set_zn(result);
        self.p.set(Status::C, self.a >= val);
    }

    /// compare X
    fn cpx(&mut self, val: u8) {
        let result = self.x.wrapping_sub(val);
        self.p.set_zn(result);
        self.p.set(Status::C, self.x >= val);
    }

    /// compare Y
    fn cpy(&mut self, val: u8) {
        let result = self.y.wrapping_sub(val);
        self.p.set_zn(result);
        self.p.set(Status::C, self.y >= val);
    }

    // System Functions

    /// break
    fn brk(&mut self) {
        self.pc += 1;
        self.push_pc();
        self.push(self.p.raw | Status::B | Status::U);
        self.p.set(Status::B | Status::U | Status::I, true);
        self.detour(BREAK_VECTOR);
    }

    /// return from interrupt
    fn rti(&mut self) {
        self.p = self.pop().into();
        self.p.set(Status::U, true);
        self.pop_pc();
    }

    // Other

    /// bit test (accumulator & memory)
    fn bit(&mut self, val: u8) {
        self.p.set(Status::Z, self.a & val == 0);
        self.p.set(Status::V, val & 0x40 != 0);
        self.p.set(Status::N, val & 0x80 != 0);
    }
}

fn page_crossed(base_addr: u16, address: u16) -> bool {
    (base_addr & 0xFF00) != (address & 0xFF00)
}

const OPCODE_CYCLES: [usize; 256] = [
    7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5,
    2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
    2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];
