#[macro_use]
mod log;

mod emulator;
mod ui;

fn main() {
    let emulator = emulator::Emulator::new(
        std::env::args()
            .nth(1)
            .expect("Please provide a path to a ROM file"),
    );

    ui::Ui::<ui::engines::SdlEngine>::new(emulator).run();
}
