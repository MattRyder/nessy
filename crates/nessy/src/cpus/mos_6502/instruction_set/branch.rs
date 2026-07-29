use crate::{
    cpus::mos_6502::{cpu::Mos6502, status::Flags},
    interpret_result::InstructionResult,
};

pub struct Branch {}

impl Branch {
    fn branch_rule<F>(cpu: &mut Mos6502, predicate: F) -> InstructionResult
    where
        F: Fn(&mut Mos6502) -> bool,
    {
        let offset = cpu.bus.read(cpu.program_counter) as i8;
        cpu.program_counter += 1;

        if predicate(cpu) {
            cpu.program_counter = cpu.program_counter.wrapping_add(offset as i16 as u16);
        }

        InstructionResult::Ok
    }

    // BCC - Branch if Carry Clear
    pub fn bcc(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| !cpu.status.contains(Flags::CARRY))
    }

    // BCS - Branch if Carry Set
    pub fn bcs(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| cpu.status.contains(Flags::CARRY))
    }

    // BEQ - Branch if Equal
    pub fn beq(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| cpu.status.contains(Flags::ZERO))
    }

    // BNE - Branch if Not Equal
    pub fn bne(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| !cpu.status.contains(Flags::ZERO))
    }

    // BMI - Branch if Minus
    pub fn bmi(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| cpu.status.contains(Flags::NEGATIVE))
    }

    // BPL - Branch if Positive
    pub fn bpl(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| !cpu.status.contains(Flags::NEGATIVE))
    }

    // BVC - Branch if Overflow Clear
    pub fn bvc(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| !cpu.status.contains(Flags::OVERFLOW))
    }

    // BVS - Branch if Overflow Set
    pub fn bvs(cpu: &mut Mos6502) -> InstructionResult {
        Branch::branch_rule(cpu, |cpu| cpu.status.contains(Flags::OVERFLOW))
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;

    use crate::cpus::mos_6502::instruction_set::helpers::Helpers;

    macro_rules! assert_branch_operation {
        ($flags:expr, $expected_pc:expr, $func:expr) => {
            let mut cpu = Helpers::create_cpu(
                0xAA,
                0x0,
                Some(vec![(0xAA, 0x02), (0xAB, 0x00)]),
                None,
                Some($flags),
            );

            ($func)(&mut cpu);

            assert_eq_hex!($expected_pc, cpu.program_counter);
        };
    }

    #[test]
    fn test_bcc_branches_if_carry_clear() {
        assert_branch_operation!(Flags::empty(), 0xAD, Branch::bcc);
    }

    #[test]
    fn test_bcc_doesnt_branches_if_carry_set() {
        assert_branch_operation!(Flags::CARRY, 0xAB, Branch::bcc);
    }

    #[test]
    fn test_bcs_branches_if_carry_set() {
        assert_branch_operation!(Flags::CARRY, 0xAD, Branch::bcs);
    }

    #[test]
    fn test_bcs_doesnt_branches_if_carry_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAB, Branch::bcs);
    }

    #[test]
    fn test_beq_branches_if_zero_set() {
        assert_branch_operation!(Flags::ZERO, 0xAD, Branch::beq);
    }

    #[test]
    fn test_beq_doesnt_branches_if_zero_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAB, Branch::beq);
    }

    #[test]
    fn test_bmi_branches_if_negative_set() {
        assert_branch_operation!(Flags::NEGATIVE, 0xAD, Branch::bmi);
    }

    #[test]
    fn test_bmi_doesnt_branches_if_negative_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAB, Branch::bmi);
    }

    #[test]
    fn test_bne_branches_if_zero_clear() {
        assert_branch_operation!(Flags::empty(), 0xAD, Branch::bne);
    }

    #[test]
    fn test_bne_doesnt_branches_if_zero_set() {
        assert_branch_operation!(Flags::ZERO, 0xAB, Branch::bne);
    }

    #[test]
    fn test_bpl_branches_if_negative_clear() {
        assert_branch_operation!(Flags::empty(), 0xAD, Branch::bpl);
    }

    #[test]
    fn test_bpl_doesnt_branches_if_negative_set() {
        assert_branch_operation!(Flags::NEGATIVE, 0xAB, Branch::bpl);
    }

    #[test]
    fn test_bvc_branches_if_overflow_clear() {
        assert_branch_operation!(Flags::empty(), 0xAD, Branch::bvc);
    }

    #[test]
    fn test_bvc_doesnt_branches_if_overflow_set() {
        assert_branch_operation!(Flags::OVERFLOW, 0xAB, Branch::bvc);
    }

    #[test]
    fn test_bvs_branches_if_overflow_set() {
        assert_branch_operation!(Flags::OVERFLOW, 0xAD, Branch::bvs);
    }

    #[test]
    fn test_bvs_doesnt_branches_if_overflow_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAB, Branch::bvs);
    }
}
