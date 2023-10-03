pub mod engines;

use crate::emulator;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
const SAMPLE_BUFFER_SIZE: usize = 2048;
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
    InputEvent {
        side: usize,
        button: JoypadButton,
        state: ButtonState,
    },
}

pub struct Ui<E: engines::UiEngine> {
    emulator: emulator::Emulator,
    engine: E,
    event_buffer: Vec<UiEvent>,
    state: UiState,

    joypad1: emulator::input_devices::StandardJoypad,
    joypad2: emulator::input_devices::StandardJoypad,
    sample_buffer: Vec<f32>,
}

impl<E: engines::UiEngine> Ui<E> {
    pub fn new(emulator: emulator::Emulator) -> Self {
        let mut engine = E::new();
        engine.set_title("sunrest");

        Self {
            emulator,
            engine,
            event_buffer: Vec::new(),
            state: UiState::Running,

            joypad1: emulator::input_devices::StandardJoypad::new(),
            joypad2: emulator::input_devices::StandardJoypad::new(),
            sample_buffer: Vec::with_capacity(SAMPLE_BUFFER_SIZE * 2),
        }
    }

    pub fn run(&mut self) {
        let mut prev_video_signal = self.emulator.video_signal();

        let sample_clock = 21_477_272 / SAMPLE_RATE;
        while self.state == UiState::Running {
            self.emulator.clock();
            self.emulator
                .clock_input_devices(&mut self.joypad1, &mut self.joypad2);

            let video_signal = self.emulator.video_signal();
            if video_signal != prev_video_signal {
                self.draw_point(
                    prev_video_signal.x,
                    prev_video_signal.y,
                    prev_video_signal.color,
                );

                if video_signal.x == SCREEN_WIDTH - 1 && video_signal.y == SCREEN_HEIGHT - 1 {
                    self.engine.present();
                    self.engine.poll_events(&mut self.event_buffer);
                    self.process_events();
                }

                prev_video_signal = video_signal;
            }

            if self.emulator.cycle % sample_clock == 0 {
                let sample = self.emulator.audio_signal().sample();
                if self.sample_buffer.len() < self.sample_buffer.capacity() {
                    self.sample_buffer.push(sample);
                }

                if self.engine.feed_samples(self.sample_buffer.as_slice()) {
                    self.sample_buffer.clear();
                }
            }
        }
    }

    fn process_events(&mut self) {
        for event in self.event_buffer.drain(..) {
            match event {
                UiEvent::Quit => self.state = UiState::Quit,
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
