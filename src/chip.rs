mod display;
mod memory;
mod opcode;
mod register;
mod stack;

use display::Display;
use memory::Memory;
use opcode::OpCode;
use register::Registers;
use stack::Stack;

use rand::Rng;

#[derive(PartialEq, Eq, Debug)]
pub struct Chip8 {
    frame_buffer: Display,
    memory: Memory,
    v: Registers,
    stack: Stack,
    index: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            frame_buffer: Display::new(),
            memory: Memory::new(),
            v: Registers::new(),
            stack: Stack::new(),
            index: 0,
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn fetch(&mut self) -> u16 {
        let next_instr = self.memory.get_word(self.program_counter);
        self.program_counter += 2;
        next_instr
    }

    fn decode(op: u16) -> OpCode {
        From::from(op)
    }

    fn execute(&mut self, op: OpCode) -> Result<(), ()> {
        match op {
            OpCode::SysAddr(_addr) => {
                // Unimplemented on most machines, this is purposefully skipped
                // TODO: Make this a possible error
            }
            OpCode::Clear => {
                self.frame_buffer.clear();
            }
            OpCode::Return => {
                let pc = self.stack.pop();
                self.program_counter = pc;
            }
            OpCode::Jump(addr) => {
                self.program_counter = addr;
            }
            OpCode::Call(addr) => {
                let to_return = self.program_counter;
                self.stack.push(to_return);
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
                self.program_counter = addr + self.v[0] as u16;
            }
            OpCode::Random(x, kk) => {
                self.v[x] = self.create_random_byte() & kk;
            }
            OpCode::Draw(x, y, _n) => {
                // TODO: Implement draw function
                let _x = self.v[x] % 64;
                let _y = self.v[y] % 32;
                self.set_vf(false);

                unimplemented!();
            }
            OpCode::SkipKeyPressed(_x) => {
                // TODO: Implement key systems
                unimplemented!();
            }
            OpCode::SkipKeyNotPressed(_x) => {
                // TODO: Implement key systems
                unimplemented!();
            }
            OpCode::LoadDelay(x) => {
                self.v[x] = self.delay_timer;
            }
            OpCode::LoadNextKeyPress(_x) => {
                // TODO: Implement key systems
                unimplemented!();
            }
            OpCode::SetDelayTimer(x) => {
                self.delay_timer = self.v[x];
            }
            OpCode::SetSoundTimer(x) => {
                self.sound_timer = self.v[x];
            }
            OpCode::AddIndexRegister(x) => {
                let res = self.index + self.v[x] as u16;
                let overflow_bit = res & 0x8;
                self.set_vf(overflow_bit == 0x8);
                self.index = res;
            }
            OpCode::IndexAtSprite(x) => {
                debug_assert!(x < 0x10);
                self.index = Memory::index_of_font_char(x);
            }
            OpCode::BinaryCodeConversion(x) => {
                let value = self.v[x];
                self.memory[self.index + 0] = value / 100;
                self.memory[self.index + 1] = (value % 100) / 10;
                self.memory[self.index + 2] = value % 10;
            }
            OpCode::StoreAllRegisters(x) => {
                // TODO: The behavior of self.index should be configurable
                //  In premodern variations, the value of self.index was set to
                //  self.index + x + 1
                for offset in 0..(x + 1) {
                    self.memory[self.index + offset as u16] = self.v[offset];
                }
            }
            OpCode::LoadAllRegisters(x) => {
                // TODO: The behavior of self.index should be configurable
                //  In premodern variations, the value of self.index was set to
                //  self.index + x + 1
                for offset in 0..(x + 1) {
                    self.v[offset] = self.memory[self.index + offset as u16];
                }
            }
        }

        Ok(())
    }

    fn set_vf(&mut self, cond: bool) {
        self.v[0xf] = if cond { 1 } else { 0 };
    }

    fn create_random_byte(&self) -> u8 {
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
        assert!(cpu == Chip8::new());
    }

    #[test]
    fn clear() {
        let mut cpu = Chip8::new();
        cpu.frame_buffer = Display::new_filled();
        assert!(cpu.execute(OpCode::Clear).is_ok());
        assert!(cpu == Chip8::new());
    }

    #[test]
    fn r#return() {
        const ADDR: u16 = 0x420;
        let mut cpu = Chip8::new();
        cpu.stack.push(ADDR);
        assert!(cpu.execute(OpCode::Return).is_ok());
        assert!(cpu.program_counter == ADDR);
    }

    #[test]
    fn jump() {
        const ADDR: u16 = 0x420;
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::Jump(ADDR)).is_ok());
        assert!(cpu.program_counter == ADDR);
    }

    #[test]
    fn call() {
        const CALL_ADDR: u16 = 0x420;
        const PROGRAM_COUNTER: u16 = 0x360;
        let mut cpu = Chip8::new();
        cpu.program_counter = PROGRAM_COUNTER;

        assert!(cpu.execute(OpCode::Call(CALL_ADDR)).is_ok());
        assert!(cpu.program_counter == CALL_ADDR);
        assert!(cpu.stack.pop() == PROGRAM_COUNTER);
    }

    #[test]
    fn skip_equal() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xAB;
        assert!(skip.execute(OpCode::SkipEqual(0xC, 0xAB)).is_ok());
        assert!(skip.program_counter == 2);

        let mut dont_skip = Chip8::new();
        dont_skip.v[0xC] = 0xAD;
        assert!(dont_skip.execute(OpCode::SkipEqual(0xC, 0xAB)).is_ok());
        assert!(dont_skip.program_counter == 0);
    }

    #[test]
    fn skip_not_equal() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xAD;
        assert!(skip.execute(OpCode::SkipNotEqual(0xC, 0xAB)).is_ok());
        assert!(skip.program_counter == 2);

        let mut dont_skip = Chip8::new();
        dont_skip.v[0xC] = 0xAB;
        assert!(dont_skip.execute(OpCode::SkipNotEqual(0xC, 0xAB)).is_ok());
        assert!(dont_skip.program_counter == 0);
    }

    #[test]
    fn skip_equal_register() {
        let mut skip = Chip8::new();
        skip.v[0xC] = 0xA;
        skip.v[0xB] = 0xA;
        assert!(skip.execute(OpCode::SkipEqualRegister(0xC, 0xB)).is_ok());
        assert!(skip.program_counter == 2);

        let mut dont_skip = Chip8::new();
        dont_skip.v[0xC] = 0xC;
        dont_skip.v[0xB] = 0xB;
        assert!(dont_skip
            .execute(OpCode::SkipEqualRegister(0xC, 0xB))
            .is_ok());
        assert!(dont_skip.program_counter == 0);
    }

    #[test]
    fn load() {
        let mut cpu = Chip8::new();
        assert!(cpu.execute(OpCode::Load(0xC, 0xAB)).is_ok());
        assert!(cpu.v[0xC] == 0xAB);
    }

    #[test]
    fn add() {
        let mut cpu = Chip8::new();
        cpu.v[0xC] = 0xA;
        assert!(cpu.execute(OpCode::Add(0xC, 0xFC)).is_ok());
        assert!(cpu.v[0xC] == 0x06 && cpu.v[0xF] == 0);
    }

    #[test]
    fn load_register() {
        let mut cpu = Chip8::new();
        cpu.v[0xC] = 0xFF;
        assert!(cpu.execute(OpCode::LoadRegister(0x0, 0xC)).is_ok());
        assert!(cpu.v[0xC] == cpu.v[0x0] && cpu.v[0x0] == 0xFF);
    }

    #[test]
    fn or_register() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0x0F;
        cpu.v[0x1] = 0xF0;
        assert!(cpu.execute(OpCode::OrRegister(0x0, 0x1)).is_ok());
        assert!(cpu.v[0x0] == 0xFF);
        assert!(cpu.v[0x1] == 0xF0);
    }

    #[test]
    fn and_register() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0x2F;
        cpu.v[0x1] = 0xF2;
        assert!(cpu.execute(OpCode::AndRegister(0x0, 0x1)).is_ok());
        assert!(cpu.v[0x0] == 0x22);
        assert!(cpu.v[0x1] == 0xF2);
    }

    #[test]
    fn xor_register() {
        let mut cpu = Chip8::new();
        cpu.v[0x0] = 0b1010_1010;
        cpu.v[0x1] = 0b0101_0101;
        assert!(cpu.execute(OpCode::XorRegister(0x0, 0x1)).is_ok());
        assert!(cpu.v[0x0] == 0b1111_1111);
        assert!(cpu.v[0x1] == 0b0101_0101);
    }

    #[test]
    fn add_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0xAF;
        set_flag.v[0x1] = 0xBC;
        assert!(set_flag.execute(OpCode::AddRegister(0x0, 0x1)).is_ok());
        assert!(set_flag.v[0x0] == 0x6B);
        assert!(set_flag.v[0x1] == 0xBC);
        assert!(set_flag.v[0xF] == 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x0F;
        no_flag.v[0x1] = 0xE1;
        assert!(no_flag.execute(OpCode::AddRegister(0x0, 0x1)).is_ok());
        assert!(no_flag.v[0x0] == 0xF0);
        assert!(no_flag.v[0x1] == 0xE1);
        assert!(no_flag.v[0xF] == 0x00);
    }

    #[test]
    fn sub_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0xFF;
        set_flag.v[0x1] = 0x01;
        assert!(set_flag.execute(OpCode::SubRegister(0x0, 0x1)).is_ok());
        assert!(set_flag.v[0x0] == 0xFE);
        assert!(set_flag.v[0x1] == 0x01);
        assert!(set_flag.v[0xF] == 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x01;
        no_flag.v[0x1] = 0x02;
        assert!(no_flag.execute(OpCode::SubRegister(0x0, 0x1)).is_ok());
        assert!(no_flag.v[0x0] == 0xFF);
        assert!(no_flag.v[0x1] == 0x02);
        assert!(no_flag.v[0xF] == 0x00);
    }

    #[test]
    fn shift_right_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0x11;
        assert!(set_flag
            .execute(OpCode::ShiftRightRegister(0x0, 0x0))
            .is_ok());
        assert!(set_flag.v[0x0] == 0x08);
        assert!(set_flag.v[0xF] == 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x10;
        assert!(no_flag
            .execute(OpCode::ShiftRightRegister(0x0, 0x0))
            .is_ok());
        assert!(no_flag.v[0x0] == 0x08);
        assert!(no_flag.v[0xF] == 0x00);
    }

    #[test]
    fn sub_reverse_register() {
        let mut set_flag = Chip8::new();
        set_flag.v[0x0] = 0x01;
        set_flag.v[0x1] = 0x02;
        assert!(set_flag.execute(OpCode::SubReverseRegister(0x0, 0x1)).is_ok());
        assert!(set_flag.v[0x0] == 0x01);
        assert!(set_flag.v[0x1] == 0x02);
        assert!(set_flag.v[0xF] == 0x01);

        let mut no_flag = Chip8::new();
        no_flag.v[0x0] = 0x0B;
        no_flag.v[0x1] = 0xAF;
        assert!(no_flag.execute(OpCode::SubReverseRegister(0x0, 0x1)).is_ok());
        assert!(no_flag.v[0x0] == 0xA4);
        assert!(no_flag.v[0x1] == 0xAF);
        assert!(no_flag.v[0xF] == 0x00);

    }

    #[test]
    fn shift_left_register() {
        unimplemented!()
    }

    #[test]
    fn skip_not_equal_register() {
        unimplemented!()
    }

    #[test]
    fn set_index_register() {
        unimplemented!()
    }

    #[test]
    fn jump_with_offset() {
        unimplemented!()
    }

    #[test]
    fn random() {
        unimplemented!()
    }

    #[test]
    fn draw() {
        unimplemented!()
    }

    #[test]
    fn skip_key_pressed() {
        unimplemented!()
    }

    #[test]
    fn skip_key_not_pressed() {
        unimplemented!()
    }

    #[test]
    fn load_delay() {
        unimplemented!()
    }

    #[test]
    fn load_next_key_press() {
        unimplemented!()
    }

    #[test]
    fn set_delay_timer() {
        unimplemented!()
    }

    #[test]
    fn set_sound_timer() {
        unimplemented!()
    }

    #[test]
    fn add_index_register() {
        unimplemented!()
    }

    #[test]
    fn index_at_sprite() {
        unimplemented!()
    }

    #[test]
    fn binary_code_conversion() {
        unimplemented!()
    }

    #[test]
    fn store_all_registers() {
        unimplemented!()
    }

    #[test]
    fn load_all_registers() {
        unimplemented!()
    }
}
