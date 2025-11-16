use crate::{
    cpus::mos_6502::{
        cpu::Mos6502, instruction_set::helpers::Helpers, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

// AND - Logical AND
pub struct And {}

impl And {
    pub fn and(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        Helpers::accumulator_rule(opcode, cpu, |cpu, address| {
            cpu.registers.a &= cpu.memory.read(address);
        })
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::Registers,
        instruction_set::helpers::Helpers,
        memory::{MEMORY_SIZE, Memory},
        status::Flags,
    };

    #[test]
    fn test_and() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0xB;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            registers: Registers {
                a: 0x05,
                x: 0,
                y: 0,
            },
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        And::and(&opcode, &mut cpu);

        assert_eq_hex!(0x01, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags);
    }

    #[test]
    fn test_and_zero_flag_set() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0x00;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            registers: Registers {
                a: 0x04,
                x: 0,
                y: 0,
            },
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        And::and(&opcode, &mut cpu);

        assert_eq!(Flags::ZERO, cpu.status.flags & Flags::ZERO);
    }
}
