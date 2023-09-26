use super::*;

macro_rules! push_data {
    ($data:expr, $ins:expr, $mode:expr, [$($args:expr),*]) => {{
        $data.push(util::opcode_lookup($ins, $mode));
        $( $data.push($args); )*
    }};
}

struct TestDisasmMem(Vec<u8>);

impl DisasmMemory for TestDisasmMem {
    fn read(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }
}

#[test]
fn test_disasm_data_string() {
    let mut data = Vec::new();
    push_data!(data, Instruction::Rti, AddressingMode::Imp, []);
    push_data!(data, Instruction::Adc, AddressingMode::Imm, [0x01]);
    push_data!(data, Instruction::Asl, AddressingMode::Zp0, [0x02]);
    push_data!(data, Instruction::Eor, AddressingMode::Zpx, [0x03]);
    push_data!(data, Instruction::Ldx, AddressingMode::Zpy, [0x04]);
    push_data!(data, Instruction::Bcc, AddressingMode::Rel, [0x05]);
    push_data!(data, Instruction::Ldy, AddressingMode::Abs, [0x06, 0x07]);
    push_data!(data, Instruction::Lsr, AddressingMode::Abx, [0x08, 0x09]);
    push_data!(data, Instruction::Ora, AddressingMode::Aby, [0x0A, 0x0B]);
    push_data!(data, Instruction::Jmp, AddressingMode::Ind, [0x0C, 0x0D]);
    push_data!(data, Instruction::Cmp, AddressingMode::Izx, [0x0E]);
    push_data!(data, Instruction::And, AddressingMode::Izy, [0x0F]);


    let mem = TestDisasmMem(data);
    let mut disasm = Disasm::new(&mem, 0x0000);
    assert_eq!(disasm.disasm_next(), "0000 RTI IMP      [40]");
    assert_eq!(disasm.disasm_next(), "0001 ADC #$01     [69, 01]");
    assert_eq!(disasm.disasm_next(), "0003 ASL $02      [06, 02]");
    assert_eq!(disasm.disasm_next(), "0005 EOR $03,X    [55, 03]");
    assert_eq!(disasm.disasm_next(), "0007 LDX $04,Y    [B6, 04]");
    assert_eq!(disasm.disasm_next(), "0009 BCC $0010    [90, 05]");
    assert_eq!(disasm.disasm_next(), "000B LDY $0706    [AC, 06, 07]");
    assert_eq!(disasm.disasm_next(), "000E LSR $0908,X  [5E, 08, 09]");
    assert_eq!(disasm.disasm_next(), "0011 ORA $0B0A,Y  [19, 0A, 0B]");
    assert_eq!(disasm.disasm_next(), "0014 JMP ($0D0C)  [6C, 0C, 0D]");
    assert_eq!(disasm.disasm_next(), "0017 CMP ($0E,X)  [C1, 0E]");
    assert_eq!(disasm.disasm_next(), "0019 AND ($0F),Y  [31, 0F]");

    let mut disasm = Disasm::new(&mem, 0x0009);
    assert_eq!(disasm.disasm_next(), "0009 BCC $0010    [90, 05]");
}
