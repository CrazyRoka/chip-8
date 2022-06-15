use sdl2::keyboard::Keycode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChipErrors {
    #[error("Unknown opcode {0:04x}")]
    UnknownOpcode(u16),
    #[error("Unknown key code {0}")]
    UnknownKeycode(Keycode),
}
