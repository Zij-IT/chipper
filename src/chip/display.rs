const SCALE: usize = 20;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    display: [[u8; WIDTH]; HEIGHT],
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video = sdl_context.video().unwrap();
        let window = video
            .window("title", (SCALE * WIDTH) as u32, (SCALE * HEIGHT) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self {
            display: [[0; 64]; 32],
            canvas,
        }
    }

    pub fn clear(&mut self) {
        self.display = [[0; 64]; 32];
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
            let prev_bit = self.display[coord_y][coord_x];

            self.display[coord_y][coord_x] ^= bit;

            erased = erased || (prev_bit == 1 && self.display[coord_y][coord_x] == 0);
            coord_x += 1;
            byte <<= 1;
        }

        erased
    }

    pub fn draw_on_canvas(&mut self) {
        for (y, row) in self.display.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x * SCALE) as u32;
                let y = (y * SCALE) as u32;

                let color = if col == 0 {
                    sdl2::pixels::Color::RGB(0, 0, 0)
                } else {
                    sdl2::pixels::Color::RGB(0xFF, 0xFF, 0xFF)
                };

                self.canvas.set_draw_color(color);
                let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(
                    x as i32,
                    y as i32,
                    SCALE as u32,
                    SCALE as u32,
                ));
            }
        }

        self.canvas.present();
    }

    pub fn frame_buffer(&self) -> &[[u8; 64]; 32] {
        &self.display
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    #[cfg(test)]
    pub fn new_filled(sdl_context: &sdl2::Sdl) -> Self {
        let video = sdl_context.video().unwrap();
        let window = video
            .window("title", (SCALE * WIDTH) as u32, (SCALE * HEIGHT) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self {
            display: [[0xFF; 64]; 32],
            canvas,
        }
    }
}
