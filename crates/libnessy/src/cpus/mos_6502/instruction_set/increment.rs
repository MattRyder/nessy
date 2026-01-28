use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        instruction_set::helpers::MSB_MASK,
        memory::MemoryAccess,
        opcode::OpCode,
        status::Flags,
    },
    interpret_result::InstructionResult,
};

pub struct Increment {}

impl Increment {
    fn increment(cpu: &mut Mos6502, operand: u8) -> u8 {
        let result = operand.wrapping_add(1);

        cpu.status.set_status_flag(Flags::ZERO, result == 0);
        cpu.status
            .set_status_flag(Flags::NEGATIVE, result & MSB_MASK != 0);

        result
    }

    // INC - Increment Memory
    pub fn inc(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if ![
            AddressMode::ZeroPage,
            AddressMode::ZeroPageX,
            AddressMode::Absolute,
            AddressMode::AbsoluteX,
        ]
        .contains(&opcode.address_mode)
        {
            return InstructionResult::IllegalInstruction;
        }

        let address = cpu.get_address(&opcode.address_mode);
        let memory_value = cpu.bus.read(address);
        cpu.program_counter += opcode.bytes as u16;

        let result = Increment::increment(cpu, memory_value);

        cpu.bus.write(address, result);

        InstructionResult::Ok
    }

    // INX - Increment X Register
    pub fn inx(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.x = Increment::increment(cpu, cpu.registers.x);

        InstructionResult::Ok
    }

    // INY - Increment Y Register
    pub fn iny(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.y = Increment::increment(cpu, cpu.registers.y);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;

    use crate::cpus::mos_6502::{cpu::Mos6502, instruction_set::helpers::Helpers};

    #[test]
    fn test_inc_increments_memory_value() {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x55), (0x55, 0x01)]),
            None,
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Increment::inc(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(0x02, cpu.bus.read(0x55));

        assert_eq!(Flags::empty(), cpu.status);
    }

    #[parameterized]
    #[case(0xFF, 0, Flags::ZERO)]
    #[case(0x7F, 0x80, Flags::NEGATIVE)]
    fn test_inx(x_reg: u8, expected_x_reg: u8, expected_flags: Flags) {
        let mut cpu = Mos6502::default();

        cpu.registers.x = x_reg;

        Increment::inx(&mut cpu);

        assert_eq_hex!(expected_x_reg, cpu.registers.x);
        assert_eq!(expected_flags, cpu.status);
    }

    #[parameterized]
    #[case(0xFF, 0, Flags::ZERO)]
    #[case(0x7F, 0x80, Flags::NEGATIVE)]
    fn test_iny(y_reg: u8, expected_y_reg: u8, expected_flags: Flags) {
        let mut cpu = Mos6502::default();

        cpu.registers.y = y_reg;

        Increment::iny(&mut cpu);

        assert_eq_hex!(expected_y_reg, cpu.registers.y);
        assert_eq!(expected_flags, cpu.status);
    }
}
