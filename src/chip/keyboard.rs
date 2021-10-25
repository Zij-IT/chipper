#[derive(Debug, PartialEq, Eq)]
pub struct Keyboard {
    keys: [bool; 16],
    key: Option<u8>,
    wait: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
            key: None,
            wait: false,
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn set_keys(&mut self, keys: [bool; 16]) {
        self.keys = keys;
    }

    pub fn press_key(&mut self, key: Option<u8>) {
        if self.wait {
            self.key = key;
        }
    }

    pub fn get_next_key(&mut self) -> Option<u8> {
        let wait = self.wait;
        self.wait = !wait || self.key.is_none();
        self.key.take().filter(|_| wait)
    }
}
