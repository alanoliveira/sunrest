#[macro_use]
mod log;

mod emulator;

fn main() {
    let mut emulator = emulator::Emulator::new(
        std::env::args()
            .nth(1)
            .expect("Please provide a path to a ROM file"),
    );

    loop {
        emulator.clock();
    }
}
