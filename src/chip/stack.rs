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

    pub fn pop(&mut self) -> u16 {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr]
    }

    pub fn push(&mut self, byte: u16) {
        self.stack[self.stack_ptr] = byte;
        self.stack_ptr += 1;
    }
}
