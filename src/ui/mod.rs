pub mod engines;

mod settings;
pub use settings::Settings;

use super::*;
use emulator::input_devices::StandardJoypad;

const FPS: usize = 60;
const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
const SAMPLE_BUFFER_SIZE: usize = 512;
const SAMPLE_RATE: usize = 44100;

#[derive(PartialEq, Eq)]
enum UiState {
    Running,
    Quit,
}

pub enum JoypadButton {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

pub enum ButtonState {
    Pressed,
    Released,
}

pub enum UiEvent {
    Quit,
    SaveState,
    LoadState,
    InputEvent {
        side: usize,
        button: JoypadButton,
        state: ButtonState,
    },
}

pub struct Ui<E: engines::UiEngine> {
    emulator: emulator::Emulator,
    joypad1: StandardJoypad,
    joypad2: StandardJoypad,
    engine: E,
    state: UiState,
    settings: Settings,

    sample_buffer: Vec<f32>,
    emulator_state: Option<emulator::TimeMachine>,
}

impl<E: engines::UiEngine> Ui<E> {
    pub fn new(emulator: emulator::Emulator) -> Self {
        let mut engine = E::new();
        engine.set_title("sunrest");

        Self {
            emulator,
            joypad1: StandardJoypad::new(),
            joypad2: StandardJoypad::new(),
            engine,
            state: UiState::Running,
            settings: Settings::from_env(),

            sample_buffer: Vec::with_capacity(SAMPLE_BUFFER_SIZE),
            emulator_state: None,
        }
    }

    pub fn run(&mut self) {
        let mut prev_video_signal = self.emulator.video_signal();

        let mut fps_calc = FpsCalc::new();
        let sample_clock_ratio = 21_477_272 as f32 / SAMPLE_RATE as f32;
        let sample_clock = (sample_clock_ratio * self.settings.speed) as usize;
        let frame_skip = (self.settings.speed as usize).saturating_sub(1);
        while self.state == UiState::Running {
            if self.sample_buffer.len() < self.sample_buffer.capacity() {
                self.emulator.clock();
                self.emulator
                    .clock_input_devices(&mut self.joypad1, &mut self.joypad2);

                let video_signal = self.emulator.video_signal();
                if video_signal != prev_video_signal {
                    prev_video_signal = video_signal;
                    self.draw_point(
                        prev_video_signal.x,
                        prev_video_signal.y,
                        prev_video_signal.color,
                    );

                    if video_signal.x == SCREEN_WIDTH - 1 && video_signal.y == SCREEN_HEIGHT - 1 {
                        if fps_calc.frame % (1 + frame_skip) == 0 {
                            self.engine.present();
                        }

                        if let Some(fps) = fps_calc.update() {
                            self.append_title(&format!("{:.02} fps", fps));
                        }

                        self.process_events();
                    }
                }

                if self.emulator.cycle % sample_clock == 0 {
                    let sample = self.emulator.audio_signal().sample();
                    self.sample_buffer.push(sample * self.settings.volume);
                }
            } else if self.engine.feed_samples(self.sample_buffer.as_slice()) {
                self.sample_buffer.clear();
            }
        }
    }

    fn process_events(&mut self) {
        for event in self.engine.poll_events() {
            match event {
                UiEvent::Quit => self.state = UiState::Quit,
                UiEvent::SaveState => {
                    self.emulator_state = Some(self.emulator.save_state());
                }
                UiEvent::LoadState => {
                    if let Some(state) = self.emulator_state.clone() {
                        self.emulator.load_state(state);
                    }
                }
                UiEvent::InputEvent {
                    side,
                    button,
                    state,
                } => {
                    let joypad = match side {
                        0 => &mut self.joypad1,
                        1 => &mut self.joypad2,
                        _ => panic!("Invalid joypad side"),
                    };

                    let button_state = match state {
                        ButtonState::Pressed => true,
                        ButtonState::Released => false,
                    };

                    match button {
                        JoypadButton::A => joypad.a = button_state,
                        JoypadButton::B => joypad.b = button_state,
                        JoypadButton::Select => joypad.select = button_state,
                        JoypadButton::Start => joypad.start = button_state,
                        JoypadButton::Up => joypad.up = button_state,
                        JoypadButton::Down => joypad.down = button_state,
                        JoypadButton::Left => joypad.left = button_state,
                        JoypadButton::Right => joypad.right = button_state,
                    }
                }
            }
        }
    }

    fn append_title(&mut self, message: &str) {
        let mut title = String::from("sunrest");
        title.push_str(" - ");
        title.push_str(message);
        self.engine.set_title(&title);
    }

    fn draw_point(&mut self, x: usize, y: usize, color: emulator::Color) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.engine.draw_point(x, y, color);
        }
    }
}

struct FpsCalc {
    last_frame_time: std::time::Instant,
    frame: usize,
}

impl FpsCalc {
    fn new() -> Self {
        Self {
            last_frame_time: std::time::Instant::now(),
            frame: 0,
        }
    }

    fn update(&mut self) -> Option<f32> {
        self.frame += 1;
        if self.frame % FPS != 0 {
            return None;
        }

        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame_time;
        let fps = FPS as f32 / elapsed.as_secs_f32();
        self.last_frame_time = now;
        Some(fps)
    }
}
