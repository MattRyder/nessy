use crate::{
    cpus::mos_6502::{cpu::Mos6502, status::Flags},
    interpret_result::InstructionResult,
};

// SP points to the top (effectively 0xFF, which pages as 0x0100 + SP = 0x01FF) and grows down.
// Stack underflow occurs if the SP attempts to POP beyond this value.
// const STACK_TOP: u16 = 0x01FF;

// Stack overflow occurs if the SP attempts to PUSH beyond this value.
pub const STACK_BOTTOM: u16 = 0x0100;

pub struct Stack {}

// Push / Pull to/from the stack
impl Stack {
    pub fn push(cpu: &mut Mos6502, operand: u8) -> Result<(), InstructionResult> {
        let stack_address = STACK_BOTTOM + cpu.stack_pointer as u16;

        cpu.bus.write(stack_address, operand);

        if cpu.stack_pointer == 0 {
            Err(InstructionResult::StackOverflow)
        } else {
            cpu.stack_pointer -= 1;
            Ok(())
        }
    }

    pub fn pop(cpu: &mut Mos6502) -> Result<u8, InstructionResult> {
        if cpu.stack_pointer == 0xFF {
            return Err(InstructionResult::StackUnderflow);
        }

        let stack_address = STACK_BOTTOM + (cpu.stack_pointer as u16 + 1);

        let result = cpu.bus.read(stack_address);

        cpu.stack_pointer += 1;
        Ok(result)
    }

    // PHA - Push Accumulator to Stack
    pub fn pha(cpu: &mut Mos6502) -> InstructionResult {
        match Stack::push(cpu, cpu.registers.a) {
            Ok(_) => InstructionResult::Ok,
            Err(result) => result,
        }
    }

    // PLA - Pull Accumulator from Stack
    pub fn pla(cpu: &mut Mos6502) -> InstructionResult {
        match Stack::pop(cpu) {
            Ok(value) => {
                cpu.registers.a = value;

                cpu.status.set_zero_flag(cpu.registers.a);
                cpu.status.set_negative_flag(cpu.registers.a);

                InstructionResult::Ok
            }
            Err(result) => result,
        }
    }

    // PHP - Push Processor Status
    pub fn php(cpu: &mut Mos6502) -> InstructionResult {
        // Set BREAK_COMMAND before pushing P to the stack.
        match Stack::push(cpu, (cpu.status | Flags::BREAK_COMMAND).bits()) {
            Err(err) => err,
            _ => InstructionResult::Ok,
        }
    }

    // PLP - Pull Processor Status
    pub fn plp(cpu: &mut Mos6502) -> InstructionResult {
        match Stack::pop(cpu) {
            Ok(v) => {
                cpu.status = (Flags::from_bits_truncate(v) & !Flags::BREAK_COMMAND) | Flags::UNUSED;
                InstructionResult::Ok
            }
            Err(err) => err,
        }
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::{
        cpu::{Mos6502, Registers},
        instruction_set::helpers::Helpers,
        status::Flags,
    };

    const STACK_TOP: u16 = 0x01FF;

    #[test]
    fn test_pha_pops_stack_into_accumulator() {
        let registers = Registers {
            a: 0xDD,
            x: 0,
            y: 0,
        };

        let mut cpu = Helpers::create_cpu(
            0xAA,
            0xFE,
            Some(vec![(STACK_TOP, 0xAA)]),
            Some(registers),
            None,
        );

        assert_eq!(InstructionResult::Ok, Stack::pha(&mut cpu));

        assert_eq_hex!(0xFD, cpu.stack_pointer);

        assert_eq_hex!(0xDD, cpu.bus.read(STACK_TOP - 1));
    }

    #[test]
    fn test_pha_returns_overflow_given_full_stack() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0xDD,
                x: 0,
                y: 0,
            },
            stack_pointer: 0,
            ..Default::default()
        };

        assert_eq!(InstructionResult::StackOverflow, Stack::pha(&mut cpu));
    }

    #[parameterized]
    #[case(0x01, Flags::empty())]
    #[case(0x0, Flags::ZERO)]
    #[case(0x80, Flags::NEGATIVE)]
    fn test_pla_pushes_to_stack(value: u8, status: Flags) {
        // Test with a used stack.
        let mut cpu = Helpers::create_cpu(0x0, 0xFE, Some(vec![(STACK_TOP, value)]), None, None);

        assert_eq!(InstructionResult::Ok, Stack::pla(&mut cpu));

        assert_eq_hex!(0xFF, cpu.stack_pointer);

        assert_eq_hex!(value, cpu.registers.a);

        assert_eq_hex!(status, cpu.status);
    }

    #[test]
    fn test_pla_returns_underflow_given_empty_stack() {
        let mut cpu = Mos6502 {
            stack_pointer: 0xFF,
            ..Default::default()
        };

        assert_eq!(InstructionResult::StackUnderflow, Stack::pla(&mut cpu));
    }

    #[test]
    fn test_php_pushes_flags_to_stack() {
        // Flags being set on the CPU before PHP
        let flags = Flags::CARRY | Flags::ZERO | Flags::NEGATIVE;

        // PHP will ALWAYS set BREAK_COMMAND when it pushes P onto the stack.
        let expected_flags = flags | Flags::BREAK_COMMAND;

        // Test with a used stack.
        let mut cpu =
            Helpers::create_cpu(0x0, 0xFE, Some(vec![(STACK_TOP, 0xAA)]), None, Some(flags));

        assert_eq!(InstructionResult::Ok, Stack::php(&mut cpu));

        assert_eq_hex!(0xFD, cpu.stack_pointer);

        let stack_flags = Flags::from_bits_truncate(cpu.bus.read(0x01FE));

        //
        assert_eq!(expected_flags, stack_flags);
    }

    #[parameterized]
    #[case(0x83, Flags::CARRY | Flags::ZERO | Flags::NEGATIVE | Flags::UNUSED)]
    #[case(0xFF, Flags::all() & !Flags::BREAK_COMMAND)]
    fn test_plp_pops_flags_from_stack(value: u8, flags: Flags) {
        // Test with a used stack.
        let mut cpu = Helpers::create_cpu(
            0x0,
            0xFE,
            Some(vec![(STACK_TOP, value)]),
            None,
            Some(Flags::DECIMAL_MODE),
        );

        assert_eq!(InstructionResult::Ok, Stack::plp(&mut cpu));

        assert_eq_hex!(0xFF, cpu.stack_pointer);

        assert_eq!(flags, cpu.status);
    }
}
