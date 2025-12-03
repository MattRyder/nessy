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
        let address_value = cpu.memory.read(address);

        let shifted_result = Shift::arithmetic_shift(cpu, address_value);

        cpu.memory.write(address, shifted_result);

        InstructionResult::Ok
    }

    // LSR - Logical Shift Right
    pub fn lsr_accumulator(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = Shift::logical_shift(cpu, cpu.registers.a);

        InstructionResult::Ok
    }

    pub fn lsr_memory(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let address_value = cpu.memory.read(address);

        let shifted_result = Shift::logical_shift(cpu, address_value);

        cpu.memory.write(address, shifted_result);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        assert_memory_value, assert_registers,
        cpus::mos_6502::{
            address_mode::AddressMode,
            cpu::{Mos6502, Registers},
            instruction_set::helpers::Helpers,
            memory::Memory,
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
        assert_eq!(Flags::empty(), cpu.status.flags);
    }

    #[test]
    fn test_asl_with_memory_does_bitwise_shift() {
        let mut cpu = Mos6502 {
            program_counter: 0x20,
            memory: Memory::new_with_bytes(vec![(0x20, 0xAA), (0xAA, 0x8)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPage);

        Shift::asl_memory(&opcode, &mut cpu);

        assert_registers!(cpu, 0, 0, 0);

        assert_memory_value!(&cpu.memory, 0xAA, 0x10);
    }

    #[test]
    fn test_lsr_with_accumulator_does_bitwise_shift() {
        let mut cpu = Mos6502 {
            registers: Registers { a: 0x2, x: 0, y: 0 },
            program_counter: 0xAA,
            ..Default::default()
        };

        Shift::lsr_accumulator(&mut cpu);

        assert_registers!(cpu, 0x01, 0, 0);
        assert_eq!(Flags::empty(), cpu.status.flags);
    }

    #[test]
    fn test_lsr_with_accumulator_sets_carry_flag() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0xFF,
                x: 0,
                y: 0,
            },
            program_counter: 0xAA,
            ..Default::default()
        };

        Shift::lsr_accumulator(&mut cpu);

        assert_registers!(cpu, 0x7F, 0, 0);
        assert_eq!(Flags::CARRY, cpu.status.flags);
    }

    #[test]
    fn test_lsr_with_memory_does_bitwise_shift() {
        let mut cpu = Mos6502 {
            program_counter: 0x20,
            memory: Memory::new_with_bytes(vec![(0x20, 0xAA), (0xAA, 0x8)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPage);

        Shift::lsr_memory(&opcode, &mut cpu);

        assert_registers!(cpu, 0, 0, 0);

        assert_memory_value!(&cpu.memory, 0xAA, 0x4);
    }
}
