use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

pub struct Ora {}

impl Ora {
    // ORA - Logical Inclusive OR
    pub fn ora(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.memory.read(address);

        cpu.registers.a |= memory_value;
        cpu.status.set_flags_for_result(cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_hex::assert_eq_hex;

    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::Mos6502,
        instruction_set::helpers::Helpers,
        memory::{MEMORY_SIZE, Memory},
        status::Flags,
    };

    #[test]
    fn test_ora_does_a_bitwise_or() {
        let mut bytes = [0u8; MEMORY_SIZE];
        bytes[0xAA] = 0x32;

        let mut cpu = Mos6502 {
            memory: Memory::new(bytes),
            program_counter: 0xAA,
            ..Default::default()
        };

        cpu.registers.a = 0x19;

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Ora::ora(&opcode, &mut cpu);

        assert_eq_hex!(0x3B, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }
}
