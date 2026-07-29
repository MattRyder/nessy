use crate::{
    cpus::mos_6502::{cpu::Mos6502, status::Flags},
    interpret_result::InstructionResult,
};

pub struct Clear {}

impl Clear {
    // CLC - Clear Carry Flag
    pub fn clc(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.remove(Flags::CARRY);
        InstructionResult::Ok
    }

    // CLD - Clear Decimal Mode
    pub fn cld(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.remove(Flags::DECIMAL_MODE);
        InstructionResult::Ok
    }

    // CLI - Clear Interrupt Disable
    pub fn cli(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.remove(Flags::INTERRUPT_DISABLE);
        InstructionResult::Ok
    }

    // CLV - Clear Overflow Flag
    pub fn clv(cpu: &mut Mos6502) -> InstructionResult {
        cpu.status.remove(Flags::OVERFLOW);
        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cpus::mos_6502::status::Flags;
    use assert_hex::assert_eq_hex;

    // Set the expected flag, and a canary flag to make sure
    // that we're not just doing a full clear.
    macro_rules! assert_flag_clear {
        ($clear_flag:expr, $canary_flag:expr, $func:expr) => {
            let mut cpu = Mos6502 {
                status: $clear_flag | $canary_flag,
                ..Default::default()
            };

            assert_eq!(InstructionResult::Ok, ($func)(&mut cpu));

            assert_eq_hex!($canary_flag, cpu.status);
        };
    }

    #[test]
    fn test_clc_clears() {
        assert_flag_clear!(Flags::CARRY, Flags::OVERFLOW, Clear::clc);
    }

    #[test]
    fn test_cld_clears() {
        assert_flag_clear!(Flags::DECIMAL_MODE, Flags::CARRY, Clear::cld);
    }

    #[test]
    fn test_cli_clears() {
        assert_flag_clear!(Flags::INTERRUPT_DISABLE, Flags::DECIMAL_MODE, Clear::cli);
    }

    #[test]
    fn test_clv_clears() {
        assert_flag_clear!(Flags::OVERFLOW, Flags::BREAK_COMMAND, Clear::clv);
    }
}
