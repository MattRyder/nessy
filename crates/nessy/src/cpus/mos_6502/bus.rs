use crate::roms::ROM;

// Constants
const MEMORY_SIZE: usize = 2048;

// Records how large a 16KB block is, for PRG ROM mirroring.
const SIXTEEN_KILOBYTES: usize = 0x4000;

// NES RAM Mirroring:
//
// The NES CPU RAM has 2k KiB available, by nature of the 11 lines attached from CPU to RAM.
// So reading any 12+ bit address results in those extra msb bits being ignored, effectively
// causing an 11-bit address read, at most.
//
// Given this wrapping, the memory is thus mirrored a few times after the 11-bits.
// Addressing 0x0000, 0x8000, and 0x1000 all return the exact same physical byte in RAM.
//
// [0x0000 - 0x07FF] 'original' memory address
// [0x0800 - 0x0FFF] mirrored
// [0x1000 - 0x17FF] mirrored
// [0x1800 - 0x1FFF] mirrored
//
const CPU_RAM_START: u16 = 0x0000;
const CPU_RAM_MIRROR_RANGE_END: u16 = 0x1FFF;
const CPU_RAM_ADDRESS_MASK: u16 = 0x07FF;

const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_RANGE_END: u16 = 0x3FFF;
const PPU_REGISTERS_MASK: u16 = 0x2007;

const CARTRIDGE_START: u16 = 0x4020;
const CARTRIDGE_END: u16 = 0xFFFF;

pub trait MemoryBus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
    fn write_slice(&mut self, start_address: u16, data: &[u8]);

    fn read_u16(&self, address: u16) -> u16;
    fn write_u16(&mut self, address: u16, data: u16);

    fn insert_rom(&mut self, rom: ROM);
}

#[derive(Debug)]
pub struct Bus {
    cpu_memory: [u8; MEMORY_SIZE],
    rom: Option<ROM>,
}

impl Default for Bus {
    fn default() -> Self {
        // Zero inits the RAM but NES state could be garbage on hardware.
        Self {
            cpu_memory: [0; MEMORY_SIZE],
            rom: None,
        }
    }
}

impl Bus {
    pub fn new(cpu_memory: [u8; MEMORY_SIZE], rom: Option<ROM>) -> Self {
        Self { cpu_memory, rom }
    }
}

impl MemoryBus for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                self.cpu_memory[addr]
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_RANGE_END => {
                let _addr = (address & PPU_REGISTERS_MASK) as usize;
                todo!("PPU read");
            }
            CARTRIDGE_START..=CARTRIDGE_END => {
                // PRG ROM size can be 16KB or 32KB.
                // As there's a 32KB addressible space here, a 16KB ROM basically
                // means that it's mirrored (i.e. mapped to the lower 16KB.)
                // If we've got a 16KB PRG ROM, and the address is over 16KB, map it down.
                //
                if let Some(rom) = self.rom.as_ref() {
                    let mut addr = address as usize;

                    if rom.program_rom().len() == SIXTEEN_KILOBYTES && addr >= SIXTEEN_KILOBYTES {
                        addr %= SIXTEEN_KILOBYTES;
                    }

                    rom.program_rom()[addr]
                } else {
                    panic!("Can't access the cartridge rom!");
                }
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    address
                );
                0
            }
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                self.cpu_memory[addr] = data;
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_RANGE_END => {
                let _addr = (address & PPU_REGISTERS_MASK) as usize;
                todo!("PPU write")
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    address
                );
            }
        }
    }

    fn write_slice(&mut self, start_address: u16, data: &[u8]) {
        // TODO: this should check the entire range is valid for the ram segment being written.
        match start_address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (start_address & CPU_RAM_ADDRESS_MASK) as usize;
                self.cpu_memory[addr..(addr + data.len())].copy_from_slice(data);
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_RANGE_END => {
                let _addr = (start_address & PPU_REGISTERS_MASK) as usize;
                todo!("PPU write_slice")
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    start_address
                );
            }
        }
    }

    fn read_u16(&self, address: u16) -> u16 {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                let addr_u16 = addr as u16;
                let lo_byte = self.read(addr_u16) as u16;
                let hi_byte = self.read(addr_u16 + 1) as u16;
                (hi_byte << 8) | lo_byte
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_RANGE_END => {
                let _addr = (address & PPU_REGISTERS_MASK) as usize;
                todo!("PPU read_u16");
            }
            CARTRIDGE_START..=CARTRIDGE_END => {
                let lo_byte = self.read(address) as u16;
                let hi_byte = self.read(address + 1) as u16;
                (hi_byte << 8) | lo_byte
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    address
                );
                0
            }
        }
    }

    fn write_u16(&mut self, address: u16, data: u16) {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                let addr_u16 = addr as u16;
                let hi_byte = (data >> 8) as u8;
                let lo_byte = (data & 0xFF) as u8;
                self.write(addr_u16, lo_byte);
                self.write(addr_u16 + 1, hi_byte);
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_MIRROR_RANGE_END => {
                let _addr = (address & PPU_REGISTERS_MASK) as usize;
                todo!("PPU read_u16");
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    address
                );
            }
        }
    }

    fn insert_rom(&mut self, rom: ROM) {
        self.rom = Some(rom);
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;

    fn setup_memory(values: Vec<(u8, u8)>) -> [u8; MEMORY_SIZE] {
        let mut memory = [0; MEMORY_SIZE];
        for (addr, value) in values {
            memory[addr as usize] = value;
        }
        memory
    }

    #[test]
    fn test_read_returns_correct_value() {
        let memory = setup_memory(vec![(0x10, 0xAA)]);
        let bus = Bus::new(memory, None);

        assert_eq_hex!(0xAA, bus.read(0x10));
    }

    #[test]
    fn test_write_sets_correct_value() {
        let memory = setup_memory(vec![]);
        let mut bus = Bus::new(memory, None);

        bus.write(0x400, 0xAA);

        assert_eq_hex!(0xAA, bus.cpu_memory[0x400]);
    }

    #[parameterized]
    #[case(vec![(0x50, 0xAA), (0x51, 0xBB)], 0x50, 0xBBAA)]
    fn test_read_u16_returns_correct_value(
        memory: Vec<(u8, u8)>,
        address: u16,
        expected_value: u16,
    ) {
        let memory = setup_memory(memory);
        let bus = Bus::new(memory, None);

        assert_eq_hex!(expected_value, bus.read_u16(address));
    }

    #[test]
    fn test_write_u16_set_correct_value() {
        let memory = setup_memory(vec![]);
        let mut bus = Bus::new(memory, None);

        bus.write_u16(0x0001, 0xAABB);

        assert_eq!(0xBB, bus.cpu_memory[0x0001]);
        assert_eq!(0xAA, bus.cpu_memory[0x0002]);
    }
}
