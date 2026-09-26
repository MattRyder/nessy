use crate::ppu::{ReadError, WriteError};

#[derive(Debug, PartialEq)]
pub enum BusError {
    // Returned when trying to access the read-only registers (typically, on the PPU).
    ReadFromWriteOnlyAddress,

    // Returned when trying to write to a read-only register.
    WriteFromReadOnlyAddress,

    // Returned when the caller attempts to write to the cartridge ROM.
    WriteToCartridgeROM,

    // Returned when the caller attempts to read from the cartridge when no cart is present.
    NoCartridgePresent,

    // Returned when the catch-all is invoked. This shouldn't ever be returned.
    UnsupportedAddressAccessed,

    // PPU Read error, refer to the inner error for more info.
    PPUReadError(ReadError),

    // PPU Write error, refer to the inner error for more info.
    PPUWriteError(WriteError),
}

impl From<ReadError> for BusError {
    fn from(value: ReadError) -> Self {
        BusError::PPUReadError(value)
    }
}

impl From<WriteError> for BusError {
    fn from(value: WriteError) -> Self {
        BusError::PPUWriteError(value)
    }
}
