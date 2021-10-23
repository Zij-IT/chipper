use anyhow::Result;

#[derive(PartialEq, Eq, Debug)]
pub struct Memory([u8; 4096]);

impl Memory {
    pub fn new() -> Self {
        let mut mem = [0; 4096];
        Self::load_font(&mut mem);
        Self(mem)
    }

    pub fn get_byte(&self, addr: u16) -> Result<u8> {
        self.0
            .get(addr as usize)
            .copied()
            .ok_or_else(|| MemoryError::InvalidMemoryAddress(addr).into())
    }

    pub fn get_byte_mut(&mut self, addr: u16) -> Result<&mut u8> {
        self.0
            .get_mut(addr as usize)
            .ok_or_else(|| MemoryError::InvalidMemoryAddress(addr).into())
    }

    pub fn get_word(&self, addr: u16) -> Result<u16> {
        Ok((u16::from(self.get_byte(addr)?) << 8) | u16::from(self.get_byte(addr + 1)?))
    }

    pub fn index_of_font_char(byte: u8) -> Result<u16> {
        if byte < 0x10 {
            Ok(u16::from(0x50 + (byte * 5)))
        } else {
            Err(MemoryError::InvalidChar(byte).into())
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<()> {
        if rom.len() >= (self.0.len() - 0x200) {
            return Err(MemoryError::RomTooLarge(rom.len()).into());
        }

        for (i, byte) in rom.iter().copied().enumerate() {
            self.0[0x200 + i] = byte;
        }

        Ok(())
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

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    RomTooLarge(usize),
    InvalidMemoryAddress(u16),
    InvalidChar(u8),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::RomTooLarge(rom_len) => format!("Rom is too large: {}", rom_len),
            Self::InvalidMemoryAddress(addr) => format!("Invalid memory address: {}", addr),
            Self::InvalidChar(byte) => format!("Attempting to get index of invalid char: {}", byte),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for MemoryError {}

