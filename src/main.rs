#![warn(clippy::pedantic)]

mod chip;

use std::time::Duration;

const SLEEP_DURATION: Duration = Duration::from_millis(2);

fn main() {
    let rom = include_bytes!("../test_opcode.ch8");

    let sdl_context = sdl2::init().unwrap();

    let mut chip8 = chip::Chip8::new(&sdl_context);
    let _ = chip8.load_rom(rom);

    while !chip8.should_quit() {
        chip8.poll_input();
        let _ = chip8.cycle();
        chip8.draw_on_screen();
        std::thread::sleep(SLEEP_DURATION);
    }
}
