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
    fn translate_opcode(op: u16) -> OpCode {
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
                if self.v[x] != self.v[y] {
                    self.program_counter += 2;
                }
            }
            OpCode::Load(x, kk) => {
                self.v[x] = kk;
            }
            OpCode::Add(x, kk) => {
                self.v[x] += kk;
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
                self.set_vf(!under);
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
            OpCode::Draw(x, y, n) => {
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
            OpCode::IndexAtSprite(_x) => {
                // TODO: Implement fonts
                unimplemented!();
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
