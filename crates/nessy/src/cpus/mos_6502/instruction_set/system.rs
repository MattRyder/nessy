use crate::{
    cpus::mos_6502::{cpu::Mos6502, instruction_set::stack::Stack, opcode::OpCode, status::Flags},
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
    // Also used for undocumented versions of NOP
    // - DOP: Double NOP
    // - TOP: Triple NOP
    pub fn nop(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        // burn the opcode's byte, if it's present.
        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    // RTI - Return from Interrupt
    // The RTI instruction is used at the end of an interrupt processing routine.
    // It pulls the processor flags from the stack followed by the program counter.
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

        cpu.status = Flags::from_bits_truncate(flags) | Flags::UNUSED;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use crate::{
        cpus::mos_6502::{address_mode::AddressMode, instruction_set::helpers::Helpers},
        interpret_result::InstructionResult,
    };

    use super::*;

    #[test]
    fn test_brk_returns_end_program() {
        let mut cpu = Helpers::create_cpu(0x0, 0x0, None, None, None);

        assert_eq!(InstructionResult::EndProgram, System::brk(&mut cpu));
        assert_eq_hex!(INTERRUPT_VECTOR, cpu.program_counter);
        assert_eq_hex!(Flags::BREAK_COMMAND, cpu.status);
    }

    #[parameterized]
    #[case(1, 0x05)]
    #[case(2, 0x06)]
    #[case(3, 0x07)]
    fn test_nop_does_nowt(bytes: u8, expected_pc: u16) {
        let opcode = Helpers::create_opcode(bytes, AddressMode::Immediate);
        let mut cpu = Helpers::create_cpu(0x05, 0x0, None, None, None);

        assert_eq!(InstructionResult::Ok, System::nop(&opcode, &mut cpu));

        assert_eq_hex!(expected_pc, cpu.program_counter);
    }

    #[test]
    fn test_rti_sets_flags_and_pc() {
        let mut cpu = Helpers::create_cpu(
            0x0,
            0xFC,
            Some(vec![(0x01FF, 0xAA), (0x01FE, 0xBB), (0x01FD, 0x83)]),
            None,
            None,
        );

        assert_eq!(InstructionResult::Ok, System::rti(&mut cpu));

        assert_eq!(
            Flags::CARRY | Flags::NEGATIVE | Flags::UNUSED | Flags::ZERO,
            cpu.status
        );

        assert_eq_hex!(0xAABB, cpu.program_counter);

        assert_eq_hex!(0xFF, cpu.stack_pointer);
    }
}
