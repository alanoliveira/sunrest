mod sdl;

use super::*;

pub use sdl::SdlEngine;

pub trait UiEngine {
    fn new() -> Self;
    fn draw_point(&mut self, x: usize, y: usize, color: emulator::Color);
    fn present(&mut self);
    fn set_title(&mut self, title: &str);
    fn poll_events(&mut self, event_buffer: &mut Vec<UiEvent>);
}
