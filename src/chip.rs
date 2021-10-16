pub struct Chip8 {
    frame_buffer: Display,
    memory: Memory,
    v: Registers,
    stack: Stack,
    index: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {

}
