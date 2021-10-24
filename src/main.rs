#![warn(clippy::pedantic)]

mod chip;
mod sdl2_wrapper;

use chip::Chip8;

use anyhow::Result;
use sdl2_wrapper::Sdl2Wrapper;

const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;
const SCALE: usize = 20;

fn main() -> Result<()> {
    let rom = include_bytes!("../games/hidden.ch8");

    let mut sdl = Sdl2Wrapper::new()?;
    let mut chip8 = Chip8::new();
    chip8.load_rom(rom)?;
    chip8.run(&mut sdl)?;

    Ok(())
}
