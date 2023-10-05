use super::*;

#[derive(Clone)]
pub struct TimeMachine {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: Status,
    signal: Option<Signal>,

    cycle: usize,
    busy_cycles: usize,
}

impl TimeMachine {
    pub fn save<M: Memory>(cpu: &Cpu<M>) -> Self {
        Self {
            a: cpu.a,
            x: cpu.x,
            y: cpu.y,
            pc: cpu.pc,
            sp: cpu.sp,
            p: cpu.p,
            signal: cpu.signal,

            cycle: cpu.cycle,
            busy_cycles: cpu.busy_cycles,
        }
    }

    pub fn load<M: Memory>(self, cpu: &mut Cpu<M>) {
        cpu.a = self.a;
        cpu.x = self.x;
        cpu.y = self.y;
        cpu.pc = self.pc;
        cpu.sp = self.sp;
        cpu.p = self.p;
        cpu.signal = self.signal;

        cpu.cycle = self.cycle;
        cpu.busy_cycles = self.busy_cycles;
    }
}
