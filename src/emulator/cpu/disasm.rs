use super::opcodes::{AddressingMode, Instruction, OPCODES};

pub trait DisasmMemory {
    fn read(&self, addr: u16) -> u8;
}

pub struct DisasmData {
    addr: u16,
    instruction: Instruction,
    addr_mode: AddressingMode,
    raw_data: Vec<u8>,
}

impl ToString for DisasmData {
    fn to_string(&self) -> String {
        format!(
            "{:04X} {:12} {}",
            self.addr,
            AsmFormatter(self).to_string(),
            format_args!("{:02X?}", self.raw_data)
        )
    }
}

pub struct Disasm<'a> {
    mem: &'a dyn DisasmMemory,
    current_addr: u16,
}

impl Disasm<'_> {
    pub fn new(mem: &dyn DisasmMemory, start_addr: u16) -> Disasm {
        Disasm {
            mem,
            current_addr: start_addr,
        }
    }

    pub fn current_addr(&self) -> u16 {
        self.current_addr
    }

    pub fn disasm_next(&mut self) -> String {
        let address = self.current_addr();
        let mut raw_data = Vec::with_capacity(3);

        raw_data.push(self.next_byte());
        let (instruction, addr_mode) = OPCODES[raw_data[0] as usize];
        for _ in 0..addr_mode.len() {
            raw_data.push(self.next_byte());
        }

        DisasmData {
            addr: address,
            instruction,
            addr_mode,
            raw_data,
        }
        .to_string()
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.mem.read(self.current_addr);
        self.current_addr = self.current_addr.wrapping_add(1);
        byte
    }
}

struct AsmFormatter<'a>(&'a DisasmData);

impl std::fmt::Display for AsmFormatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?} ", self.0.instruction).to_uppercase())?;
        match self.0.addr_mode {
            AddressingMode::Imm => write!(f, "#${:02X}", self.0.raw_data[1]),
            AddressingMode::Zp0 => write!(f, "${:02X}", self.0.raw_data[1]),
            AddressingMode::Zpx => write!(f, "${:02X},X", self.0.raw_data[1]),
            AddressingMode::Zpy => write!(f, "${:02X},Y", self.0.raw_data[1]),
            AddressingMode::Izx => write!(f, "(${:02X},X)", self.0.raw_data[1]),
            AddressingMode::Izy => write!(f, "(${:02X}),Y", self.0.raw_data[1]),
            AddressingMode::Abs => {
                write!(f, "${:02X}{:02X}", self.0.raw_data[2], self.0.raw_data[1])
            }
            AddressingMode::Abx => {
                write!(f, "${:02X}{:02X},X", self.0.raw_data[2], self.0.raw_data[1])
            }
            AddressingMode::Aby => {
                write!(f, "${:02X}{:02X},Y", self.0.raw_data[2], self.0.raw_data[1])
            }
            AddressingMode::Ind => {
                write!(f, "(${:02X}{:02X})", self.0.raw_data[2], self.0.raw_data[1])
            }
            AddressingMode::Rel => {
                let offset = self.0.raw_data[1] as i8 + self.0.raw_data.len() as i8;
                let rel_addr = self.0.addr.wrapping_add(offset as u16);
                write!(f, "${:04X}", rel_addr)
            }
            AddressingMode::Imp => write!(f, "IMP"),
        }
    }
}
