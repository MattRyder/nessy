use crate::interpret_result::InstructionResult;

// BRK - Force Interrupt
pub struct Brk {}

impl Brk {
    pub fn brk() -> InstructionResult {
        InstructionResult::EndProgram
    }
}

#[test]
fn brk_returns_end_program() {
    assert_eq!(InstructionResult::EndProgram, Brk::brk());
}
