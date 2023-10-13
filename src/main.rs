#[macro_use]
mod log;

mod emulator;
mod joypad_handler;
mod ui;
use emulator::input_devices;
use std::borrow::BorrowMut;
mod joypad_cable;

fn main() {
    let cartridge = emulator::cartridge::open_rom(
        std::env::args()
            .nth(1)
            .expect("Please provide a path to a ROM file")
            .as_ref(),
    );

    let mut emulator = emulator::Emulator::new(cartridge);
    let joypad1 = joypad_handler::JoypadHandler::new();
    let joypad2 = joypad_handler::JoypadHandler::new();
    emulator.connect_port1(Some(Box::new(joypad1.clone())));
    emulator.connect_port2(Some(Box::new(joypad2.clone())));

    let mut ui = ui::Ui::<ui::engines::SdlEngine>::new(emulator);

    ui.joypad1_cable = Some(Box::new(joypad1));
    ui.joypad2_cable = Some(Box::new(joypad2));
    ui.run();
}
