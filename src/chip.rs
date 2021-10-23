mod display;
mod keyboard;
mod memory;
mod opcode;
mod register;
mod stack;

pub use display::FrameBuffer;

use display::Display;
use keyboard::Keyboard;
use memory::Memory;
use opcode::OpCode;
use register::Registers;
use stack::Stack;

use anyhow::Result;
use rand::Rng;
use std::convert::TryFrom;

#[derive(PartialEq, Eq, Debug)]
pub struct Chip8 {
    display: Display,
    memory: Memory,
    v: Registers,
    stack: Stack,
    input: Keyboard,
    index: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            memory: Memory::new(),
            v: Registers::new(),
            stack: Stack::new(),
            input: Keyboard::new(),
            index: 0,
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<()> {
        self.memory.load_rom(rom)
    }

    pub fn get_frame_buffer(&mut self) -> &FrameBuffer {
        self.display.get_frame_buffer()
    }

    pub fn cycle(&mut self, keys: [bool; 16]) -> Result<()> {
        self.input.set_keys(keys);
        let word = self.fetch()?;
        let op = Self::decode(word)?;
        self.execute(op)
    }

    fn fetch(&mut self) -> Result<u16> {
        let next_instr = self.memory.get_word(self.program_counter)?;
        self.program_counter += 2;
        Ok(next_instr)
    }

    fn decode(op: u16) -> Result<OpCode> {
        TryFrom::try_from(op)
    }

    fn execute(&mut self, op: OpCode) -> Result<()> {
        match op {
            OpCode::SysAddr(_addr) => {
                // Unimplemented on most machines, this is purposefully skipped
                // TODO: Make this a possible error
            }
            OpCode::Clear => {
                self.display.clear();
            }
            OpCode::Return => {
                let pc = self.stack.pop()?;
                self.program_counter = pc;
            }
            OpCode::Jump(addr) => {
                self.program_counter = addr;
            }
            OpCode::Call(addr) => {
                let to_return = self.program_counter;
                self.stack.push(to_return)?;
                self.program_counter = addr;
            }
            OpCode::SkipEqual(x, kk) => {
                if self.v[x] == kk {
                    self.program_counter += 2;
                }
            }
            OpCode::SkipNotEqual(x, kk) => {
                if self.v[x] != kk {
                    self.program_counter += 2;
                }
            }
            OpCode::SkipEqualRegister(x, y) => {
                if self.v[x] == self.v[y] {
                    self.program_counter += 2;
                }
            }
            OpCode::Load(x, kk) => {
                self.v[x] = kk;
            }
            OpCode::Add(x, kk) => {
                self.v[x] = self.v[x].wrapping_add(kk);
            }
            OpCode::LoadRegister(x, y) => {
                self.v[x] = self.v[y];
            }
            OpCode::OrRegister(x, y) => {
                self.v[x] |= self.v[y];
            }
            OpCode::AndRegister(x, y) => {
                self.v[x] &= self.v[y];
            }
            OpCode::XorRegister(x, y) => {
                self.v[x] ^= self.v[y];
            }
            OpCode::AddRegister(x, y) => {
                let (res, over) = self.v[x].overflowing_add(self.v[y]);
                self.set_vf(over);
                self.v[x] = res;
            }
            OpCode::SubRegister(x, y) => {
                let (res, under) = self.v[x].overflowing_sub(self.v[y]);
                self.set_vf(!under); // VF is set if underflow did not occur
                self.v[x] = res;
            }
            OpCode::ShiftRightRegister(x, _y) => {
                // TODO:
                //  This instruction has a step that was changed in CHIP-48 and SUPER-CHIP
                //  This should be configurable
                let shifted_bit = self.v[x] & 0x1;
                self.set_vf(shifted_bit == 0x1);
                self.v[x] >>= 1;
            }
            OpCode::SubReverseRegister(x, y) => {
                let (res, under) = self.v[y].overflowing_sub(self.v[x]);
                self.set_vf(!under);
                self.v[x] = res;
            }
            OpCode::ShiftLeftRegister(x, _y) => {
                let bit = self.v[x] & 0x80;
                self.set_vf(bit == 0x80);
                self.v[x] <<= 1;
            }
            OpCode::SkipNotEqualRegister(x, y) => {
                if self.v[x] != self.v[y] {
                    self.program_counter += 2;
                }
            }
            OpCode::SetIndexRegister(addr) => {
                self.index = addr;
            }
            OpCode::JumpWithOffset(addr) => {
                // TODO:
                //  This instruction was changed in CHIP-48 and SUPER-CHIP
                //  This should be configurable
                self.program_counter = addr + u16::from(self.v[0]);
            }
            OpCode::Random(x, kk) => {
                self.v[x] = Self::generate_random_byte() & kk;
            }
            OpCode::Draw(x, y, n) => {
                let x = self.v[x] % 64;
                let y = self.v[y] % 32;
                let mut pixel_changed = false;

                for sy in 0..n {
                    let byte = self.memory.get_byte(self.index + u16::from(sy))?;
                    pixel_changed = pixel_changed || self.display.draw_byte(byte, x, y + sy);
                }

                self.set_vf(pixel_changed);
            }
            OpCode::SkipKeyPressed(x) => {
                if self.input.is_key_pressed(x) {
                    self.program_counter += 2;
                }
            }
            OpCode::SkipKeyNotPressed(x) => {
                if !self.input.is_key_pressed(x) {
                    self.program_counter += 2;
                }
            }
            OpCode::LoadDelay(x) => {
                self.v[x] = self.delay_timer;
            }
            OpCode::LoadNextKeyPress(x) => {
                if let Some(key) = self.input.get_next_key() {
                    self.v[x] = key;
                } else {
                    self.program_counter -= 2;
                }
            }
            OpCode::SetDelayTimer(x) => {
                self.delay_timer = self.v[x];
            }
            OpCode::SetSoundTimer(x) => {
                self.sound_timer = self.v[x];
            }
            OpCode::AddIndexRegister(x) => {
                let res = self.index + u16::from(self.v[x]);
                let overflow_bit = res & 0x8;
                self.set_vf(overflow_bit == 0x8);
                self.index = res;
            }
            OpCode::IndexAtSprite(x) => {
                self.index = Memory::index_of_font_char(x)?;
            }
            OpCode::BinaryCodeConversion(x) => {
                let value = self.v[x];
                *self.memory.get_byte_mut(self.index)? = value / 100;
                *self.memory.get_byte_mut(self.index + 1)? = (value % 100) / 10;
                *self.memory.get_byte_mut(self.index + 2)? = value % 10;
            }
            OpCode::StoreAllRegisters(x) => {
                // TODO: The behavior of self.index should be configurable
                //  In premodern variations, the value of self.index was set to
                //  self.index + x + 1
                for offset in 0..=x {
                    *self.memory.get_byte_mut(self.index + u16::from(offset))? = self.v[offset];
                }
            }
            OpCode::LoadAllRegisters(x) => {
                // TODO: The behavior of self.index should be configurable
                //  In premodern variations, the value of self.index was set to
                //  self.index + x + 1
                for offset in 0..=x {
                    self.v[offset] = self.memory.get_byte(self.index + u16::from(offset))?;
                }
            }
        }

        Ok(())
    }

    fn set_vf(&mut self, cond: bool) {
        self.v[0xf] = if cond { 1 } else { 0 };
    }

    fn generate_random_byte() -> u8 {
        rand::thread_rng().gen::<u8>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sysaddr() {
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::SysAddr(0x0000)).is_ok());
        assert_eq!(cpu, Chip8::new());
    }

    #[test]
    fn clear() {
        let mut cpu = Chip8::new();
        cpu.display.fill_buffer();
        assert!(cpu.execute(OpCode::Clear).is_ok());
        assert_eq!(cpu, Chip8::new());
    }

    #[test]
    fn r#return() {
        const ADDR: u16 = 0x420;
        let mut cpu = Chip8::new();
        cpu.stack.push(ADDR).unwrap();
        assert!(cpu.execute(OpCode::Return).is_ok());
        assert_eq!(cpu.program_counter, ADDR);
    }

    #[test]
    fn jump() {
        const ADDR: u16 = 0x420;
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::Jump(ADDR)).is_ok());
        assert_eq!(cpu.program_counter, ADDR);
    }

    #[test]
    fn call() {
        const CALL_ADDR: u16 = 0x420;
        const PROGRAM_COUNTER: u16 = 0x360;
        let mut cpu = Chip8::new();
        cpu.program_counter = PROGRAM_COUNTER;

        assert!(cpu.execute(OpCode::Call(CALL_ADDR)).is_ok());
        assert_eq!(cpu.program_counter, CALL_ADDR);
        assert_eq!(cpu.stack.pop().unwrap(), PROGRAM_COUNTER);
    }

    #[test]
    fn skip_equal() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xAB;
        assert!(skip.execute(OpCode::SkipEqual(0xC, 0xAB)).is_ok());
        assert_eq!(skip.program_counter, 0x200 + 2);

        let mut dont_skip = Chip8::new();
        dont_skip.v[0xC] = 0xAD;
        assert!(dont_skip.execute(OpCode::SkipEqual(0xC, 0xAB)).is_ok());
        assert_eq!(dont_skip.program_counter, 0x200 + 0);
    }

    #[test]
    fn skip_not_equal() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xAD;
        assert!(skip.execute(OpCode::SkipNotEqual(0xC, 0xAB)).is_ok());
        assert_eq!(skip.program_counter, 0x200 + 2);

        let mut dont_skip = Chip8::new();
        dont_skip.v[0xC] = 0xAB;
        assert!(dont_skip.execute(OpCode::SkipNotEqual(0xC, 0xAB)).is_ok());
        assert_eq!(dont_skip.program_counter, 0x200 + 0);
    }

    #[test]
    fn skip_equal_register() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xA;
        skip.v[0xB] = 0xA;
        assert!(skip.execute(OpCode::SkipEqualRegister(0xC, 0xB)).is_ok());
        assert_eq!(skip.program_counter, 0x200 + 2);

        let mut dont_skip = Chip8::new();
        dont_skip.v[0xC] = 0xC;
        dont_skip.v[0xB] = 0xB;
        assert!(dont_skip
            .execute(OpCode::SkipEqualRegister(0xC, 0xB))
            .is_ok());
        assert_eq!(dont_skip.program_counter, 0x200 + 0);
    }

    #[test]
    fn load() {
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::Load(0xC, 0xAB)).is_ok());
        assert_eq!(cpu.v[0xC], 0xAB);
    }

    #[test]
    fn add() {
        let mut cpu = Chip8::new();
        cpu.v[0xC] = 0xA;
        assert!(cpu.execute(OpCode::Add(0xC, 0xFC)).is_ok());
        assert_eq!(cpu.v[0xC], 0x06);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn load_register() {
        let mut cpu = Chip8::new();
        cpu.v[0xC] = 0xFF;
        assert!(cpu.execute(OpCode::LoadRegister(0x0, 0xC)).is_ok());
        assert_eq!(cpu.v[0x0], cpu.v[0xC]);
        assert_eq!(cpu.v[0x0], 0xFF);
    }

    #[test]
    fn or_register() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0x0F;
        cpu.v[0x1] = 0xF0;
        assert!(cpu.execute(OpCode::OrRegister(0x0, 0x1)).is_ok());
        assert_eq!(cpu.v[0x0], 0xFF);
        assert_eq!(cpu.v[0x1], 0xF0);
    }

    #[test]
    fn and_register() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0x2F;
        cpu.v[0x1] = 0xF2;
        assert!(cpu.execute(OpCode::AndRegister(0x0, 0x1)).is_ok());
        assert_eq!(cpu.v[0x0], 0x22);
        assert_eq!(cpu.v[0x1], 0xF2);
    }

    #[test]
    fn xor_register() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0b1010_1010;
        cpu.v[0x1] = 0b0101_0101;
        assert!(cpu.execute(OpCode::XorRegister(0x0, 0x1)).is_ok());
        assert_eq!(cpu.v[0x0], 0b1111_1111);
        assert_eq!(cpu.v[0x1], 0b0101_0101);
    }

    #[test]
    fn add_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0xAF;
        set_flag.v[0x1] = 0xBC;
        assert!(set_flag.execute(OpCode::AddRegister(0x0, 0x1)).is_ok());
        assert_eq!(set_flag.v[0x0], 0x6B);
        assert_eq!(set_flag.v[0x1], 0xBC);
        assert_eq!(set_flag.v[0xF], 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x0F;
        no_flag.v[0x1] = 0xE1;
        assert!(no_flag.execute(OpCode::AddRegister(0x0, 0x1)).is_ok());
        assert_eq!(no_flag.v[0x0], 0xF0);
        assert_eq!(no_flag.v[0x1], 0xE1);
        assert_eq!(no_flag.v[0xF], 0x00);
    }

    #[test]
    fn sub_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0xFF;
        set_flag.v[0x1] = 0x01;
        assert!(set_flag.execute(OpCode::SubRegister(0x0, 0x1)).is_ok());
        assert_eq!(set_flag.v[0x0], 0xFE);
        assert_eq!(set_flag.v[0x1], 0x01);
        assert_eq!(set_flag.v[0xF], 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x01;
        no_flag.v[0x1] = 0x02;
        assert!(no_flag.execute(OpCode::SubRegister(0x0, 0x1)).is_ok());
        assert_eq!(no_flag.v[0x0], 0xFF);
        assert_eq!(no_flag.v[0x1], 0x02);
        assert_eq!(no_flag.v[0xF], 0x00);
    }

    #[test]
    fn shift_right_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0x11;
        assert!(set_flag
            .execute(OpCode::ShiftRightRegister(0x0, 0x0))
            .is_ok());
        assert_eq!(set_flag.v[0x0], 0x08);
        assert_eq!(set_flag.v[0xF], 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x10;
        assert!(no_flag
            .execute(OpCode::ShiftRightRegister(0x0, 0x0))
            .is_ok());
        assert_eq!(no_flag.v[0x0], 0x08);
        assert_eq!(no_flag.v[0xF], 0x00);
    }

    #[test]
    fn sub_reverse_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0x01;
        set_flag.v[0x1] = 0x02;
        assert!(set_flag
            .execute(OpCode::SubReverseRegister(0x0, 0x1))
            .is_ok());
        assert_eq!(set_flag.v[0x0], 0x01);
        assert_eq!(set_flag.v[0x1], 0x02);
        assert_eq!(set_flag.v[0xF], 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x0B;
        no_flag.v[0x1] = 0x0A;
        assert!(no_flag
            .execute(OpCode::SubReverseRegister(0x0, 0x1))
            .is_ok());
        assert_eq!(no_flag.v[0x0], 0xFF);
        assert_eq!(no_flag.v[0x1], 0x0A);
        assert_eq!(no_flag.v[0xF], 0x00);
    }

    #[test]
    fn shift_left_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0x88;
        assert!(set_flag
            .execute(OpCode::ShiftLeftRegister(0x0, 0x0))
            .is_ok());
        assert_eq!(set_flag.v[0x0], 0x10);
        assert_eq!(set_flag.v[0xF], 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x01;
        assert!(no_flag.execute(OpCode::ShiftLeftRegister(0x0, 0x0)).is_ok());
        assert_eq!(no_flag.v[0x0], 0x02);
        assert_eq!(no_flag.v[0xF], 0x00);
    }

    #[test]
    fn skip_not_equal_register() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xC;
        skip.v[0xB] = 0xB;
        assert!(skip.execute(OpCode::SkipEqualRegister(0xC, 0xB)).is_ok());
        assert_eq!(skip.program_counter, 0x200 + 0);

        let mut not_skip = Chip8::new();
        not_skip.v[0xC] = 0xA;
        not_skip.v[0xB] = 0xA;
        assert!(not_skip
            .execute(OpCode::SkipEqualRegister(0xC, 0xB))
            .is_ok());
        assert_eq!(not_skip.program_counter, 0x200 + 2);
    }

    #[test]
    fn set_index_register() {
        const ADDR: u16 = 0x500;

        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::SetIndexRegister(ADDR)).is_ok());
        assert_eq!(cpu.index, 0x500);
    }

    #[test]
    fn jump_with_offset() {
        const ADDR: u16 = 0x500;

        let mut cpu = Chip8::new();
        cpu.v[0] = 0x20;
        assert!(cpu.execute(OpCode::JumpWithOffset(ADDR)).is_ok());
        assert_eq!(cpu.program_counter, 0x520);
    }

    #[test]
    fn random() {
        // There is not really a good test for this, as it just generates a random byte
        // so, I will just make sure that the CPU has a function for generating a random byte.
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::Random(0x0, 0xFF)).is_ok());
        let _byte = Chip8::generate_random_byte();
    }

    #[test]
    fn skip_key_pressed() {
        let mut cpu = Chip8::new();
        let mut keys = [false; 16];
        keys[0xA] = true;
        cpu.input.set_keys(keys);

        assert!(cpu.execute(OpCode::SkipKeyPressed(0xA)).is_ok());
        assert_eq!(cpu.program_counter, 0x200 + 2);
    }

    #[test]
    fn skip_key_not_pressed() {
        let mut cpu = Chip8::new();
        let mut keys = [false; 16];
        keys[0xA] = true;
        cpu.input.set_keys(keys);

        assert!(cpu.execute(OpCode::SkipKeyNotPressed(0xA)).is_ok());
        assert_eq!(cpu.program_counter, 0x200 + 0);
    }

    #[test]
    fn load_delay() {
        let mut cpu = Chip8::new();
        cpu.delay_timer = 0x20;
        assert!(cpu.execute(OpCode::LoadDelay(0x0)).is_ok());
        assert_eq!(cpu.v[0x0], cpu.delay_timer);
    }

    #[test]
    fn load_next_key_press() {
        let mut cpu = Chip8::new();
        let mut keys = [false; 16];
        keys[0xA] = true;
        cpu.input.set_keys(keys);

        assert!(cpu.execute(OpCode::LoadNextKeyPress(0x0)).is_ok());
        assert_eq!(cpu.v[0x0], 0xA);
    }

    #[test]
    fn set_delay_timer() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0x20;
        cpu.delay_timer = 0x1;
        assert!(cpu.execute(OpCode::SetDelayTimer(0x0)).is_ok());
        assert_eq!(cpu.v[0x0], cpu.delay_timer);
    }

    #[test]
    fn set_sound_timer() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0x20;
        cpu.sound_timer = 0x1;
        assert!(cpu.execute(OpCode::SetSoundTimer(0x0)).is_ok());
        assert_eq!(cpu.v[0x0], cpu.sound_timer);
    }

    #[test]
    fn add_index_register() {
        let mut cpu = Chip8::new();
        cpu.index = 0x01;
        cpu.v[0x0] = 0x20;
        assert!(cpu.execute(OpCode::AddIndexRegister(0x0)).is_ok());
        assert_eq!(cpu.index, 0x21);
        assert_eq!(cpu.v[0xF], 0x0);
    }

    #[test]
    fn index_at_sprite() {
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::IndexAtSprite(0x1)).is_ok());
        assert_eq!(cpu.index, Memory::index_of_font_char(0x1).unwrap());
    }

    #[test]
    fn binary_code_conversion() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 152;
        cpu.index = 0x100;
        assert!(cpu.execute(OpCode::BinaryCodeConversion(0x0)).is_ok());
        assert_eq!(cpu.v[0x0], 152);
        assert_eq!(cpu.index, 0x100);
        assert_eq!(cpu.memory.get_byte(0x100).unwrap(), 0x1);
        assert_eq!(cpu.memory.get_byte(0x101).unwrap(), 0x5);
        assert_eq!(cpu.memory.get_byte(0x102).unwrap(), 0x2);
    }

    #[test]
    fn store_all_registers() {
        let mut cpu = Chip8::new();
        cpu.index = 0x100;
        cpu.v[0x0] = 0xB;
        cpu.v[0x1] = 0xE;
        cpu.v[0x2] = 0xE;
        cpu.v[0x3] = 0xF;

        assert!(cpu.execute(OpCode::StoreAllRegisters(0x3)).is_ok());
        assert_eq!(cpu.memory.get_byte(0x100).unwrap(), 0xB);
        assert_eq!(cpu.memory.get_byte(0x101).unwrap(), 0xE);
        assert_eq!(cpu.memory.get_byte(0x102).unwrap(), 0xE);
        assert_eq!(cpu.memory.get_byte(0x103).unwrap(), 0xF);
    }

    #[test]
    fn load_all_registers() {
        let mut cpu = Chip8::new();
        cpu.index = 0x100;
        *cpu.memory.get_byte_mut(0x100).unwrap() = 0xB;
        *cpu.memory.get_byte_mut(0x101).unwrap() = 0xE;
        *cpu.memory.get_byte_mut(0x102).unwrap() = 0xE;
        *cpu.memory.get_byte_mut(0x103).unwrap() = 0xF;

        assert!(cpu.execute(OpCode::LoadAllRegisters(0x3)).is_ok());
        assert_eq!(cpu.v[0x0], 0xB);
        assert_eq!(cpu.v[0x1], 0xE);
        assert_eq!(cpu.v[0x2], 0xE);
        assert_eq!(cpu.v[0x3], 0xF);
    }
}
