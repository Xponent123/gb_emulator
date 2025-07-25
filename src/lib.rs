#![crate_type = "lib"]

pub use crate::gpu::{SCREEN_H, SCREEN_W};
pub use crate::keypad::KeypadKey;
pub use crate::sound::AudioPlayer;

pub mod device;

mod cpu;
mod gbmode;
mod gpu;
mod instructions;
mod keypad;
mod mbc;
mod mmu;
mod register;
mod sound;
mod timer;

pub type StrResult<T> = Result<T, &'static str>;
