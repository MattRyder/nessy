pub trait MemoryAccess {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
    fn write_slice(&mut self, start_address: u16, data: &[u8]);
}

const MEMORY_SIZE: usize = 0xFFFF;

// Memory Map
const _CPU_RAM_START: usize = 0x0000;
const _IO_REGISTERS_START: usize = 0x2000;
const _EXPANSION_ROM_START: usize = 0x4020;
const _SAVE_ROM_START: usize = 0x6000;
pub const PROGRAM_ROM_START: u16 = 0x8000;

#[derive(Debug)]
pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        // Zero inits the RAM but NES state could be garbage on hardware.
        Self {
            bytes: [0u8; MEMORY_SIZE],
        }
    }
}

impl MemoryAccess for Memory {
    fn read(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.bytes[address as usize] = data;
    }

    fn write_slice(&mut self, start_address: u16, data: &[u8]) {
        self.bytes[start_address as usize..(start_address as usize + data.len())]
            .copy_from_slice(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_returns_correct_value() {
        let mut memory = Memory::default();
        memory.bytes[0x100] = 0xAA;

        let byte_read = memory.read(0x100);

        assert_eq!(0xAA, byte_read);
    }

    #[test]
    fn test_write_sets_correct_value() {
        let mut memory = Memory::default();
        memory.write(0xFF0F, 0xAA);
        assert_eq!(0xAA, memory.bytes[0xFF0F]);
    }
}
