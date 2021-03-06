#![allow(clippy::cast_possible_truncation)]

use super::chip::FrameBuffer;
use super::{CHIP8_HEIGHT, CHIP8_WIDTH, SCALE};

use anyhow::Error;
use anyhow::Result;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

use std::convert::TryFrom;

pub struct Sdl2Wrapper {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    audio_device: AudioDevice<SquareWave>,
}

impl Sdl2Wrapper {
    pub fn new() -> Result<Self> {
        let sdl_context = Self::create_sdl_context()?;
        let audio_device = Self::setup_audio_device(&sdl_context)?;
        let canvas = Self::setup_canvas(&sdl_context)?;
        let event_pump = sdl_context.event_pump().map_err(Error::msg)?;

        Ok(Self {
            canvas,
            event_pump,
            audio_device,
        })
    }

    pub fn draw_on_canvas(&mut self, buffer: &FrameBuffer) -> Result<()> {
        // TODO:
        // Clean this up and add checks for proper conversion between u32 and i32
        for (y, row) in buffer.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x * SCALE) as u32;
                let y = (y * SCALE) as u32;

                let color = if col == 0 {
                    sdl2::pixels::Color::RGB(0, 0, 0)
                } else {
                    sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF)
                };

                self.canvas.set_draw_color(color);
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        TryFrom::try_from(x)?,
                        TryFrom::try_from(y)?,
                        SCALE as u32,
                        SCALE as u32,
                    ))
                    .map_err(Error::msg)?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }

    pub fn poll_input(&mut self) -> [bool; 16] {
        let mut pressed_keys = [false; 16];

        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Self::translate_scancode)
            .for_each(|key| {
                pressed_keys[key as usize] = true;
            });

        pressed_keys
    }

    pub fn beep(&mut self) {
        self.audio_device.resume();
    }

    pub fn stop_beep(&mut self) {
        self.audio_device.pause();
    }

    fn create_sdl_context() -> Result<Sdl> {
        sdl2::init().map_err(Error::msg)
    }

    fn setup_audio_device(sdl_context: &sdl2::Sdl) -> Result<AudioDevice<SquareWave>> {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: (240.0 / f64::from(spec.freq)) as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .map_err(Error::msg)?;

        Ok(device)
    }

    fn setup_canvas(sdl_context: &sdl2::Sdl) -> Result<Canvas<Window>> {
        let video = sdl_context.video().map_err(Error::msg)?;

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

    pub fn translate_scancode(key: Scancode) -> Option<u8> {
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

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out {
            *x = if self.phase < 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
