#[derive(PartialEq, Eq, Debug)]
pub struct Display([[u8; 64]; 32]);

impl Display {
    pub fn new() -> Self {
        Self([[0; 64]; 32])
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    #[cfg(test)]
    pub fn new_filled() -> Self {
        Self([[0; 64]; 32])
    }
}
