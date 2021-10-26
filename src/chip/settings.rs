#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, PartialEq, Eq)]
pub struct Settings {
    pub rom_addr: u16,
    pub cpu_freq: u16,
    pub delay_freq: u16,
    pub sound_freq: u16,
    pub load_store_quirk: bool,
    pub shift_quirk: bool,
    pub index_overflow: bool,
    pub vertical_wrap: bool,
    pub jump_quirk: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            rom_addr: 0x200,
            cpu_freq: 700,
            delay_freq: 60,
            sound_freq: 60,
            load_store_quirk: false,
            index_overflow: false,
            vertical_wrap: false,
            shift_quirk: true,
            jump_quirk: false,
        }
    }
}
