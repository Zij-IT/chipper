#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    SysAddr(u16),
    Clear,
    Return,
    Jump(u16),
    Call(u16),
    SkipEqual(u8, u8),
    SkipNotEqual(u8, u8),
    SkipEqualRegister(u8, u8),
    Load(u8, u8),
    Add(u8, u8),
    LoadRegister(u8, u8),
    OrRegister(u8, u8),
    AndRegister(u8, u8),
    XorRegister(u8, u8),
    AddRegister(u8, u8),
    SubRegister(u8, u8),
    ShiftRightRegister(u8, u8),
    SubReverseRegister(u8, u8),
    ShiftLeftRegister(u8, u8),
    SkipNotEqualRegister(u8, u8),
    SetIndexRegister(u16),
    JumpWithOffset(u16),
    Random(u8, u8),
    Draw(u8, u8, u8),
    SkipKeyPressed(u8),
    SkipKeyNotPressed(u8),
    LoadDelay(u8),
    LoadNextKeyPress(u8),
    SetDelayTimer(u8),
    SetSoundTimer(u8),
    AddIndexRegister(u8),
    IndexAtSprite(u8),
    BinaryCodeConversion(u8),
    StoreAllRegisters(u8),
    LoadAllRegisters(u8),
}

impl From<u16> for OpCode {
    fn from(op: u16) -> Self {
        let nibbles = (
            ((op & 0xF000) >> 12) as u8,
            ((op & 0x0F00) >> 8) as u8,
            ((op & 0x00F0) >> 4) as u8,
            (op & 0x000F) as u8,
        );

        let nnn = op & 0x0FFF;
        let kk = (op & 0x00FF) as u8;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Self::Clear,
            (0x0, 0x0, 0xE, 0xE) => Self::Return,
            (0x0, _, _, _) => Self::SysAddr(nnn),
            (0x1, _, _, _) => Self::Jump(nnn),
            (0x2, _, _, _) => Self::Call(nnn),
            (0x3, _, _, _) => Self::SkipEqual(x, kk),
            (0x4, _, _, _) => Self::SkipNotEqual(x, kk),
            (0x5, _, _, 0x0) => Self::SkipEqualRegister(x, y),
            (0x6, _, _, _) => Self::Load(x, kk),
            (0x7, _, _, _) => Self::Add(x, kk),
            (0x8, _, _, 0x0) => Self::LoadRegister(x, y),
            (0x8, _, _, 0x1) => Self::OrRegister(x, y),
            (0x8, _, _, 0x2) => Self::AndRegister(x, y),
            (0x8, _, _, 0x3) => Self::XorRegister(x, y),
            (0x8, _, _, 0x4) => Self::AddRegister(x, y),
            (0x8, _, _, 0x5) => Self::SubRegister(x, y),
            (0x8, _, _, 0x6) => Self::ShiftRightRegister(x, y),
            (0x8, _, _, 0x7) => Self::SubReverseRegister(x, y),
            (0x8, _, _, 0xE) => Self::ShiftLeftRegister(x, y),
            (0x9, _, _, 0x0) => Self::SkipNotEqualRegister(x, y),
            (0xA, _, _, _) => Self::SetIndexRegister(nnn),
            (0xB, _, _, _) => Self::JumpWithOffset(nnn),
            (0xC, _, _, _) => Self::Random(x, kk),
            (0xD, _, _, _) => Self::Draw(x, y, n),
            (0xE, _, 0x9, 0xE) => Self::SkipKeyPressed(x),
            (0xE, _, 0xA, 0x1) => Self::SkipKeyNotPressed(x),
            (0xF, _, 0x0, 0x7) => Self::LoadDelay(x),
            (0xF, _, 0x0, 0xA) => Self::LoadNextKeyPress(x),
            (0xF, _, 0x1, 0x5) => Self::SetDelayTimer(x),
            (0xF, _, 0x1, 0x8) => Self::SetSoundTimer(x),
            (0xF, _, 0x1, 0xE) => Self::AddIndexRegister(x),
            (0xF, _, 0x2, 0x9) => Self::IndexAtSprite(x),
            (0xF, _, 0x3, 0x3) => Self::BinaryCodeConversion(x),
            (0xF, _, 0x5, 0x5) => Self::StoreAllRegisters(x),
            (0xF, _, 0x6, 0x5) => Self::LoadAllRegisters(x),
            _ => unreachable!(),
        }
    }
}
