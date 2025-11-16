use crate::{
    cpus::mos_6502::{
        cpu::Mos6502, instruction_set::helpers::Helpers, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

// ORA - Logical Inclusive OR
pub struct Ora {}

impl Ora {
    pub fn ora(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        Helpers::accumulator_rule(opcode, cpu, |cpu, address| {
            cpu.registers.a |= cpu.memory.read(address);
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_hex::assert_eq_hex;

    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::Mos6502,
        instruction_set::helpers::Helpers,
        memory::{MEMORY_SIZE, Memory},
        status::Flags,
    };

    #[test]
    fn test_ora_does_a_bitwise_or() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0x32;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            ..Default::default()
        };

        cpu.registers.a = 0x19;

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Ora::ora(&opcode, &mut cpu);

        assert_eq_hex!(0x3B, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }
}
