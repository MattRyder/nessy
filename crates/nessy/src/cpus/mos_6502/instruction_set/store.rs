use crate::{
    cpus::mos_6502::{address_mode::MemoryAddressing, cpu::Mos6502, opcode::OpCode},
    interpret_result::InstructionResult,
};

pub struct Store {}

impl Store {
    // STA - Store Accumulator
    pub fn sta(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.bus.write(address, cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // STX - Store X Register
    pub fn stx(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.bus.write(address, cpu.registers.x);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // STY - Store Y Register
    pub fn sty(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.bus.write(address, cpu.registers.y);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // SAX - AND X register with accumulator and store result in memory. [undocumented]
    pub fn sax(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);
        cpu.program_counter += opcode.bytes as u16;

        let value = cpu.registers.a & cpu.registers.x;
        cpu.bus.write(address, value);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::Registers,
        instruction_set::{helpers::Helpers, store::Store},
        status::Flags,
    };

    #[test]
    fn test_sta_writes_accumulator_to_memory() {
        let mut cpu = Helpers::create_cpu(
            0x0,
            0x0,
            Some(vec![(0x0, 0x01)]),
            Some(Registers {
                a: 0x22,
                x: 0,
                y: 0,
            }),
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        Store::sta(&opcode, &mut cpu);

        assert_eq!(0x22, cpu.bus.read(0x01))
    }

    #[test]
    fn test_stx_writes_register_x_to_memory() {
        let mut cpu = Helpers::create_cpu(
            0x0,
            0x0,
            Some(vec![(0x0, 0x01)]),
            Some(Registers {
                a: 0x0,
                x: 0x22,
                y: 0,
            }),
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        Store::stx(&opcode, &mut cpu);

        assert_eq!(0x22, cpu.bus.read(0x01))
    }

    #[test]
    fn test_sty_writes_register_y_to_memory() {
        let mut cpu = Helpers::create_cpu(
            0x0,
            0x0,
            Some(vec![(0x0, 0x01)]),
            Some(Registers {
                a: 0x0,
                x: 0,
                y: 0x22,
            }),
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        Store::sty(&opcode, &mut cpu);

        assert_eq!(0x22, cpu.bus.read(0x01))
    }

    #[parameterized]
    #[case(vec![(0xAA, 0x41)], Registers { a: 0x05, x: 0x0F, y: 0}, None, 0x05, Flags::empty())]
    #[case(vec![(0xAA, 0x41)], Registers { a: 0x3E, x: 0x17, y: 0x44}, Some(Flags::from_bits_truncate(0xE6)), 0x16, Flags::from_bits_truncate(0xE6))]
    fn test_sax(
        memory: Vec<(u16, u8)>,
        registers: Registers,
        status: Option<Flags>,
        expected_value: u8,
        expected_flags: Flags,
    ) {
        let mut cpu = Helpers::create_cpu(0xAA, 0x0, Some(memory), Some(registers), status);

        let opcode = Helpers::create_opcode(1, AddressMode::Immediate);

        Store::sax(&opcode, &mut cpu);

        assert_eq_hex!(expected_value, cpu.bus.read(0xAA));
        assert_eq!(expected_flags, cpu.status);
    }
}
