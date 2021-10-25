use std::ops::Index;
use std::ops::IndexMut;

use anyhow::Error;
use anyhow::Result;

#[derive(PartialEq, Eq, Debug)]
pub struct Registers([u8; 16]);

impl Registers {
    pub fn new() -> Self {
        Self([0; 16])
    }

    pub fn get<T>(&mut self, idx: T) -> Result<&u8>
    where
        T: Into<usize> + std::fmt::UpperHex + Copy,
    {
        self.0
            .get(idx.into())
            .ok_or_else(|| Error::msg(format!("Invalid register: 0x{:X}", idx)))
    }

    pub fn get_mut<T>(&mut self, idx: T) -> Result<&mut u8>
    where
        T: Into<usize> + std::fmt::UpperHex + Copy,
    {
        self.0
            .get_mut(idx.into())
            .ok_or_else(|| Error::msg(format!("Invalid register: 0x{:X}", idx)))
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
