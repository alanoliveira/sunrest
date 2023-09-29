use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::EventPump;

use crate::ui::*;

use super::UiEngine;

type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

struct SdlContext {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator,
}

impl SdlContext {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL");
        let video_subsystem = sdl_context.video().expect("Failed to initialize SDL video");

        let window = video_subsystem
            .window(
                "rust-sdl2 demo: Video",
                SCREEN_WIDTH as u32 * 4,
                SCREEN_HEIGHT as u32 * 4,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create window");

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create canvas");
        canvas.set_scale(4.0, 4.0).unwrap();

        let event_pump = sdl_context
            .event_pump()
            .expect("Failed to get SDL event pump");

        let texture_creator = canvas.texture_creator();

        Self {
            event_pump,
            canvas,
            texture_creator,
        }
    }
}

pub struct SdlEngine {
    game_screen_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
    game_screen_texture: Texture<'static>,
    canvas: &'static mut Canvas<Window>,
    event_pump: &'static mut EventPump,
}

impl UiEngine for SdlEngine {
    fn new() -> Self {
        let sdl_context = Box::leak(Box::new(SdlContext::new()));

        let game_screen_texture = sdl_context
            .texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .expect("Failed to create texture");

        Self {
            game_screen_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            game_screen_texture,
            canvas: &mut sdl_context.canvas,
            event_pump: &mut sdl_context.event_pump,
        }
    }

    fn draw_point(&mut self, x: usize, y: usize, color: emulator::Color) {
        let offset = (y * SCREEN_WIDTH + x) * 3;
        self.game_screen_buffer[offset] = color.0;
        self.game_screen_buffer[offset + 1] = color.1;
        self.game_screen_buffer[offset + 2] = color.2;
    }

    fn present(&mut self) {
        self.game_screen_texture
            .update(None, &self.game_screen_buffer, SCREEN_WIDTH * 3)
            .expect("Failed to update texture");
        self.canvas
            .copy(&self.game_screen_texture, None, None)
            .expect("Failed to copy texture");
        self.canvas.present();
    }

    fn set_title(&mut self, title: &str) {
        self.canvas
            .window_mut()
            .set_title(title)
            .expect("Failed to set title");
    }

    fn poll_events(&mut self, event_buffer: &mut Vec<UiEvent>) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => event_buffer.push(UiEvent::Quit),

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let state = if matches!(event, Event::KeyDown { .. }) {
                        ButtonState::Pressed
                    } else {
                        ButtonState::Released
                    };
                    let joypad_button_evt = match keycode {
                        // Joypad 1
                        Keycode::W => Some((0, JoypadButton::Up)),
                        Keycode::A => Some((0, JoypadButton::Left)),
                        Keycode::S => Some((0, JoypadButton::Down)),
                        Keycode::D => Some((0, JoypadButton::Right)),
                        Keycode::J => Some((0, JoypadButton::A)),
                        Keycode::K => Some((0, JoypadButton::B)),
                        Keycode::Return => Some((0, JoypadButton::Start)),
                        Keycode::Backspace => Some((0, JoypadButton::Select)),

                        // Joypad 2
                        // Keycode::Up => Some((1, JoypadButton::Up)),
                        // Keycode::Left => Some((1, JoypadButton::Left)),
                        // Keycode::Down => Some((1, JoypadButton::Down)),
                        // Keycode::Right => Some((1, JoypadButton::Right)),
                        // Keycode::Z => Some((1, JoypadButton::A)),
                        // Keycode::X => Some((1, JoypadButton::B)),
                        // Keycode::C => Some((1, JoypadButton::Start)),
                        // Keycode::V => Some((1, JoypadButton::Select)),
                        _ => None,
                    };
                    if let Some((side, button)) = joypad_button_evt {
                        event_buffer.push(UiEvent::InputEvent {
                            side,
                            button,
                            state,
                        })
                    }
                }
                _ => {}
            }
        }
    }
}
