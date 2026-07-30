use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing,
        cpu::Mos6502,
        instruction_set::{helpers::MSB_MASK, rotate::Rotate},
        opcode::OpCode,
        status::Flags,
    },
    interpret_result::InstructionResult,
};

pub struct Arithmetic {}

// The arithmetic operations perform addition and subtraction on the contents of the accumulator.
impl Arithmetic {
    fn sbc_impl(cpu: &mut Mos6502, operand: u8, memory_value: u8) -> u8 {
        // 1 = no borrow
        // 0 = borrow
        let carry = if cpu.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        // 6502 hw will do A + (~M) + C
        let not_m = !memory_value;

        let result = operand.wrapping_add(not_m).wrapping_add(carry);

        // cpu.registers.a = result;

        let carry_set = (operand as u16) + (not_m as u16) + (carry as u16) > 0xFF;
        cpu.status.set_status_flag(Flags::CARRY, carry_set);

        cpu.status.set_zero_flag(result);

        cpu.status.set_negative_flag(result);

        let overflow_set = ((operand ^ result) & (operand ^ memory_value) & MSB_MASK) != 0;

        cpu.status.set_status_flag(Flags::OVERFLOW, overflow_set);

        result
    }

    fn adc_impl(cpu: &mut Mos6502, operand: u8, memory_value: u8) -> u8 {
        // 1 = no borrow
        // 0 = borrow
        let carry = if cpu.status.contains(Flags::CARRY) {
            1
        } else {
            0
        };

        let result = operand.wrapping_add(memory_value).wrapping_add(carry);

        cpu.registers.a = result;

        let carry_set = (operand as u16) + (memory_value as u16) + (carry as u16) > 0xFF;
        cpu.status.set_status_flag(Flags::CARRY, carry_set);

        cpu.status.set_zero_flag(result);

        cpu.status.set_negative_flag(result);

        let overflow_set = (!(operand ^ memory_value) & (operand ^ result) & MSB_MASK) != 0;

        cpu.status.set_status_flag(Flags::OVERFLOW, overflow_set);

        result
    }

    // ADC - Add with Carry
    pub fn adc(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        let m = cpu.bus.read(address);

        cpu.program_counter += opcode.bytes as u16;

        cpu.registers.a = Arithmetic::adc_impl(cpu, cpu.registers.a, m);

        InstructionResult::Ok
    }

    // SBC - Subtract with Carry
    // This was a hard one to get my head around, but one thing to remember:
    // SBC requires CARRY set if you want a standard `A - M` subtraction.
    // Otherwise it does a borrow based on the empty carry, which makes it `A - M - 1`
    // and you find that you've got an extra -1 on the result.
    pub fn sbc(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.program_counter += opcode.bytes as u16;

        let m = cpu.bus.read(address);

        cpu.registers.a = Arithmetic::sbc_impl(cpu, cpu.registers.a, m);

        InstructionResult::Ok
    }

    // ISB - Increase memory by one, then subtract memory from accu-mulator (with borrow).
    // [undocumented]
    pub fn isb(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.program_counter += opcode.bytes as u16;

        let memory_value = cpu.bus.read(address);

        let result = memory_value.wrapping_add(1);

        cpu.bus.write(address, result);

        cpu.registers.a = Arithmetic::sbc_impl(cpu, cpu.registers.a, result);

        InstructionResult::Ok
    }

    // Rotate one bit right in memory, then add memory to accumulator (with carry).
    pub fn rra(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.program_counter += opcode.bytes as u16;

        let memory_value = cpu.bus.read(address);

        let result = Rotate::rotate_right(cpu, memory_value);

        cpu.bus.write(address, result);

        cpu.registers.a = Arithmetic::adc_impl(cpu, cpu.registers.a, result);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use crate::{
        cpus::mos_6502::{
            address_mode::AddressMode, cpu::Registers, instruction_set::helpers::Helpers,
        },
        interpret_result::InstructionResult,
    };

    use super::*;

    #[parameterized]
    #[case(0x02, 0x03, Flags::empty(), 0x01, 0x04, Flags::empty())]
    fn test_rra(
        accumulator: u8,
        memory_value: u8,
        flags: Flags,
        expected_memory_value: u8,
        expected_accumulator: u8,
        expected_flags: Flags,
    ) {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, memory_value)]),
            Some(Registers {
                a: accumulator,
                x: 0,
                y: 0,
            }),
            Some(flags),
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        assert_eq!(InstructionResult::Ok, Arithmetic::rra(&opcode, &mut cpu));

        assert_eq_hex!(expected_memory_value, cpu.bus.read(0x02));

        assert_eq_hex!(expected_accumulator, cpu.registers.a);
        assert_eq!(expected_flags, cpu.status);
    }

    #[parameterized]
    #[case(Flags::CARRY, 0x0, Flags::ZERO | Flags::CARRY)]
    #[case(Flags::empty(), 0xFF, Flags::NEGATIVE)]
    fn test_sbc_works_given_flags(
        cpu_flags: Flags,
        expected_accumulator: u8,
        expected_flags: Flags,
    ) {
        let registers = Registers {
            a: 0x01,
            x: 0,
            y: 0,
        };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, 0x01)]),
            Some(registers),
            Some(cpu_flags),
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Arithmetic::sbc(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected_accumulator, cpu.registers.a);

        assert_eq!(expected_flags, cpu.status);
    }

    #[parameterized]
    #[case(Flags::empty(), 0x01, 0x01, 0x02, Flags::empty())]
    #[case(Flags::CARRY, 0x01, 0x01, 0x03, Flags::empty())]
    #[case(Flags::empty(), 0x00, 0x00, 0x00, Flags::ZERO)]
    #[case(Flags::empty(), 0xFF, 0x01, 0x00, Flags::CARRY | Flags::ZERO)]
    #[case(Flags::CARRY, 0xFF, 0x01, 0x01, Flags::CARRY)]
    #[case(Flags::CARRY, 0x7F, 0x7F, 0xFF, Flags::OVERFLOW | Flags::NEGATIVE)]
    fn test_adc_works_given_flags(
        cpu_flags: Flags,
        accumulator: u8,
        memory_value: u8,
        expected_accumulator: u8,
        expected_flags: Flags,
    ) {
        let registers = Registers {
            a: accumulator,
            x: 0,
            y: 0,
        };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, memory_value)]),
            Some(registers),
            Some(cpu_flags),
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Arithmetic::adc(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(expected_accumulator, cpu.registers.a);

        assert_eq!(expected_flags, cpu.status);
    }

    #[parameterized]
    #[case(0x01, Flags::empty(), 0x2, 0x2, Flags::CARRY)]
    #[case(0x04, Flags::CARRY, 0x5, 0x0, Flags::CARRY | Flags::ZERO)]
    #[case(0x05, Flags::empty(), 0x6, 0xFE, Flags::NEGATIVE)]
    fn test_isb_works_given_flags(
        memory_value: u8,
        cpu_flags: Flags,
        expected_new_memory_value: u8,
        expected_accumulator: u8,
        expected_flags: Flags,
    ) {
        let registers = Registers {
            a: 0x05,
            x: 0,
            y: 0,
        };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, memory_value)]),
            Some(registers),
            Some(cpu_flags),
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Arithmetic::isb(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq!(expected_new_memory_value, cpu.bus.read(0x02));

        assert_eq_hex!(expected_accumulator, cpu.registers.a);

        assert_eq!(expected_flags, cpu.status);
    }
}
