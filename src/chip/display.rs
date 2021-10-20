#[derive(PartialEq, Eq, Debug)]
pub struct Display([[u8; 64]; 32]);

impl Display {
    pub fn new() -> Self {
        Self([[0; 64]; 32])
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut coord_x = x as usize % self.width();
        let coord_y = y as usize % self.height();
        let mut erased = false;
        let mut byte = byte;

        for _ in 0..8 {
            if coord_x >= self.width() {
                break;
            }

            let bit = (byte & 0x80) >> 7;
            let prev_bit = self.0[coord_y][coord_x];

            self.0[coord_y][coord_x] ^= bit;

            erased = erased || (prev_bit == 1 && self.0[coord_y][coord_x] == 0);
            coord_x += 1;
            byte <<= 1;
        }

        erased
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    #[cfg(test)]
    pub fn new_filled() -> Self {
        Self([[0; 64]; 32])
    }
}
