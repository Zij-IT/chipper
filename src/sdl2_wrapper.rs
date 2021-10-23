use super::chip::FrameBuffer;
use super::{CHIP8_HEIGHT, CHIP8_WIDTH, SCALE};

use anyhow::Result;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn create_sdl_context() -> Result<sdl2::Sdl> {
    sdl2::init().map_err(|e| Sdl2Error::UnableToBuildSdl(e).into())
}

pub fn setup_canvas(sdl_context: &sdl2::Sdl) -> Result<Canvas<Window>> {
    let video = sdl_context
        .video()
        .map_err(Sdl2Error::UnableToBuildVideo)?;

    let window = video
        .window(
            "Chipper: Chip8 Emulator",
            (SCALE * CHIP8_WIDTH) as u32,
            (SCALE * CHIP8_HEIGHT) as u32,
        )
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    Ok(canvas)
}

pub fn draw_on_canvas(canvas: &mut Canvas<Window>, buffer: &FrameBuffer) -> Result<()> {
    for (y, row) in buffer.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            let x = (x * SCALE) as u32;
            let y = (y * SCALE) as u32;

            let color = if col == 0 {
                sdl2::pixels::Color::RGB(0, 0, 0)
            } else {
                sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF)
            };

            canvas.set_draw_color(color);
            canvas
                .fill_rect(sdl2::rect::Rect::new(
                    x as i32,
                    y as i32,
                    SCALE as u32,
                    SCALE as u32,
                ))
                .map_err(Sdl2Error::UnableToDraw)?;
        }
    }

    canvas.present();
    Ok(())
}

pub fn poll_input(event_pump: &mut sdl2::EventPump) -> (bool, [bool; 16]) {
    let mut pressed_keys = [false; 16];

    let quit = event_pump
        .poll_iter()
        .any(|event| matches!(event, sdl2::event::Event::Quit { .. }));

    event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(translate_scancode)
        .for_each(|key| {
            pressed_keys[key as usize] = true;
        });

    (quit, pressed_keys)
}

fn translate_scancode(key: Scancode) -> Option<u8> {
    match key {
        Scancode::Num1 => Some(0x1),
        Scancode::Num2 => Some(0x2),
        Scancode::Num3 => Some(0x3),
        Scancode::Num4 => Some(0xc),
        Scancode::Q => Some(0x4),
        Scancode::W => Some(0x5),
        Scancode::E => Some(0x6),
        Scancode::R => Some(0xd),
        Scancode::A => Some(0x7),
        Scancode::S => Some(0x8),
        Scancode::D => Some(0x9),
        Scancode::F => Some(0xe),
        Scancode::Z => Some(0xa),
        Scancode::X => Some(0x0),
        Scancode::C => Some(0xb),
        Scancode::V => Some(0xf),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sdl2Error {
    UnableToBuildSdl(String),
    UnableToBuildVideo(String),
    UnableToBuildEventPump(String),
    UnableToDraw(String),
}

impl std::fmt::Display for Sdl2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            Self::UnableToBuildSdl(e) => format!("Unable to build SDL: {}", e),
            Self::UnableToBuildVideo(e) => format!("Unable to build SDL Context: {}", e),
            Self::UnableToBuildEventPump(e) => format!("Unable to build SDL Event Pump: {}", e),
            Self::UnableToDraw(e) => format!("Unable to draw on SDL canvas: {}", e),
        };

        write!(f, "{}", error_msg)
    }
}

impl std::error::Error for Sdl2Error {}
