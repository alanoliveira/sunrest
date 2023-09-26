pub mod engines;

use crate::emulator;

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;

#[derive(PartialEq, Eq)]
enum UiState {
    Running,
    Quit,
}

pub enum UiEvent {
    Quit,
}

pub struct Ui<E: engines::UiEngine> {
    emulator: emulator::Emulator,
    engine: E,
    event_buffer: Vec<UiEvent>,
    state: UiState,
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
        }
    }

    pub fn run(&mut self) {
        while self.state == UiState::Running {
            self.emulator.clock();
            let video_signal = self.emulator.video_signal();
            self.draw_point(video_signal.x, video_signal.y, video_signal.color);

            if video_signal.x == SCREEN_WIDTH - 1 && video_signal.y == SCREEN_HEIGHT - 1 {
                self.engine.present();
                self.engine.poll_events(&mut self.event_buffer);
                self.process_events();
            }
        }
    }

    fn process_events(&mut self) {
        for event in self.event_buffer.drain(..) {
            match event {
                UiEvent::Quit => self.state = UiState::Quit,
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
