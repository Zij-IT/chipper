#![warn(clippy::pedantic)]

mod chip;
mod sdl2_wrapper;

use chip::Chip8;

use anyhow::Result;
use anyhow::Error;
use sdl2_wrapper::Sdl2Wrapper;

const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;
const SCALE: usize = 20;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| Error::msg("No rom path provided. Exiting."))?;
    let rom = std::fs::read(path)?;

    let mut sdl = Sdl2Wrapper::new()?;
    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom)?;
    chip8.run(&mut sdl)?;

    Ok(())
}
