#[macro_use]
mod log;

mod emulator;
mod joypad_handler;
mod ui;
use std::path::PathBuf;

use emulator::input_devices;
use joypad_cable::JoypadCable;
mod joypad_cable;
use clap::{arg, value_parser, Command};

fn cli() -> Command {
    clap::Command::new("sunrest")
        .arg(clap::arg!(<ROM> "Path to a ROM file").value_parser(value_parser!(PathBuf)))
        .arg(arg!(--volume <num> "Volume of the audio").value_parser(value_parser!(f32)))
        .arg(arg!(--speed <num> "Speed of the emulation").value_parser(value_parser!(f32)))
        .arg(
            arg!(--replay <FILE> "Path to the replay file")
                .conflicts_with("record")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(--record <FILE> "Path to save the replay file")
                .conflicts_with("replay")
                .value_parser(value_parser!(PathBuf)),
        )
}

fn main() {
    let matches = cli().get_matches();

    let mut settings = ui::Settings::from_env();
    if let Some(volume) = matches.get_one::<f32>("volume") {
        settings.volume = *volume;
    }
    if let Some(speed) = matches.get_one::<f32>("speed") {
        settings.speed = *speed;
    }

    let rom_path = matches.get_one::<PathBuf>("ROM").unwrap();
    let cartridge = emulator::cartridge::open_rom(rom_path);

    let mut emulator = emulator::Emulator::new(cartridge);
    let joypad1 = joypad_handler::JoypadHandler::new();
    let joypad2 = joypad_handler::JoypadHandler::new();
    emulator.connect_port1(Some(Box::new(joypad1.clone())));
    emulator.connect_port2(Some(Box::new(joypad2.clone())));

    let mut ui = ui::Ui::<ui::engines::SdlEngine>::new(emulator, settings);

    let joypad1_cable: Box<dyn JoypadCable> =
        if let Some(replay_path) = matches.get_one::<PathBuf>("replay") {
            let file = std::fs::File::open(replay_path).unwrap();
            Box::new(joypad_cable::InputCable::new(joypad1, file))
        } else if let Some(record_path) = matches.get_one::<PathBuf>("record") {
            let file = std::fs::File::create(record_path).unwrap();
            Box::new(joypad_cable::OutputCable::new(joypad1, file))
        } else {
            Box::new(joypad1)
        };

    ui.joypad1_cable = Some(joypad1_cable);
    ui.joypad2_cable = Some(Box::new(joypad2));
    ui.run();
}
