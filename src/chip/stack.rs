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
            Err(StackError::Underflow.into())
        } else {
            self.stack_ptr -= 1;
            Ok(self.stack[self.stack_ptr])
        }
    }

    pub fn push(&mut self, byte: u16) -> Result<()> {
        if self.stack_ptr >= 16 {
            Err(StackError::Overflow.into())
        } else {
            self.stack[self.stack_ptr] = byte;
            self.stack_ptr += 1;
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackError {
    Overflow,
    Underflow,
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Overflow => "overflowed",
            Self::Underflow => "underflowed",
        };

        write!(f, "Stack has {}", msg)
    }
}

impl std::error::Error for StackError {}
