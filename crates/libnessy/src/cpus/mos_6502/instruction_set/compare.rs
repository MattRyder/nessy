use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        instruction_set::helpers::MSB_MASK,
        memory::MemoryAccess,
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
        let memory_value = cpu.memory.read(address);

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
        let memory_value = cpu.memory.read(address);

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
        let memory_value = cpu.memory.read(address);

        cpu.program_counter += opcode.bytes as u16;

        Compare::compare_set_flags(cpu, cpu.registers.y, memory_value);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use crate::cpus::mos_6502::{
        address_mode::AddressMode, cpu::Registers, instruction_set::helpers::Helpers,
        memory::Memory, status::Flags,
    };

    use super::*;

    #[test]
    fn test_cmp_sets_zero_and_carry_flag() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0x01,
                x: 0,
                y: 0,
            },
            program_counter: 0xAA,
            memory: Memory::new_with_bytes(vec![(0xAA, 0x01)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cmp(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(Flags::ZERO | Flags::CARRY, cpu.status.flags);
    }

    #[test]
    fn test_cmp_sets_negative_flag() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0x01,
                x: 0,
                y: 0,
            },
            program_counter: 0xAA,
            memory: Memory::new_with_bytes(vec![(0xAA, 0x05)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cmp(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn test_cpx_sets_zero_and_carry_flag() {
        let mut cpu = Mos6502 {
            registers: Registers { a: 0, x: 0x1, y: 0 },
            program_counter: 0xAA,
            memory: Memory::new_with_bytes(vec![(0xAA, 0x01)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cpx(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(Flags::ZERO | Flags::CARRY, cpu.status.flags);
    }

    #[test]
    fn test_cpx_sets_negative_flag() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: 0x01,
                y: 0,
            },
            program_counter: 0xAA,
            memory: Memory::new_with_bytes(vec![(0xAA, 0x05)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cpx(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn test_cpy_sets_zero_and_carry_flag() {
        let mut cpu = Mos6502 {
            registers: Registers { a: 0, x: 0, y: 0x1 },
            program_counter: 0xAA,
            memory: Memory::new_with_bytes(vec![(0xAA, 0x01)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cpy(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(Flags::ZERO | Flags::CARRY, cpu.status.flags);
    }

    #[test]
    fn test_cpy_sets_negative_flag() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0x01,
            },
            program_counter: 0xAA,
            memory: Memory::new_with_bytes(vec![(0xAA, 0x05)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Immediate);

        let result = Compare::cpy(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(Flags::NEGATIVE, cpu.status.flags);
    }
}
