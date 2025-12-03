use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

pub struct Load {}

impl Load {
    fn set_flags(cpu: &mut Mos6502, value: u8) {
        cpu.status.set_zero_flag(value);
        cpu.status.set_negative_flag(value);
    }

    // LDA - Load Accumulator
    pub fn lda(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.a = cpu.memory.read(address);
        Load::set_flags(cpu, cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // LDX - Load X Register
    pub fn ldx(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.x = cpu.memory.read(address);
        Load::set_flags(cpu, cpu.registers.x);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // LDY - Load Y Register
    pub fn ldy(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.y = cpu.memory.read(address);
        Load::set_flags(cpu, cpu.registers.y);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode, instruction_set::helpers::Helpers, memory::Memory, status::Flags,
    };

    fn create_cpu(memory_value: u8) -> Mos6502 {
        Mos6502 {
            memory: Memory::new_with_bytes(vec![(0xAA, memory_value)]),
            program_counter: 0xAA,
            ..Default::default()
        }
    }

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = create_cpu(0x05);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::lda(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }

    #[test]
    fn test_lda_zero_flag_set() {
        let mut cpu = create_cpu(0x00);
        cpu.memory.write(0xAA, 0x00);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::lda(&opcode, &mut cpu);

        assert_eq!(Flags::ZERO, cpu.status.flags);
    }

    #[test]
    fn test_ldx_loads_into_the_x_register() {
        let mut cpu = create_cpu(0x05);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldx(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.x);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }

    #[test]
    fn test_ldx_sets_negative_flag() {
        let mut cpu = create_cpu(0xFF);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldx(&opcode, &mut cpu);

        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn test_ldy_loads_into_the_y_register() {
        let mut cpu = create_cpu(0x05);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldy(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.y);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }

    #[test]
    fn test_ldy_sets_negative_flag() {
        let mut cpu = create_cpu(0xFF);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldy(&opcode, &mut cpu);

        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }
}
