use crate::{
    cpus::mos_6502::{address_mode::MemoryAddressing, cpu::Mos6502, opcode::OpCode, status::Flags},
    interpret_result::InstructionResult,
};

pub enum Direction {
    Left,
    Right,
}

const MSB_MASK: u8 = 0b1000_0000;
const LSB_MASK: u8 = 0b0000_0001;

pub struct Rotate {}

impl Rotate {
    fn set_flags(cpu: &mut Mos6502, value: u8, carry_value: bool) {
        cpu.status.set_zero_flag(value);

        cpu.status.set_negative_flag(value);

        cpu.status.set_status_flag(Flags::CARRY, carry_value);
    }

    // ROL - Rotate Left
    pub fn rotate_left(cpu: &mut Mos6502, operand: u8) -> u8 {
        let old_carry = cpu.status.contains(Flags::CARRY);
        let new_carry = operand & MSB_MASK != 0;

        let result = (operand << 1) | if old_carry { LSB_MASK } else { 0 };

        Self::set_flags(cpu, result, new_carry);

        result
    }

    // ROR - Rotate Right
    pub fn rotate_right(cpu: &mut Mos6502, operand: u8) -> u8 {
        let old_carry = cpu.status.contains(Flags::CARRY);
        let new_carry = operand & LSB_MASK != 0;

        let result = (operand >> 1) | if old_carry { MSB_MASK } else { 0 };

        Self::set_flags(cpu, result, new_carry);

        result
    }

    pub fn rotate_accumulator(cpu: &mut Mos6502, direction: Direction) -> InstructionResult {
        cpu.registers.a = match direction {
            Direction::Left => Rotate::rotate_left(cpu, cpu.registers.a),
            Direction::Right => Rotate::rotate_right(cpu, cpu.registers.a),
        };

        InstructionResult::Ok
    }

    pub fn rotate_memory(
        opcode: &OpCode,
        cpu: &mut Mos6502,
        direction: Direction,
    ) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.bus.read(address);
        cpu.program_counter += opcode.bytes as u16;

        let result = match direction {
            Direction::Left => Rotate::rotate_left(cpu, memory_value),
            Direction::Right => Rotate::rotate_right(cpu, memory_value),
        };
        cpu.bus.write(address, result);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode, cpu::Registers, instruction_set::helpers::Helpers,
    };

    #[parameterized]
    #[case(0x02, Direction::Left, 0x04, Flags::empty())]
    #[case(0x01, Direction::Right, 0x00, Flags::CARRY | Flags::ZERO)]
    #[case(0xFF, Direction::Left, 0xFE, Flags::CARRY | Flags::NEGATIVE)]
    #[case(0xFF, Direction::Right, 0x7F, Flags::CARRY)]
    fn test_rotate_accumulator(accumulator: u8, direction: Direction, expected: u8, flags: Flags) {
        let mut cpu = Helpers::create_cpu(
            0x0,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, 0x00)]),
            Some(Registers {
                a: accumulator,
                x: 0,
                y: 0,
            }),
            None,
        );

        let result = Rotate::rotate_accumulator(&mut cpu, direction);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected, cpu.registers.a);

        assert_eq!(flags, cpu.status);
    }

    #[parameterized]
    #[case(0x02, Direction::Left, 0x04, Flags::empty())]
    #[case(0x01, Direction::Right, 0x00, Flags::CARRY | Flags::ZERO)]
    #[case(0xFF, Direction::Left, 0xFE, Flags::CARRY | Flags::NEGATIVE)]
    #[case(0xFF, Direction::Right, 0x7F, Flags::CARRY)]
    fn test_rotate_memory(mem_value: u8, direction: Direction, expected: u8, flags: Flags) {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, mem_value)]),
            None,
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Rotate::rotate_memory(&opcode, &mut cpu, direction);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected, cpu.bus.read(0x02));

        assert_eq!(flags, cpu.status);
    }
}
