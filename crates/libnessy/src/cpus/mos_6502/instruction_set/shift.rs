use std::ops::{Shl, Shr};

use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, instruction_set::helpers::MSB_MASK,
        memory::MemoryAccess, opcode::OpCode, status::Flags,
    },
    interpret_result::InstructionResult,
};

pub struct Shift {}

impl Shift {
    fn arithmetic_shift(cpu: &mut Mos6502, value: u8) -> u8 {
        let shifted_result = value.shl(1);

        cpu.status
            .set_status_flag(Flags::CARRY, value & MSB_MASK != 0);
        cpu.status.set_zero_flag(shifted_result);
        cpu.status.set_negative_flag(shifted_result);

        shifted_result
    }

    fn logical_shift(cpu: &mut Mos6502, value: u8) -> u8 {
        let shifted_result = value.shr(1);

        cpu.status.set_status_flag(Flags::CARRY, value & 0x01 != 0);
        cpu.status.set_zero_flag(shifted_result);
        cpu.status.set_negative_flag(shifted_result);

        shifted_result
    }

    // ASL - Arithmetic Shift Left
    pub fn asl_accumulator(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = Shift::arithmetic_shift(cpu, cpu.registers.a);

        InstructionResult::Ok
    }

    pub fn asl_memory(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let address_value = cpu.bus.read(address);
        cpu.program_counter += opcode.bytes as u16;

        let shifted_result = Shift::arithmetic_shift(cpu, address_value);

        cpu.bus.write(address, shifted_result);

        InstructionResult::Ok
    }

    // LSR - Logical Shift Right
    pub fn lsr_accumulator(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = Shift::logical_shift(cpu, cpu.registers.a);

        InstructionResult::Ok
    }

    pub fn lsr_memory(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let address_value = cpu.bus.read(address);
        cpu.program_counter += opcode.bytes as u16;

        let shifted_result = Shift::logical_shift(cpu, address_value);

        cpu.bus.write(address, shifted_result);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use sif::parameterized;

    use super::*;
    use crate::{
        assert_memory_value, assert_registers,
        cpus::mos_6502::{
            address_mode::AddressMode,
            cpu::{Mos6502, Registers},
            instruction_set::helpers::Helpers,
            status::Flags,
        },
    };

    #[test]
    fn test_asl_with_accumulator_does_bitwise_shift() {
        let mut cpu = Mos6502 {
            registers: Registers { a: 0x2, x: 0, y: 0 },
            program_counter: 0xAA,
            ..Default::default()
        };

        Shift::asl_accumulator(&mut cpu);

        assert_registers!(cpu, 0x04, 0, 0);
        assert_eq!(Flags::empty(), cpu.status);
    }

    #[test]
    fn test_asl_with_memory_does_bitwise_shift() {
        let mut cpu =
            Helpers::create_cpu(0x20, 0x0, Some(vec![(0x20, 0xAA), (0xAA, 0x8)]), None, None);

        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPage);

        Shift::asl_memory(&opcode, &mut cpu);

        assert_registers!(cpu, 0, 0, 0);

        assert_memory_value!(&cpu.bus, 0xAA, 0x10);
    }

    #[parameterized]
    #[case(0x02, 0x01, Flags::empty())]
    #[case(0xFF, 0x7F, Flags::CARRY)]
    fn test_lsr_with_accumulator(register_a: u8, expected_a: u8, expected_flags: Flags) {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: register_a,
                x: 0,
                y: 0,
            },
            program_counter: 0xAA,
            ..Default::default()
        };

        Shift::lsr_accumulator(&mut cpu);

        assert_registers!(cpu, expected_a, 0, 0);
        assert_eq!(expected_flags, cpu.status);
    }

    #[test]
    fn test_lsr_with_memory_does_bitwise_shift() {
        let mut cpu =
            Helpers::create_cpu(0x20, 0x0, Some(vec![(0x20, 0xAA), (0xAA, 0x8)]), None, None);

        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPage);

        Shift::lsr_memory(&opcode, &mut cpu);

        assert_registers!(cpu, 0, 0, 0);

        assert_memory_value!(&cpu.bus, 0xAA, 0x4);
    }
}
