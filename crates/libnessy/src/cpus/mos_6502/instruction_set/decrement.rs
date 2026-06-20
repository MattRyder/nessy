use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        instruction_set::helpers::MSB_MASK,
        opcode::OpCode,
        status::Flags,
    },
    interpret_result::InstructionResult,
};

pub struct Decrement {}

impl Decrement {
    fn decrement(cpu: &mut Mos6502, operand: u8) -> u8 {
        let result = operand.wrapping_sub(1);

        cpu.status.set_status_flag(Flags::ZERO, result == 0);

        cpu.status
            .set_status_flag(Flags::NEGATIVE, result & MSB_MASK != 0);

        result
    }

    // DEC - Decrement Memory
    pub fn dec(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
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
        cpu.program_counter += opcode.bytes as u16;

        let memory_value = cpu.bus.read(address);

        let decrement_result = Decrement::decrement(cpu, memory_value);

        cpu.bus.write(address, decrement_result);

        InstructionResult::Ok
    }

    // DEX - Decrement X Register
    pub fn dex(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if opcode.address_mode.ne(&AddressMode::Implied) {
            return InstructionResult::IllegalInstruction;
        }

        let result = Decrement::decrement(cpu, cpu.registers.x);

        cpu.registers.x = result;

        InstructionResult::Ok
    }

    // DEY - Decrement Y Register
    pub fn dey(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if opcode.address_mode.ne(&AddressMode::Implied) {
            return InstructionResult::IllegalInstruction;
        }

        let result = Decrement::decrement(cpu, cpu.registers.y);

        cpu.registers.y = result;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use crate::{
        cpus::mos_6502::{
            address_mode::AddressMode,
            cpu::{Mos6502, Registers},
            instruction_set::{decrement::Decrement, helpers::Helpers},
            status::Flags,
        },
        interpret_result::InstructionResult,
    };

    #[test]
    fn test_dec_decrements_memory_value() {
        let mut cpu = Helpers::create_cpu(
            0xAA,
            0x0,
            Some(vec![(0xAA, 0x02), (0x02, 0x01)]),
            None,
            None,
        );

        let opcode = Helpers::create_opcode(2, AddressMode::ZeroPage);

        let result = Decrement::dec(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(0x00, cpu.bus.read(0x02));

        assert_eq!(Flags::ZERO, cpu.status);
    }

    #[test]
    fn test_dex_decrements_x_value_and_sets_zero_flag() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: 0x01,
                y: 0,
            },
            status: Flags::empty(),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Implied);

        let result = Decrement::dex(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(0x00, cpu.registers.x);

        assert_eq!(Flags::ZERO, cpu.status);
    }

    #[test]
    fn test_dey_decrements_y_value_and_sets_negative_flag() {
        let mut cpu = Mos6502 {
            registers: Registers { a: 0, x: 0, y: 0 },
            status: Flags::empty(),
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Implied);

        let result = Decrement::dey(&opcode, &mut cpu);

        assert_eq!(InstructionResult::Ok, result);

        assert_eq_hex!(0xFF, cpu.registers.y);

        assert_eq!(Flags::NEGATIVE, cpu.status);
    }
}
