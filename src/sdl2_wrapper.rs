use super::chip::FrameBuffer;
use super::{CHIP8_HEIGHT, CHIP8_WIDTH, SCALE};
use sdl2::keyboard::Scancode;

pub enum Sdl2Errors {

}

pub fn setup_canvas(sdl_context: &sdl2::Sdl) -> sdl2::render::Canvas<sdl2::video::Window> {
    let video = sdl_context.video().unwrap();
    let window = video
        .window(
            "title",
            (SCALE * CHIP8_WIDTH) as u32,
            (SCALE * CHIP8_HEIGHT) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    canvas
}

pub fn draw_on_canvas(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    buffer: &FrameBuffer,
) {
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
            let _ = canvas.fill_rect(sdl2::rect::Rect::new(
                x as i32,
                y as i32,
                SCALE as u32,
                SCALE as u32,
            ));
        }
    }

    canvas.present();
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
