use crate::models::errors::ChipErrors;

pub enum Opcode {
    SetI(u16),
    SetVConstant(u8, u8),
    SetV(u8, u8),
    ClearScreen,
    ReturnFromSubroutine,
    CallSubroutine(u16),
    Add(u8, u8),
    Subtract(u8, u8),
    SubtractOpposite(u8, u8),
    ShiftLeft(u8),
    ShiftRight(u8),
    Or(u8, u8),
    And(u8, u8),
    Xor(u8, u8),
    AddConstant(u8, u8),
    BinaryCodedDecimal(u8),
    SkipRegistersEqual(u8, u8),
    SkipRegistersNonEqual(u8, u8),
    SkipEqual(u8, u8),
    SkipNonEqual(u8, u8),
    SkipKeyEqual(u8),
    SkipKeyNonEqual(u8),
    Draw(u8, u8, u8),
    Jump(u16),
    JumpPlus(u16),
    SetDelayTimer(u8),
    GetDelayTimer(u8),
    Dump(u8),
    Load(u8),
    SpriteAddress(u8),
    RandAnd(u8, u8),
    AddMemory(u8),
    GetKey(u8),
}

impl Opcode {
    pub fn parse(code: u16) -> Result<Self, ChipErrors> {
        if code & 0xF000 == 0xA000 {
            return Ok(Self::SetI(code & 0x0FFF));
        } else if code == 0x00E0 {
            return Ok(Self::ClearScreen);
        } else if code == 0x00EE {
            return Ok(Self::ReturnFromSubroutine);
        } else if code & 0xF000 == 0x2000 {
            return Ok(Self::CallSubroutine(code & 0x0FFF));
        } else if code & 0xF00F == 0x8004 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::Add(x, y));
        } else if code & 0xF0FF == 0xF033 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::BinaryCodedDecimal(x));
        } else if code & 0xF000 == 0xD000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            let n = ((code & 0x000F) >> 0) as u8;
            return Ok(Self::Draw(x, y, n));
        } else if code & 0xF000 == 0x6000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let n = ((code & 0x00FF) >> 0) as u8;
            return Ok(Self::SetVConstant(x, n));
        } else if code & 0xF00F == 0x8000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::SetV(x, y));
        } else if code & 0xF000 == 0xC000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let n = ((code & 0x00FF) >> 0) as u8;
            return Ok(Self::RandAnd(x, n));
        } else if code & 0xF00F == 0x8001 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::Or(x, y));
        } else if code & 0xF00F == 0x8002 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::And(x, y));
        } else if code & 0xF00F == 0x8003 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::Xor(x, y));
        } else if code & 0xF00F == 0x800E {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::ShiftLeft(x));
        } else if code & 0xF00F == 0x8006 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::ShiftRight(x));
        } else if code & 0xF00F == 0x8005 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::Subtract(x, y));
        } else if code & 0xF00F == 0x8007 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::SubtractOpposite(x, y));
        } else if code & 0xF000 == 0x7000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let n = ((code & 0x00FF) >> 0) as u8;
            return Ok(Self::AddConstant(x, n));
        } else if code & 0xF000 == 0x1000 {
            let n = code & 0x0FFF;
            return Ok(Self::Jump(n));
        } else if code & 0xF000 == 0xB000 {
            let n = code & 0x0FFF;
            return Ok(Self::JumpPlus(n));
        } else if code & 0xF000 == 0x3000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let n = ((code & 0x00FF) >> 0) as u8;
            return Ok(Self::SkipEqual(x, n));
        } else if code & 0xF000 == 0x4000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let n = ((code & 0x00FF) >> 0) as u8;
            return Ok(Self::SkipNonEqual(x, n));
        } else if code & 0xF0FF == 0xF015 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::SetDelayTimer(x));
        } else if code & 0xF0FF == 0xF055 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::Dump(x));
        } else if code & 0xF0FF == 0xF065 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::Load(x));
        } else if code & 0xF0FF == 0xF029 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::SpriteAddress(x));
        } else if code & 0xF0FF == 0xF007 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::GetDelayTimer(x));
        } else if code & 0xF0FF == 0xF01E {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::AddMemory(x));
        } else if code & 0xF0FF == 0xF00A {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::GetKey(x));
        } else if code & 0xF0FF == 0xE09E {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::SkipKeyEqual(x));
        } else if code & 0xF0FF == 0xE0A1 {
            let x = ((code & 0x0F00) >> 8) as u8;
            return Ok(Self::SkipKeyNonEqual(x));
        } else if code & 0xF00F == 0x5000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::SkipRegistersEqual(x, y));
        } else if code & 0xF00F == 0x9000 {
            let x = ((code & 0x0F00) >> 8) as u8;
            let y = ((code & 0x00F0) >> 4) as u8;
            return Ok(Self::SkipRegistersNonEqual(x, y));
        } else {
            Err(ChipErrors::UnknownOpcode(code))
        }
    }
}
