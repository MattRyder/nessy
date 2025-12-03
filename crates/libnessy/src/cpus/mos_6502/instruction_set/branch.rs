use crate::{
    cpus::mos_6502::{cpu::Mos6502, memory::MemoryAccess, status::Flags},
    interpret_result::InstructionResult,
};

pub struct Branch {}

impl Branch {
    // BCC - Branch if Carry Clear
    pub fn bcc(cpu: &mut Mos6502) -> InstructionResult {
        if !cpu.status.flags.contains(Flags::CARRY) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BCS - Branch if Carry Set
    pub fn bcs(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.status.flags.contains(Flags::CARRY) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BEQ - Branch if Equal
    pub fn beq(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.status.flags.contains(Flags::ZERO) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BMI - Branch if Minus
    pub fn bmi(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.status.flags.contains(Flags::NEGATIVE) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BNE - Branch if Not Equal
    pub fn bne(cpu: &mut Mos6502) -> InstructionResult {
        if !cpu.status.flags.contains(Flags::ZERO) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BPL - Branch if Positive
    pub fn bpl(cpu: &mut Mos6502) -> InstructionResult {
        if !cpu.status.flags.contains(Flags::NEGATIVE) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BVC - Branch if Overflow Clear
    pub fn bvc(cpu: &mut Mos6502) -> InstructionResult {
        if !cpu.status.flags.contains(Flags::OVERFLOW) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }

    // BVS - Branch if Overflow Set
    pub fn bvs(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.status.flags.contains(Flags::OVERFLOW) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{cpu::Mos6502, memory::Memory, status::Status};

    macro_rules! assert_branch_operation {
        ($flags:expr, $expected_pc:expr, $func:expr) => {
            let mut cpu = Mos6502 {
                memory: Memory::new_with_bytes(vec![(0xAA, 0x02), (0xAB, 0x00)]),
                program_counter: 0xAA,
                status: Status { flags: $flags },
                ..Default::default()
            };

            ($func)(&mut cpu);

            assert_eq_hex!($expected_pc, cpu.program_counter);
        };
    }

    #[test]
    fn test_bcc_branches_if_carry_clear() {
        assert_branch_operation!(Flags::empty(), 0xAC, Branch::bcc);
    }

    #[test]
    fn test_bcc_doesnt_branches_if_carry_set() {
        assert_branch_operation!(Flags::CARRY, 0xAA, Branch::bcc);
    }

    #[test]
    fn test_bcs_branches_if_carry_set() {
        assert_branch_operation!(Flags::CARRY, 0xAC, Branch::bcs);
    }

    #[test]
    fn test_bcs_doesnt_branches_if_carry_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAA, Branch::bcs);
    }

    #[test]
    fn test_beq_branches_if_zero_set() {
        assert_branch_operation!(Flags::ZERO, 0xAC, Branch::beq);
    }

    #[test]
    fn test_beq_doesnt_branches_if_zero_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAA, Branch::beq);
    }

    #[test]
    fn test_bmi_branches_if_negative_set() {
        assert_branch_operation!(Flags::NEGATIVE, 0xAC, Branch::bmi);
    }

    #[test]
    fn test_bmi_doesnt_branches_if_negative_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAA, Branch::bmi);
    }

    #[test]
    fn test_bne_branches_if_zero_clear() {
        assert_branch_operation!(Flags::empty(), 0xAC, Branch::bne);
    }

    #[test]
    fn test_bne_doesnt_branches_if_zero_set() {
        assert_branch_operation!(Flags::ZERO, 0xAA, Branch::bne);
    }

    #[test]
    fn test_bpl_branches_if_negative_clear() {
        assert_branch_operation!(Flags::empty(), 0xAC, Branch::bpl);
    }

    #[test]
    fn test_bpl_doesnt_branches_if_negative_set() {
        assert_branch_operation!(Flags::NEGATIVE, 0xAA, Branch::bpl);
    }

    #[test]
    fn test_bvc_branches_if_overflow_clear() {
        assert_branch_operation!(Flags::empty(), 0xAC, Branch::bvc);
    }

    #[test]
    fn test_bvc_doesnt_branches_if_overflow_set() {
        assert_branch_operation!(Flags::OVERFLOW, 0xAA, Branch::bvc);
    }

    #[test]
    fn test_bvs_branches_if_overflow_set() {
        assert_branch_operation!(Flags::OVERFLOW, 0xAC, Branch::bvs);
    }

    #[test]
    fn test_bvs_doesnt_branches_if_overflow_not_set() {
        assert_branch_operation!(Flags::empty(), 0xAA, Branch::bvs);
    }
}
