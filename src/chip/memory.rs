use anyhow::Error;
use anyhow::Result;

const RAM_SIZE: usize = 4096;

#[derive(PartialEq, Eq, Debug)]
pub struct Memory([u8; RAM_SIZE]);

impl Memory {
    pub fn new() -> Self {
        let mut mem = [0; RAM_SIZE];
        Self::load_font(&mut mem);
        Self(mem)
    }

    pub fn get_byte(&self, addr: u16) -> Result<&u8> {
        self.0.get(addr as usize).ok_or_else(|| {
            Error::msg(format!(
                "Attempting to access memory at {:X}, but {:X} is the maximum. Exiting.",
                addr, RAM_SIZE,
            ))
        })
    }

    pub fn get_byte_mut(&mut self, addr: u16) -> Result<&mut u8> {
        self.0.get_mut(addr as usize).ok_or_else(|| {
            Error::msg(format!(
                "Attempting to access memory at {:X}, but {:X} is the maximum. Exiting.",
                addr, RAM_SIZE,
            ))
        })
    }

    pub fn get_word(&self, addr: u16) -> Result<u16> {
        Ok((u16::from(*self.get_byte(addr)?) << 8) | u16::from(*self.get_byte(addr + 1)?))
    }

    pub fn index_of_font_char(byte: u8) -> Result<u16> {
        if byte < 0x10 {
            Ok(u16::from(0x50 + (byte * 5)))
        } else {
            Err(Error::msg(format!(
                "'{}' is not a character within the current font. Exiting.",
                byte
            )))
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<()> {
        if rom.len() >= (self.0.len() - 0x200) {
            return Err(Error::msg(format!(
                "The rom that you are attempting to load is too large ({}). {} bytes is the maximum.",
                rom.len(),
                self.0.len(),
            )));
        }

        for (i, byte) in rom.iter().copied().enumerate() {
            self.0[0x200 + i] = byte;
        }

        Ok(())
    }

    fn load_font(memory: &mut [u8]) {
        const FONT: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, byte) in FONT.iter().copied().enumerate() {
            memory[0x50 + i] = byte;
        }
    }
}
