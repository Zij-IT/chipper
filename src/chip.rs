mod register;
mod display;
mod memory;
mod opcode;
mod stack;

use register::Registers;
use display::Display;
use memory::Memory;
pub use opcode::OpCode;
use stack::Stack;

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
    pub fn translate_opcode(op: u16) -> OpCode {
        todo!()
    }
}
