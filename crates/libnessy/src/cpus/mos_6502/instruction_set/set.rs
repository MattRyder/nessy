use crate::{
    cpus::mos_6502::{cpu::Mos6502, status::Flags},
    interpret_result::InstructionResult,
};

pub struct Set {}

impl Set {
    // SEC - Set Carry Flag
    pub fn sec(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.set_status_flag(Flags::CARRY, true);
        InstructionResult::Ok
    }

    // SED - Set Decimal Flag
    pub fn sed(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.set_status_flag(Flags::DECIMAL_MODE, true);
        InstructionResult::Ok
    }

    // SEI - Set Interrupt Disable
    pub fn sei(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.set_status_flag(Flags::INTERRUPT_DISABLE, true);
        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cpus::mos_6502::cpu::Mos6502;

    #[test]
    fn test_sec_sets_carry_flag() {
        let mut cpu = Mos6502::default();

        let result = Set::sec(&mut cpu);

        assert_eq!(InstructionResult::Ok, result);
        assert_eq!(Flags::CARRY, cpu.status);
    }

    #[test]
    fn test_sed_sets_decimal_mode_flag() {
        let mut cpu = Mos6502::default();

        let result = Set::sed(&mut cpu);

        assert_eq!(InstructionResult::Ok, result);
        assert_eq!(Flags::DECIMAL_MODE, cpu.status);
    }

    #[test]
    fn test_sei_sets_interrupt_disable_flag() {
        let mut cpu = Mos6502::default();

        let result = Set::sei(&mut cpu);

        assert_eq!(InstructionResult::Ok, result);
        assert_eq!(Flags::INTERRUPT_DISABLE, cpu.status);
    }
}
