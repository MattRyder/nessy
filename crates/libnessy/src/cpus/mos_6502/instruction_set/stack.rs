use crate::{
    cpus::mos_6502::{bus::MemoryAccess, cpu::Mos6502, status::Flags},
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
                InstructionResult::Ok
            }
            Err(result) => result,
        }
    }

    // PHP - Push Processor Status
    pub fn php(cpu: &mut Mos6502) -> InstructionResult {
        match Stack::push(cpu, cpu.status.bits()) {
            Err(err) => err,
            _ => InstructionResult::Ok,
        }
    }

    // PLP - Pull Processor Status
    pub fn plp(cpu: &mut Mos6502) -> InstructionResult {
        match Stack::pop(cpu) {
            Ok(v) => {
                cpu.status = Flags::from_bits_truncate(v);
                InstructionResult::Ok
            }
            Err(err) => err,
        }
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        bus::MemoryAccess,
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

    #[test]
    fn test_pla_pushes_to_stack() {
        // Test with a used stack.
        let mut cpu = Helpers::create_cpu(0x0, 0xFE, Some(vec![(STACK_TOP, 0xAA)]), None, None);

        assert_eq!(InstructionResult::Ok, Stack::pla(&mut cpu));

        assert_eq_hex!(0xFF, cpu.stack_pointer);

        assert_eq_hex!(0xAA, cpu.registers.a);
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
        let flags = Flags::CARRY | Flags::ZERO | Flags::NEGATIVE;

        // Test with a used stack.
        let mut cpu =
            Helpers::create_cpu(0x0, 0xFE, Some(vec![(STACK_TOP, 0xAA)]), None, Some(flags));

        assert_eq!(InstructionResult::Ok, Stack::php(&mut cpu));

        assert_eq_hex!(0xFD, cpu.stack_pointer);

        let stack_flags = Flags::from_bits_truncate(cpu.bus.read(0x01FE));
        assert_eq!(flags, stack_flags);
    }

    #[test]
    fn test_plp_pops_flags_from_stack() {
        // Test with a used stack.
        let mut cpu = Helpers::create_cpu(
            0x0,
            0xFE,
            Some(vec![(STACK_TOP, 0x83)]),
            None,
            Some(Flags::DECIMAL_MODE),
        );

        assert_eq!(InstructionResult::Ok, Stack::plp(&mut cpu));

        assert_eq_hex!(0xFF, cpu.stack_pointer);

        assert_eq!(Flags::CARRY | Flags::ZERO | Flags::NEGATIVE, cpu.status);
    }
}
