use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct Keyboard {
    quit: bool,
    keys: [bool; 16],
    events: sdl2::EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Self {
            quit: false,
            keys: [false; 16],
            events: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn poll_input(&mut self) {
        self.quit = self
            .events
            .poll_iter()
            .any(|event| matches!(event, Event::Quit { .. }));

        let keys = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Self::translate_scancode)
            .collect::<Vec<_>>();

        for key in keys {
            self.keys[key as usize] = true;
        }
    }

    pub fn get_next_key(&self) -> Option<u8> {
        self.keys
            .iter()
            .copied()
            .zip(0_u8..)
            .find(|(pressed, _idx)| *pressed)
            .map(|(_pressed, idx)| idx)
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
}
