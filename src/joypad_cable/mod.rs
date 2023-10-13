mod input_cable;
mod output_cable;
pub use input_cable::InputCable;
pub use output_cable::OutputCable;

pub trait JoypadCable {
    fn write(&mut self, state: u8);
}
