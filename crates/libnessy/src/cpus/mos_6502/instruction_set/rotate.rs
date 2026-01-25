use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, instruction_set::helpers::MSB_MASK,
        memory::MemoryAccess, opcode::OpCode, status::Flags,
    },
    interpret_result::InstructionResult,
};

pub enum Direction {
    Left,
    Right,
}

pub struct Rotate {}

impl Rotate {
    // ROL - Rotate Left
    // ROR - Rotate Right
    pub fn rotate(cpu: &mut Mos6502, direction: Direction, operand: u8) -> u8 {
        let operand_msb = MSB_MASK & operand;

        let shifted = match direction {
            Direction::Left => operand.wrapping_shl(1),
            Direction::Right => operand.wrapping_shr(1),
        };

        let carry_bit: u8 = if cpu.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let result = shifted & 0xFE | carry_bit;

        cpu.status.set_zero_flag(result);

        cpu.status.set_negative_flag(result);

        cpu.status.set_status_flag(Flags::CARRY, operand_msb > 0);

        result
    }

    pub fn rotate_accumulator(cpu: &mut Mos6502, direction: Direction) -> InstructionResult {
        cpu.registers.a = Rotate::rotate(cpu, direction, cpu.registers.a);
        InstructionResult::Ok
    }

    pub fn rotate_memory(
        opcode: &OpCode,
        cpu: &mut Mos6502,
        direction: Direction,
    ) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.memory.read(address);
        cpu.program_counter += opcode.bytes as u16;

        let result = Rotate::rotate(cpu, direction, memory_value);

        cpu.memory.write(address, result);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::{Mos6502, Registers},
        instruction_set::helpers::Helpers,
        memory::Memory,
    };

    #[parameterized]
    #[case(0x02, Direction::Left, 0x04, Flags::empty())]
    #[case(0x01, Direction::Right, 0x00, Flags::ZERO)]
    #[case(0xFF, Direction::Left, 0xFE, Flags::CARRY | Flags::NEGATIVE)]
    #[case(0xFF, Direction::Right, 0x7E, Flags::CARRY)]
    fn test_rotate_accumulator(accumulator: u8, direction: Direction, expected: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: accumulator,
                x: 0,
                y: 0,
            },
            ..Default::default()
        };

        let result = Rotate::rotate_accumulator(&mut cpu, direction);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected, cpu.registers.a);

        assert_eq!(flags, cpu.status);
    }

    #[parameterized]
    #[case(0x02, Direction::Left, 0x04, Flags::empty())]
    #[case(0x01, Direction::Right, 0x00, Flags::ZERO)]
    #[case(0xFF, Direction::Left, 0xFE, Flags::CARRY | Flags::NEGATIVE)]
    #[case(0xFF, Direction::Right, 0x7E, Flags::CARRY)]
    fn test_rotate_memory(mem_value: u8, direction: Direction, expected: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![(0xAA, 0x02), (0x02, mem_value)]),
            program_counter: 0xAA,
            registers: Registers {
                a: 0x01,
                x: 0,
                y: 0,
            },
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Rotate::rotate_memory(&opcode, &mut cpu, direction);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected, cpu.memory.read(0x02));

        assert_eq!(flags, cpu.status);
    }
}
