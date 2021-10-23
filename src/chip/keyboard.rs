#[derive(PartialEq, Eq, Debug)]
pub struct Keyboard {
    keys: [bool; 16],
    last_key: u8,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            last_key: 0,
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn press_key(&mut self, key: u8) {
        self.keys[key as usize] = true;
        self.last_key = key;
    }

    pub fn release_key(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }
}
