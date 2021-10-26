use anyhow::Error;
use anyhow::Result;

#[derive(PartialEq, Eq, Debug)]
pub struct Stack {
    stack: [u16; 16],
    stack_ptr: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; 16],
            stack_ptr: 0,
        }
    }

    pub fn pop(&mut self) -> Result<u16> {
        if self.stack_ptr == 0 {
            Err(Error::msg("The stack has underflowed."))
        } else {
            self.stack_ptr -= 1;
            Ok(self.stack[self.stack_ptr])
        }
    }

    pub fn push(&mut self, byte: u16) -> Result<()> {
        if self.stack_ptr >= 16 {
            Err(Error::msg("The stack has overflowed."))
        } else {
            self.stack[self.stack_ptr] = byte;
            self.stack_ptr += 1;
            Ok(())
        }
    }
}
