use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, bus::MemoryAccess, cpu::Mos6502, opcode::OpCode,
    },
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
}

#[cfg(test)]
mod test {
    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        bus::MemoryAccess,
        cpu::Registers,
        instruction_set::{helpers::Helpers, store::Store},
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
}
