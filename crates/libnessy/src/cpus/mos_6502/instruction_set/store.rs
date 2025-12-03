use crate::{
    cpus::mos_6502::{
        address_mode::MemoryAddressing, cpu::Mos6502, memory::MemoryAccess, opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

pub struct Store {}

impl Store {
    // STA - Store Accumulator
    pub fn sta(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.memory.write(address, cpu.registers.a);

        InstructionResult::Ok
    }

    // STX - Store X Register
    pub fn stx(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.memory.write(address, cpu.registers.x);

        InstructionResult::Ok
    }

    // STY - Store Y Register
    pub fn sty(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        cpu.memory.write(address, cpu.registers.y);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use crate::cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::{Mos6502, Registers},
        instruction_set::{helpers::Helpers, store::Store},
        memory::{Memory, MemoryAccess},
    };

    #[test]
    fn test_sta_writes_accumulator_to_memory() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0x22,
                x: 0,
                y: 0,
            },
            program_counter: 0,
            memory: Memory::new_with_bytes(vec![(0x0, 0x01)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        Store::sta(&opcode, &mut cpu);

        assert_eq!(0x22, cpu.memory.read(0x01))
    }

    #[test]
    fn test_stx_writes_register_x_to_memory() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: 0x22,
                y: 0,
            },
            program_counter: 0,
            memory: Memory::new_with_bytes(vec![(0x0, 0x01)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        Store::stx(&opcode, &mut cpu);

        assert_eq!(0x22, cpu.memory.read(0x01))
    }

    #[test]
    fn test_sty_writes_register_y_to_memory() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0x22,
            },
            program_counter: 0,
            memory: Memory::new_with_bytes(vec![(0x0, 0x01)]),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        Store::sty(&opcode, &mut cpu);

        assert_eq!(0x22, cpu.memory.read(0x01))
    }
}
