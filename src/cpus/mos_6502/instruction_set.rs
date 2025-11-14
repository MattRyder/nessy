use crate::{
    cpus::mos_6502::{address_mode::MemoryAddressing, memory::MemoryAccess, opcode::OpCode},
    interpret_result::InstructionResult,
};

use super::cpu::Mos6502;

pub struct InstructionSet {}

impl InstructionSet {
    pub fn brk() -> InstructionResult {
        InstructionResult::EndProgram
    }

    // LDA - Load Accumulator
    pub fn lda(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.a = cpu.memory.read(address);
        cpu.status.set_flags_for_result(cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // TAX - Transfer Accumulator to X
    pub fn tax(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.x = cpu.registers.a;
        cpu.status.set_flags_for_result(cpu.registers.x);

        InstructionResult::Ok
    }

    // INX - Increment X Register
    pub fn inx(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.registers.x < 0xFF {
            cpu.registers.x += 1;
        } else {
            cpu.registers.x = 0;
        }

        cpu.status.set_flags_for_result(cpu.registers.x);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        memory::{MEMORY_SIZE, Memory},
        status::Flags,
    };

    fn create_opcode(bytes: u8, address_mode: AddressMode) -> OpCode {
        OpCode::new(0xFF, "TEST", bytes, 1, address_mode, |_, _| {
            InstructionResult::Ok
        })
    }

    #[test]
    fn test_lda_immediate_load_data() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0x05;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            ..Default::default()
        };

        let opcode = create_opcode(1, AddressMode::Immediate);

        InstructionSet::lda(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }

    #[test]
    fn test_lda_zero_flag_set() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0x00;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            ..Default::default()
        };

        let opcode = create_opcode(1, AddressMode::Immediate);

        InstructionSet::lda(&opcode, &mut cpu);

        assert_eq!(Flags::ZERO, cpu.status.flags & Flags::ZERO);
    }

    #[test]
    fn test_tax_copies_accumulator_to_x_register() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0x05;

        InstructionSet::tax(&mut cpu);

        assert_eq_hex!(0x05, cpu.registers.x);
        assert_eq!(
            Flags::empty(),
            cpu.status.flags & (Flags::ZERO | Flags::NEGATIVE)
        );
    }

    #[test]
    fn test_tax_zero_flag_set() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        InstructionSet::tax(&mut cpu);

        assert_eq_hex!(0x00, cpu.registers.x);
        assert_eq!(Flags::ZERO, cpu.status.flags);
    }

    #[test]
    fn test_tax_negative_flag_set() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0xF0;

        InstructionSet::tax(&mut cpu);

        assert_eq_hex!(0xF0, cpu.registers.x);
        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn test_inx_overflows() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0xFF;

        InstructionSet::inx(&mut cpu);

        assert_eq_hex!(cpu.registers.x, 1)
    }
}
