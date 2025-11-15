use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

// LDA - Load Accumulator
pub struct Lda {}

impl Lda {
    pub fn lda(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.a = cpu.memory.read(address);
        cpu.status.set_flags_for_result(cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        instruction_set::helpers::Helpers,
        memory::{MEMORY_SIZE, Memory},
        status::Flags,
    };

    #[test]
    fn test_lda_immediate_load_data() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0x05;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Lda::lda(&opcode, &mut cpu);

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

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Lda::lda(&opcode, &mut cpu);

        assert_eq!(Flags::ZERO, cpu.status.flags & Flags::ZERO);
    }
}
