pub const MEMORY_SIZE: usize = 0xFFFF;

// Memory Map
const _CPU_RAM_START: usize = 0x0000;
const _IO_REGISTERS_START: usize = 0x2000;
const _EXPANSION_ROM_START: usize = 0x4020;
const _SAVE_ROM_START: usize = 0x6000;

// pub const PROGRAM_ROM_START: u16 = 0x8000;
pub const PROGRAM_ROM_START: u16 = 0x0600;
