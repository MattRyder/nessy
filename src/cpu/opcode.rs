pub enum OpCode {
    Break,
    LoadAccumulator,
    TransferAccumulatorToX,
    IncrementX,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::Break),
            0xA9 => Ok(OpCode::LoadAccumulator),
            0xAA => Ok(OpCode::TransferAccumulatorToX),
            0xE8 => Ok(OpCode::IncrementX),
            _ => Err(()),
        }
    }
}
