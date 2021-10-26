use std::ops::Index;
use std::ops::IndexMut;

#[derive(PartialEq, Eq, Debug)]
pub struct Registers([u8; 16]);

impl Registers {
    pub fn new() -> Self {
        Self([0; 16])
    }
}

impl Index<u8> for Registers {
    type Output = u8;

    fn index(&self, idx: u8) -> &Self::Output {
        debug_assert!(idx < 16);
        &self.0[idx as usize]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, idx: u8) -> &mut Self::Output {
        debug_assert!(idx < 16);
        &mut self.0[idx as usize]
    }
}
