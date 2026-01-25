use crate::{
    cpus::mos_6502::{cpu::Mos6502, instruction_set::stack::Stack, status::Flags},
    interpret_result::InstructionResult,
};

const INTERRUPT_VECTOR: u16 = 0xFFFE;

// The remaining instructions perform useful but rarely used functions.
pub struct System {}

impl System {
    // BRK - Force Interrupt
    pub fn brk(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status |= Flags::BREAK_COMMAND;
        cpu.program_counter = INTERRUPT_VECTOR;
        InstructionResult::EndProgram
    }

    // NOP - No Operation
    pub fn nop() -> InstructionResult {
        InstructionResult::Ok
    }

    // RTI - Return from Interrupt
    pub fn rti(cpu: &mut Mos6502) -> InstructionResult {
        let flags = match Stack::pop(cpu) {
            Ok(v) => v,
            Err(err) => return err,
        };

        let lo = match Stack::pop(cpu) {
            Ok(v) => v,
            Err(err) => return err,
        };

        let hi = match Stack::pop(cpu) {
            Ok(v) => v,
            Err(err) => return err,
        };

        cpu.program_counter = u16::from(lo) | (u16::from(hi) << 8);

        cpu.status = Flags::from_bits_truncate(flags);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;

    use crate::{
        cpus::mos_6502::{cpu::Mos6502, memory::Memory},
        interpret_result::InstructionResult,
    };

    use super::*;

    #[test]
    fn test_brk_returns_end_program() {
        let mut cpu = Mos6502::default();

        assert_eq!(InstructionResult::EndProgram, System::brk(&mut cpu));
        assert_eq_hex!(INTERRUPT_VECTOR, cpu.program_counter);
        assert_eq_hex!(Flags::BREAK_COMMAND, cpu.status);
    }

    #[test]
    fn test_nop_does_nowt() {
        assert_eq!(InstructionResult::Ok, System::nop());
    }

    #[test]
    fn test_rti_sets_flags_and_pc() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![(0x01FF, 0xAA), (0x01FE, 0xBB), (0x01FD, 0x83)]),
            stack_pointer: 0xFC,
            ..Default::default()
        };

        assert_eq!(InstructionResult::Ok, System::rti(&mut cpu));

        assert_eq!(
            Flags::CARRY | Flags::NEGATIVE | Flags::ZERO,
            cpu.status
        );

        assert_eq_hex!(0xAABB, cpu.program_counter);

        assert_eq_hex!(0xFF, cpu.stack_pointer);
    }
}
