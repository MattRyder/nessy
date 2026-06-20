use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        opcode::OpCode,
        status::Flags,
    },
    interpret_result::InstructionResult,
};

// The following instructions perform logical operations on the contents
// of the accumulator and another value held in memory.
pub struct Logical {}

impl Logical {
    fn accumulator_rule(
        opcode: &OpCode,
        cpu: &mut Mos6502,
        operation: fn(cpu: &mut Mos6502, address: u16),
    ) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        cpu.program_counter += opcode.bytes as u16;

        operation(cpu, address);

        cpu.status.set_zero_flag(cpu.registers.a);
        cpu.status.set_negative_flag(cpu.registers.a);

        InstructionResult::Ok
    }

    // AND - Logical AND
    pub fn and(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        Logical::accumulator_rule(opcode, cpu, |cpu, address| {
            cpu.registers.a &= cpu.bus.read(address);
        })
    }

    // EOR - Exclusive OR
    pub fn eor(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        Logical::accumulator_rule(opcode, cpu, |cpu, address| {
            cpu.registers.a ^= cpu.bus.read(address);
        })
    }

    // ORA - Logical Inclusive OR
    pub fn ora(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        Logical::accumulator_rule(opcode, cpu, |cpu, address| {
            cpu.registers.a |= cpu.bus.read(address);
        })
    }

    // BIT - Bit Test
    pub fn bit(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if ![AddressMode::ZeroPage, AddressMode::Absolute].contains(&opcode.address_mode) {
            return InstructionResult::IllegalInstruction;
        }

        let address = cpu.get_address(&opcode.address_mode);
        cpu.program_counter += opcode.bytes as u16;

        let memory_value = cpu.bus.read(address);

        let acca_and_mem = cpu.registers.a & memory_value;

        cpu.status.set_zero_flag(acca_and_mem);
        cpu.status.set_negative_flag(memory_value);
        cpu.status
            .set_status_flag(Flags::OVERFLOW, memory_value & 0b0100_0000 != 0);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode, cpu::Registers, instruction_set::helpers::Helpers, status::Flags,
    };

    #[parameterized]
    #[case(vec![(0xAA, 0x0B)], Registers { a: 0x05, x: 0, y: 0}, 0x01, Flags::empty())]
    #[case(vec![(0xAA, 0x00)], Registers { a: 0x04, x: 0, y: 0}, 0x00, Flags::ZERO)]
    fn test_and(
        memory: Vec<(u16, u8)>,
        registers: Registers,
        expected_a_reg: u8,
        expected_flags: Flags,
    ) {
        let mut cpu = Helpers::create_cpu(0xAA, 0x0, Some(memory), Some(registers), None);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Logical::and(&opcode, &mut cpu);

        assert_eq_hex!(expected_a_reg, cpu.registers.a);
        assert_eq!(expected_flags, cpu.status);
    }

    #[test]
    fn test_ora_does_a_bitwise_or() {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x32)]),
            Some(Registers {
                a: 0x19,
                x: 0,
                y: 0,
            }),
            None,
        );

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Logical::ora(&opcode, &mut cpu);

        assert_eq_hex!(0x3B, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status & Flags::NEGATIVE);
    }

    #[test]
    fn test_bit_sets_overflow_flags() {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, 0xFF)]),
            Some(Registers {
                a: 0x7F,
                x: 0,
                y: 0,
            }),
            None,
        );

        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPage);

        Logical::bit(&opcode, &mut cpu);

        assert_eq_hex!(0x7F, cpu.registers.a);
        assert_eq!(Flags::OVERFLOW | Flags::NEGATIVE, cpu.status);
    }

    #[test]
    fn test_bit_sets_zero_flag() {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, 0x00)]),
            Some(Registers {
                a: 0x00,
                x: 0,
                y: 0,
            }),
            None,
        );

        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPage);

        Logical::bit(&opcode, &mut cpu);

        assert_eq_hex!(0x00, cpu.registers.a);
        assert_eq!(Flags::ZERO, cpu.status);
    }
}
