#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Imp,
    Imm,
    Zp0,
    Zpx,
    Zpy,
    Rel,
    Abs,
    Abx,
    Aby,
    Ind,
    Izx,
    Izy,
}

impl AddressingMode {
    pub fn len(&self) -> usize {
        match self {
            AddressingMode::Imp => 0,
            AddressingMode::Imm => 1,
            AddressingMode::Zp0 => 1,
            AddressingMode::Zpx => 1,
            AddressingMode::Zpy => 1,
            AddressingMode::Rel => 1,
            AddressingMode::Abs => 2,
            AddressingMode::Abx => 2,
            AddressingMode::Aby => 2,
            AddressingMode::Ind => 2,
            AddressingMode::Izx => 1,
            AddressingMode::Izy => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Instruction {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk,
    Bvc, Bvs, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex,
    Dey, Eor, Inc, Inx, Iny, Jmp, Jsr, Lda, Ldx, Ldy, Lsr,
    Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti, Rts, Sbc,
    Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs,
    Tya, Unk,
}

pub const OPCODES: [(Instruction, AddressingMode); 256] = [
    (Instruction::Brk, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Zp0),
    (Instruction::Asl, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Php, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Imm),
    (Instruction::Asl, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Abs),
    (Instruction::Asl, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bpl, AddressingMode::Rel),
    (Instruction::Ora, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Zpx),
    (Instruction::Asl, AddressingMode::Zpx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Clc, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Aby),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ora, AddressingMode::Abx),
    (Instruction::Asl, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Jsr, AddressingMode::Abs),
    (Instruction::And, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bit, AddressingMode::Zp0),
    (Instruction::And, AddressingMode::Zp0),
    (Instruction::Rol, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Plp, AddressingMode::Imp),
    (Instruction::And, AddressingMode::Imm),
    (Instruction::Rol, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bit, AddressingMode::Abs),
    (Instruction::And, AddressingMode::Abs),
    (Instruction::Rol, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bmi, AddressingMode::Rel),
    (Instruction::And, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::And, AddressingMode::Zpx),
    (Instruction::Rol, AddressingMode::Zpx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sec, AddressingMode::Imp),
    (Instruction::And, AddressingMode::Aby),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::And, AddressingMode::Abx),
    (Instruction::Rol, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Rti, AddressingMode::Imp),
    (Instruction::Eor, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Eor, AddressingMode::Zp0),
    (Instruction::Lsr, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Pha, AddressingMode::Imp),
    (Instruction::Eor, AddressingMode::Imm),
    (Instruction::Lsr, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Jmp, AddressingMode::Abs),
    (Instruction::Eor, AddressingMode::Abs),
    (Instruction::Lsr, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bvc, AddressingMode::Rel),
    (Instruction::Eor, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Eor, AddressingMode::Zpx),
    (Instruction::Lsr, AddressingMode::Zpx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cli, AddressingMode::Imp),
    (Instruction::Eor, AddressingMode::Aby),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Eor, AddressingMode::Abx),
    (Instruction::Lsr, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Rts, AddressingMode::Imp),
    (Instruction::Adc, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Adc, AddressingMode::Zp0),
    (Instruction::Ror, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Pla, AddressingMode::Imp),
    (Instruction::Adc, AddressingMode::Imm),
    (Instruction::Ror, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Jmp, AddressingMode::Ind),
    (Instruction::Adc, AddressingMode::Abs),
    (Instruction::Ror, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bvs, AddressingMode::Rel),
    (Instruction::Adc, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Adc, AddressingMode::Zpx),
    (Instruction::Ror, AddressingMode::Zpx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sei, AddressingMode::Imp),
    (Instruction::Adc, AddressingMode::Aby),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Adc, AddressingMode::Abx),
    (Instruction::Ror, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sta, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sty, AddressingMode::Zp0),
    (Instruction::Sta, AddressingMode::Zp0),
    (Instruction::Stx, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Dey, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Txa, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sty, AddressingMode::Abs),
    (Instruction::Sta, AddressingMode::Abs),
    (Instruction::Stx, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bcc, AddressingMode::Rel),
    (Instruction::Sta, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sty, AddressingMode::Zpx),
    (Instruction::Sta, AddressingMode::Zpx),
    (Instruction::Stx, AddressingMode::Zpy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Tya, AddressingMode::Imp),
    (Instruction::Sta, AddressingMode::Aby),
    (Instruction::Txs, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sta, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ldy, AddressingMode::Imm),
    (Instruction::Lda, AddressingMode::Izx),
    (Instruction::Ldx, AddressingMode::Imm),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ldy, AddressingMode::Zp0),
    (Instruction::Lda, AddressingMode::Zp0),
    (Instruction::Ldx, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Tay, AddressingMode::Imp),
    (Instruction::Lda, AddressingMode::Imm),
    (Instruction::Tax, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ldy, AddressingMode::Abs),
    (Instruction::Lda, AddressingMode::Abs),
    (Instruction::Ldx, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bcs, AddressingMode::Rel),
    (Instruction::Lda, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ldy, AddressingMode::Zpx),
    (Instruction::Lda, AddressingMode::Zpx),
    (Instruction::Ldx, AddressingMode::Zpy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Clv, AddressingMode::Imp),
    (Instruction::Lda, AddressingMode::Aby),
    (Instruction::Tsx, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Ldy, AddressingMode::Abx),
    (Instruction::Lda, AddressingMode::Abx),
    (Instruction::Ldx, AddressingMode::Aby),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cpy, AddressingMode::Imm),
    (Instruction::Cmp, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cpy, AddressingMode::Zp0),
    (Instruction::Cmp, AddressingMode::Zp0),
    (Instruction::Dec, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Iny, AddressingMode::Imp),
    (Instruction::Cmp, AddressingMode::Imm),
    (Instruction::Dex, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cpy, AddressingMode::Abs),
    (Instruction::Cmp, AddressingMode::Abs),
    (Instruction::Dec, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Bne, AddressingMode::Rel),
    (Instruction::Cmp, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cmp, AddressingMode::Zpx),
    (Instruction::Dec, AddressingMode::Zpx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cld, AddressingMode::Imp),
    (Instruction::Cmp, AddressingMode::Aby),
    (Instruction::Nop, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cmp, AddressingMode::Abx),
    (Instruction::Dec, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cpx, AddressingMode::Imm),
    (Instruction::Sbc, AddressingMode::Izx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Cpx, AddressingMode::Zp0),
    (Instruction::Sbc, AddressingMode::Zp0),
    (Instruction::Inc, AddressingMode::Zp0),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Inx, AddressingMode::Imp),
    (Instruction::Sbc, AddressingMode::Imm),
    (Instruction::Nop, AddressingMode::Imp),
    (Instruction::Sbc, AddressingMode::Imp),
    (Instruction::Cpx, AddressingMode::Abs),
    (Instruction::Sbc, AddressingMode::Abs),
    (Instruction::Inc, AddressingMode::Abs),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Beq, AddressingMode::Rel),
    (Instruction::Sbc, AddressingMode::Izy),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sbc, AddressingMode::Zpx),
    (Instruction::Inc, AddressingMode::Zpx),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sed, AddressingMode::Imp),
    (Instruction::Sbc, AddressingMode::Aby),
    (Instruction::Nop, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Unk, AddressingMode::Imp),
    (Instruction::Sbc, AddressingMode::Abx),
    (Instruction::Inc, AddressingMode::Abx),
    (Instruction::Unk, AddressingMode::Imp),
];
