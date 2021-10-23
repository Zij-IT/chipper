#![warn(clippy::pedantic)]

mod chip;
mod sdl2_wrapper;

use chip::Chip8;
use sdl2_wrapper::{draw_on_canvas, setup_canvas, poll_input};
use std::time::Duration;

const SLEEP_DURATION: Duration = Duration::from_millis(2);
const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;
const SCALE: usize = 20;

fn main() {
    let rom = include_bytes!("../test_opcode.ch8");

    let sdl_context = sdl2::init().unwrap();
    let mut canvas = setup_canvas(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    let _ = chip8.load_rom(rom);

    let mut quit = false;
    while !quit {
        let (quit_signal, keys) = poll_input(&mut event_pump);
        quit = quit_signal;

        let _ = chip8.cycle(keys);
        draw_on_canvas(&mut canvas, chip8.get_frame_buffer());

        std::thread::sleep(SLEEP_DURATION);
    }
}
