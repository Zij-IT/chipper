use std::ops::Index;
use std::ops::IndexMut;

pub struct Memory([u8; 4096]);

impl Memory {
    pub fn new() -> Self {
        let mut mem = [0; 4096];
        Self::load_font(&mut mem);
        Self(mem)
    }

    pub fn index_of_font_char(byte: u8) -> u16 {
        (0x50 + (byte * 5)) as u16
    }

    fn load_font(memory: &mut [u8]) {
        let zero = [0xF0, 0x90, 0x90, 0x90, 0xF0]; // 0
        let one = [0x20, 0x60, 0x20, 0x20, 0x70]; // 1
        let two = [0xF0, 0x10, 0xF0, 0x80, 0xF0]; // 2
        let three = [0xF0, 0x10, 0xF0, 0x10, 0xF0]; // 3
        let four = [0x90, 0x90, 0xF0, 0x10, 0x10]; // 4
        let five = [0xF0, 0x80, 0xF0, 0x10, 0xF0]; // 5
        let six = [0xF0, 0x80, 0xF0, 0x90, 0xF0]; // 6
        let seven = [0xF0, 0x10, 0x20, 0x40, 0x40]; // 7
        let eight = [0xF0, 0x90, 0xF0, 0x90, 0xF0]; // 8
        let nine = [0xF0, 0x90, 0xF0, 0x10, 0xF0]; // 9
        let a = [0xF0, 0x90, 0xF0, 0x90, 0x90]; // A
        let b = [0xE0, 0x90, 0xE0, 0x90, 0xE0]; // B
        let c = [0xF0, 0x80, 0x80, 0x80, 0xF0]; // C
        let d = [0xE0, 0x90, 0x90, 0x90, 0xE0]; // D
        let e = [0xF0, 0x80, 0xF0, 0x80, 0xF0]; // E
        let f = [0xF0, 0x80, 0xF0, 0x80, 0x80]; // F
        let font = [
            zero, one, two, three, four, five, six, seven, eight, nine, a, b, c, d, e, f,
        ];

        for (i, byte) in font.iter().flatten().copied().enumerate() {
            memory[0x50 + i] = byte;
        }
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, idx: u16) -> &Self::Output {
        debug_assert!(idx < 4096);
        &self.0[idx as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, idx: u16) -> &mut Self::Output {
        debug_assert!(idx < 4096);
        &mut self.0[idx as usize]
    }
}
