use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        instruction_set::helpers::MSB_MASK,
        opcode::OpCode,
        status::Flags,
    },
    interpret_result::InstructionResult,
};

pub struct Compare {}

impl Compare {
    fn compare_set_flags(cpu: &mut Mos6502, left_operand: u8, right_operand: u8) {
        let result = left_operand.wrapping_sub(right_operand);

        cpu.status
            .set_status_flag(Flags::ZERO, left_operand == right_operand);

        cpu.status
            .set_status_flag(Flags::CARRY, left_operand >= right_operand);

        cpu.status
            .set_status_flag(Flags::NEGATIVE, result & MSB_MASK != 0);
    }

    // CMP - Compare
    pub fn cmp(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.bus.read(address);

        cpu.program_counter += opcode.bytes as u16;

        Compare::compare_set_flags(cpu, cpu.registers.a, memory_value);

        InstructionResult::Ok
    }

    // CPX - Compare X Register
    pub fn cpx(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if ![
            AddressMode::Immediate,
            AddressMode::ZeroPage,
            AddressMode::Absolute,
        ]
        .contains(&opcode.address_mode)
        {
            return InstructionResult::IllegalInstruction;
        }

        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.bus.read(address);

        cpu.program_counter += opcode.bytes as u16;

        Compare::compare_set_flags(cpu, cpu.registers.x, memory_value);

        InstructionResult::Ok
    }

    // CPY - Compare Y Register
    pub fn cpy(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if ![
            AddressMode::Immediate,
            AddressMode::ZeroPage,
            AddressMode::Absolute,
        ]
        .contains(&opcode.address_mode)
        {
            return InstructionResult::IllegalInstruction;
        }

        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.bus.read(address);

        cpu.program_counter += opcode.bytes as u16;

        Compare::compare_set_flags(cpu, cpu.registers.y, memory_value);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use crate::cpus::mos_6502::{
        address_mode::AddressMode, cpu::Registers, instruction_set::helpers::Helpers, status::Flags,
    };

    use super::*;

    #[parameterized]
    #[case(Flags::ZERO | Flags::CARRY, 0x01)]
    #[case(Flags::NEGATIVE, 0x05)]
    fn test_cmp(expected_flags: Flags, memory_value: u8) {
        let registers = Registers {
            a: 0x01,
            x: 0,
            y: 0,
        };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, memory_value)]),
            Some(registers),
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cmp(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected_flags, cpu.status);
    }

    #[parameterized]
    #[case(Flags::ZERO | Flags::CARRY, 0x01)]
    #[case(Flags::NEGATIVE, 0x05)]
    fn test_cpx(expected_flags: Flags, memory_value: u8) {
        let registers = Registers { a: 0, x: 0x1, y: 0 };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, memory_value)]),
            Some(registers),
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cpx(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected_flags, cpu.status);
    }

    #[parameterized]
    #[case(Flags::ZERO | Flags::CARRY, 0x01)]
    #[case(Flags::NEGATIVE, 0x05)]
    fn test_cpy(expected_flags: Flags, memory_value: u8) {
        let registers = Registers { a: 0, x: 0, y: 0x1 };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, memory_value)]),
            Some(registers),
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cpy(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected_flags, cpu.status);
    }
}
