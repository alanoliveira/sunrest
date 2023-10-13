mod fps_calc;
mod settings;
use super::*;

pub mod engines;
pub use settings::Settings;

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

pub enum UiEvent {
    Quit,
    KeyPress(i32),
    KeyRelease(i32),
}

pub struct Ui<E: engines::UiEngine> {
    emulator: emulator::Emulator,
    engine: E,
    state: UiState,
    settings: Settings,
    joypad1_state: input_devices::standard_joypad::State,
    joypad2_state: input_devices::standard_joypad::State,
    pub joypad1_cable: Option<Box<dyn joypad_cable::JoypadCable>>,
    pub joypad2_cable: Option<Box<dyn joypad_cable::JoypadCable>>,

    sample_buffer: Vec<f32>,
    emulator_state: Option<emulator::TimeMachine>,
    base_title: String,
}

impl<E: engines::UiEngine> Ui<E> {
    pub fn new(emulator: emulator::Emulator) -> Self {
        let mut engine = E::new();
        let base_title = format!("sunrest - {}", emulator.rom_info().name);
        engine.set_title(&base_title);

        Self {
            emulator,
            engine,
            state: UiState::Running,
            settings: Settings::from_env(),
            joypad1_state: Default::default(),
            joypad2_state: Default::default(),
            joypad1_cable: None,
            joypad2_cable: None,

            sample_buffer: Vec::with_capacity(SAMPLE_BUFFER_SIZE),
            emulator_state: None,
            base_title,
        }
    }

    pub fn run(&mut self) {
        let mut prev_video_signal = self.emulator.video_signal();

        let mut fps_calc = fps_calc::FpsCalc::new(FPS);
        let sample_clock_ratio = 21_477_272 as f32 / SAMPLE_RATE as f32;
        let sample_clock = (sample_clock_ratio * self.settings.speed) as usize;
        let frame_skip = (self.settings.speed as usize).saturating_sub(1);
        while self.state == UiState::Running {
            if self.sample_buffer.len() < self.sample_buffer.capacity() {
                self.emulator.clock();

                let video_signal = self.emulator.video_signal();
                if video_signal != prev_video_signal {
                    prev_video_signal = video_signal;
                    self.draw_point(
                        prev_video_signal.x,
                        prev_video_signal.y,
                        prev_video_signal.color,
                    );

                    if video_signal.x == SCREEN_WIDTH - 1 && video_signal.y == SCREEN_HEIGHT - 1 {
                        if fps_calc.frame() % (1 + frame_skip) == 0 {
                            self.engine.present();
                        }

                        if let Some(fps) = fps_calc.update() {
                            self.set_title(&format!("{:.02} fps", fps));
                        }

                        self.process_events();
                        if let Some(joy1) = self.joypad1_cable.as_deref_mut() {
                            joy1.write(self.joypad1_state.into())
                        }
                        if let Some(joy2) = self.joypad2_cable.as_deref_mut() {
                            joy2.write(self.joypad2_state.into())
                        }
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
                UiEvent::Quit | UiEvent::KeyPress(27) => self.state = UiState::Quit,
                UiEvent::KeyPress(keycode) if keycode == '[' as i32 => {
                    self.emulator_state = Some(self.emulator.save_state())
                }
                UiEvent::KeyPress(keycode) if keycode == ']' as i32 => {
                    if let Some(state) = self.emulator_state.as_ref().cloned() {
                        self.emulator.load_state(state);
                    }
                }
                UiEvent::KeyPress(keycode) | UiEvent::KeyRelease(keycode) => {
                    let is_pressed = matches!(event, UiEvent::KeyPress(_));
                    match keycode {
                        _ if keycode == 'w' as i32 => self.joypad1_state.up = is_pressed,
                        _ if keycode == 'a' as i32 => self.joypad1_state.left = is_pressed,
                        _ if keycode == 's' as i32 => self.joypad1_state.down = is_pressed,
                        _ if keycode == 'd' as i32 => self.joypad1_state.right = is_pressed,
                        _ if keycode == 'k' as i32 => self.joypad1_state.b = is_pressed,
                        _ if keycode == 'j' as i32 => self.joypad1_state.a = is_pressed,
                        _ if keycode == '\r' as i32 => self.joypad1_state.start = is_pressed,
                        _ if keycode == ' ' as i32 => self.joypad1_state.select = is_pressed,
                        _ => {}
                    }
                }
            }
        }
    }

    fn set_title(&mut self, message: &str) {
        self.engine
            .set_title(&format!("{} - {}", self.base_title, message))
    }

    fn draw_point(&mut self, x: usize, y: usize, color: emulator::Color) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.engine.draw_point(x, y, color);
        }
    }
}
