use crate::{cpus::mos_6502::cpu::Mos6502, interpret_result::InstructionResult};

pub struct Return {}

impl Return {
    // RTI - Return from Interrupt
    pub fn rti(_cpu: &mut Mos6502) -> InstructionResult {
        InstructionResult::Ok
    }
}
