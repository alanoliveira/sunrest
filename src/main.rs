#[macro_use]
mod log;

mod emulator;
mod input_devices;
mod ui;

fn main() {
    let cartridge = emulator::cartridge::open_rom(
        std::env::args()
            .nth(1)
            .expect("Please provide a path to a ROM file")
            .as_ref(),
    );
    let emulator = emulator::Emulator::new(cartridge);

    ui::Ui::<ui::engines::SdlEngine>::new(emulator).run();
}
