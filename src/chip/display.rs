use crate::CHIP8_HEIGHT;
use crate::CHIP8_WIDTH;

pub type FrameBuffer = [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT];

#[derive(Debug, PartialEq, Eq)]
pub struct Display {
    buffer: FrameBuffer,
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: u8) -> bool {
        let x = x % CHIP8_WIDTH;
        let y = y % CHIP8_HEIGHT;

        let erased = (self.buffer[y][x] & pixel) == 1;
        self.buffer[y][x] ^= pixel;

        erased
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        &self.buffer
    }

    #[cfg(test)]
    pub fn fill_buffer(&mut self) {
        self.buffer = [[1; 64]; 32];
    }
}
