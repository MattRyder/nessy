use crate::{
    cpus::mos_6502::{address_mode::MemoryAddressing, cpu::Mos6502, opcode::OpCode},
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

        cpu.registers.a = cpu.bus.read(address);
        Load::set_flags(cpu, cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // LDX - Load X Register
    pub fn ldx(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.x = cpu.bus.read(address);
        Load::set_flags(cpu, cpu.registers.x);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // LDY - Load Y Register
    pub fn ldy(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.registers.y = cpu.bus.read(address);
        Load::set_flags(cpu, cpu.registers.y);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // LAX - Load accumulator and X register with memory. [undocumented]
    pub fn lax(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        let value = cpu.bus.read(address);
        cpu.registers.a = value;
        cpu.registers.x = value;

        Load::set_flags(cpu, value);
        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::{
        address_mode::AddressMode, bus::Bus, instruction_set::helpers::Helpers, status::Flags,
    };

    fn create_cpu(memory_value: u8) -> Mos6502 {
        let mut cpu = Mos6502 {
            bus: Box::new(Bus::default()),
            program_counter: 0xAA,
            status: Flags::empty(),
            ..Default::default()
        };

        cpu.bus.write(0xAA, memory_value);

        cpu
    }

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = create_cpu(0x05);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::lda(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status & Flags::NEGATIVE);
    }

    #[test]
    fn test_lda_zero_flag_set() {
        let mut cpu = create_cpu(0x00);
        cpu.bus.write(0xAA, 0x00);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::lda(&opcode, &mut cpu);

        assert_eq!(Flags::ZERO, cpu.status);
    }

    #[test]
    fn test_ldx_loads_into_the_x_register() {
        let mut cpu = create_cpu(0x05);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldx(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.x);
        assert_eq!(Flags::empty(), cpu.status & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status & Flags::NEGATIVE);
    }

    #[test]
    fn test_ldx_sets_negative_flag() {
        let mut cpu = create_cpu(0xFF);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldx(&opcode, &mut cpu);

        assert_eq!(Flags::NEGATIVE, cpu.status);
    }

    #[test]
    fn test_ldy_loads_into_the_y_register() {
        let mut cpu = create_cpu(0x05);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldy(&opcode, &mut cpu);

        assert_eq_hex!(0x05, cpu.registers.y);
        assert_eq!(Flags::empty(), cpu.status & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status & Flags::NEGATIVE);
    }

    #[test]
    fn test_ldy_sets_negative_flag() {
        let mut cpu = create_cpu(0xFF);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Load::ldy(&opcode, &mut cpu);

        assert_eq!(Flags::NEGATIVE, cpu.status);
    }

    #[parameterized]
    #[case(0x00, Flags::ZERO)]
    #[case(0x05, Flags::empty())]
    #[case(0x80, Flags::NEGATIVE)]
    #[case(0xFF, Flags::NEGATIVE)]
    fn test_lax(expected_register_value: u8, expected_status: Flags) {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x55), (0x55, expected_register_value)]),
            None,
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        assert_eq!(InstructionResult::Ok, Load::lax(&opcode, &mut cpu));

        assert_eq_hex!(expected_register_value, cpu.registers.a);
        assert_eq_hex!(expected_register_value, cpu.registers.x);

        assert_eq!(expected_status, cpu.status);
    }
}
