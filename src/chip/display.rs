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

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut coord_x = x as usize % CHIP8_WIDTH;
        let coord_y = y as usize % CHIP8_HEIGHT;
        let mut erased = false;
        let mut byte = byte;

        for _ in 0..8 {
            if coord_x >= CHIP8_WIDTH {
                break;
            }

            let bit = (byte & 0x80) >> 7;
            let prev_bit = self.buffer[coord_y][coord_x];

            self.buffer[coord_y][coord_x] ^= bit;

            erased = erased || (prev_bit == 1 && self.buffer[coord_y][coord_x] == 0);
            coord_x += 1;
            byte <<= 1;
        }

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
