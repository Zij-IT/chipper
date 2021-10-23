#![warn(clippy::pedantic)]

mod chip;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

fn main() {
    let rom = include_bytes!("../test_opcode.ch8");
    let mut chip8 = chip::Chip8::new();
    chip8.load_rom(rom).unwrap();

    let mut window = Window::new("Test - ESC to exit", 64, 32, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    let mut last_instruction_run_time = Instant::now();
    let mut last_drawn_time = Instant::now();

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip8.cycle().unwrap();
            last_instruction_run_time = Instant::now();
        }

        if Instant::now() - last_drawn_time > Duration::from_millis(10) {
            let chip_buffer = chip8
                .frame_buffer()
                .iter()
                .flatten()
                .copied()
                .map(|x| if x == 1 { 0xFFFF_FFFF_u32 } else { 0x0_u32 })
                .collect::<Vec<_>>();

            window.update_with_buffer(&chip_buffer, 64, 32).unwrap();

            last_drawn_time = Instant::now();
        }
    }
}
