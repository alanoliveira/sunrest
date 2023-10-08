mod sdl;

use super::*;

pub use sdl::SdlEngine;

pub trait UiEngine {
    type EventIter<'a>: Iterator<Item = UiEvent> + 'a
    where
        Self: 'a;

    fn new() -> Self;
    fn draw_point(&mut self, x: usize, y: usize, color: emulator::Color);
    fn present(&mut self);
    fn set_title(&mut self, title: &str);
    fn poll_events(&mut self) -> Self::EventIter<'_>;
    fn feed_samples(&mut self, samples: &[f32]) -> bool;
}
