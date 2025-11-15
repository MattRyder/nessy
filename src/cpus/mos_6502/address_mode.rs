use super::cpu::Mos6502;
use super::memory::MemoryAccess;

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Accumulator,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    None,
}

pub trait MemoryAddressing {
    fn get_address(&self, address_mode: &AddressMode) -> u16;
}

impl MemoryAddressing for Mos6502 {
    fn get_address(&self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            AddressMode::Accumulator => panic!("Don't use get_address for accumulator."),
            AddressMode::Immediate => self.program_counter,
            AddressMode::ZeroPage => self.memory.read(self.program_counter) as u16,
            AddressMode::ZeroPageX => {
                let pc_address = self.memory.read(self.program_counter);
                pc_address.wrapping_add(self.registers.x) as u16
            }
            AddressMode::ZeroPageY => {
                let pc_address = self.memory.read(self.program_counter);
                pc_address.wrapping_add(self.registers.y) as u16
            }
            AddressMode::Absolute => self.memory.read_u16(self.program_counter),
            AddressMode::AbsoluteX => {
                let pc_address = self.memory.read_u16(self.program_counter);
                pc_address.wrapping_add(self.registers.x as u16)
            }
            AddressMode::AbsoluteY => {
                let pc_address = self.memory.read_u16(self.program_counter);
                pc_address.wrapping_add(self.registers.y as u16)
            }
            AddressMode::IndirectX => {
                let base_address = self.memory.read(self.program_counter);

                let pointer = base_address.wrapping_add(self.registers.x);
                let lo = self.memory.read(pointer as u16);
                let hi = self.memory.read(pointer.wrapping_add(1) as u16);

                (hi as u16) << 8 | (lo as u16)
            }
            AddressMode::IndirectY => {
                let base_address = self.memory.read(self.program_counter);

                let lo = self.memory.read(base_address as u16);
                let hi = self.memory.read((base_address).wrapping_add(1) as u16);

                let dereference_base = (hi as u16) << 8 | (lo as u16);
                dereference_base.wrapping_add(self.registers.y as u16)
            }
            AddressMode::None | AddressMode::Implied => {
                panic!("Unsupported address mode: {:?}", &address_mode)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpus::mos_6502::memory::{MEMORY_SIZE, Memory};
    use crate::cpus::mos_6502::{address_mode::MemoryAddressing, cpu::Mos6502};

    #[test]
    #[should_panic(expected = "Unsupported address mode: None")]
    fn test_get_address_with_none_return_err() {
        let cpu = Mos6502::default();
        cpu.get_address(&AddressMode::None);
    }

    #[test]
    fn test_get_address_with_immediate_returns_value_at_pc() {
        let cpu = Mos6502 {
            program_counter: 0xFF,
            ..Default::default()
        };

        let result = cpu.get_address(&AddressMode::Immediate);

        assert_eq!(0xFF, result);
    }

    #[test]
    fn test_get_address_with_zero_page_returns_u16_at_pc() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0x10] = 0xAA;

        let cpu = Mos6502 {
            program_counter: 0x10,
            memory: Memory::new(memory),
            ..Default::default()
        };

        let result = cpu.get_address(&AddressMode::ZeroPage);

        assert_eq!(0xAA, result);
    }

    #[test]
    fn test_get_address_with_zero_page_x_returns_pc_add_register_x() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0x05] = 0xBB;

        let mut cpu = Mos6502 {
            program_counter: 0x05,
            memory: Memory::new(memory),
            ..Default::default()
        };

        cpu.registers.x = 0x01;

        let result = cpu.get_address(&AddressMode::ZeroPageX);

        assert_eq!(0xBC, result);
    }

    #[test]
    fn test_get_address_with_zero_page_y_returns_pc_add_register_y() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0xAA] = 0xC5;

        let mut cpu = Mos6502 {
            program_counter: 0xAA,
            memory: Memory::new(memory),
            ..Default::default()
        };

        cpu.registers.y = 0x01;

        let result = cpu.get_address(&AddressMode::ZeroPageY);

        assert_eq!(0xC6, result);
    }

    #[test]
    fn test_get_address_with_absolute_returns_value_at_pc_address() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0xAA] = 0x34;
        memory[0xAB] = 0x12;

        let cpu = Mos6502 {
            program_counter: 0xAA,
            memory: Memory::new(memory),
            ..Default::default()
        };

        let result = cpu.get_address(&AddressMode::Absolute);

        assert_eq!(0x1234, result);
    }

    #[test]
    fn test_get_address_with_absolute_x_returns_value_at_pc_address_plus_x_register() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0xAA] = 0x34;
        memory[0xAB] = 0x12;

        let mut cpu = Mos6502 {
            program_counter: 0xAA,
            memory: Memory::new(memory),
            ..Default::default()
        };

        cpu.registers.x = 0x01;

        let result = cpu.get_address(&AddressMode::AbsoluteX);

        assert_eq!(0x1235, result);
    }

    #[test]
    fn test_get_address_with_absolute_y_returns_value_at_pc_address_plus_y_register() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0xAA] = 0x34;
        memory[0xAB] = 0x12;

        let mut cpu = Mos6502 {
            program_counter: 0xAA,
            memory: Memory::new(memory),
            ..Default::default()
        };

        cpu.registers.y = 0x01;

        let result = cpu.get_address(&AddressMode::AbsoluteY);

        assert_eq!(0x1235, result);
    }

    #[test]
    fn test_get_address_with_indirect_x_returns_address() {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[0xAA] = 0x13;

        // The indirect read address
        memory[0x14] = 0xFC;
        memory[0x15] = 0xBA;

        let mut cpu = Mos6502 {
            program_counter: 0xAA,
            memory: Memory::new(memory),
            ..Default::default()
        };

        cpu.registers.x = 0x01;

        let result = cpu.get_address(&AddressMode::IndirectX);

        assert_eq!(0xBAFC, result);
    }

    #[test]
    fn test_get_address_with_indirect_y_returns_address() {
        let mut memory = [0u8; MEMORY_SIZE];

        // Setup the memory with the address we're going to:
        memory[0xAA] = 0x50;
        memory[0x50] = 0xFB;
        memory[0x51] = 0xFF;

        let mut cpu = Mos6502 {
            program_counter: 0xAA,
            memory: Memory::new(memory),
            ..Default::default()
        };

        cpu.registers.y = 0x01;

        let result = cpu.get_address(&AddressMode::IndirectY);

        assert_eq!(0xFFFC, result);
    }
}
