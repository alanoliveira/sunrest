#[macro_use]
mod log;

mod emulator;

fn main() {
    let mut emulator = emulator::Emulator::new();

    loop {
        emulator.clock();
    }
}
