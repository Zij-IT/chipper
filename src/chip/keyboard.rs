#[derive(Debug, PartialEq, Eq)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn set_keys(&mut self, keys: [bool; 16]) {
        self.keys = keys;
    }

    pub fn get_next_key(&self) -> Option<u8> {
        self.keys
            .iter()
            .copied()
            .zip(0_u8..)
            .find(|(pressed, _idx)| *pressed)
            .map(|(_pressed, idx)| idx)
    }
}
