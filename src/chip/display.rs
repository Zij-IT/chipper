pub struct Display([[u8; 32]; 64]);

impl Display {
    pub fn new() -> Self {
        Self([[0; 32]; 64])
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}
