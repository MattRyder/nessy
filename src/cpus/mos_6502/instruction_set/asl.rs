use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

const MSB_MASK: u8 = 0b1000_0000;

// ASL - Arithmetic Shift Left
pub struct Asl {}

impl Asl {
    fn shift_and_set_flags(cpu: &mut Mos6502, value: u8) -> u8 {
        let shifted_result = value << 1;

        cpu.status.set_carry_flag(value & MSB_MASK != 0);
        cpu.status.set_flags_for_result(shifted_result);

        shifted_result & 0xFF
    }

    pub fn asl_accumulator(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = Asl::shift_and_set_flags(cpu, cpu.registers.a);

        InstructionResult::Ok
    }

    pub fn asl_memory(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let address_value = cpu.memory.read(address);

        let shifted_result = Asl::shift_and_set_flags(cpu, address_value);

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

        Asl::asl_accumulator(&mut cpu);

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

        Asl::asl_memory(&opcode, &mut cpu);

        assert_registers!(cpu, 0, 0, 0);

        assert_memory_value!(&cpu.memory, 0xAA, 0x10);
    }
}
