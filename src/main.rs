#![warn(clippy::pedantic)]

mod chip;
mod sdl2_wrapper;

use chip::Chip8;

use anyhow::Result;
use sdl2_wrapper::Sdl2Wrapper;
use std::time::Duration;

const SLEEP_DURATION: Duration = Duration::from_millis(2);
const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;
const SCALE: usize = 20;

fn main() -> Result<()> {
    let rom = include_bytes!("../test_opcode.ch8");

    let mut sdl = Sdl2Wrapper::new()?;
    let mut chip8 = Chip8::new();
    chip8.load_rom(rom)?;

    loop {
        let (quit_signal, keys) = sdl.poll_input();

        chip8.cycle(keys)?;
        sdl.draw_on_canvas(chip8.get_frame_buffer())?;

        std::thread::sleep(SLEEP_DURATION);

        if quit_signal {
            break;
        }
    }

    Ok(())
}
