use std::ops::Index;
use std::ops::IndexMut;

pub struct Memory([u8; 4096]);

impl Memory {

}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, idx: u16) -> &Self::Output {
        debug_assert!(idx < 4096);
        &self.0[idx as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, idx: u16) -> &mut Self::Output {
        debug_assert!(idx < 4096);
        &mut self.0[idx as usize]
    }
}
