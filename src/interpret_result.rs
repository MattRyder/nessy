#[derive(Debug, PartialEq)]
pub enum InstructionResult {
    Ok,
    IllegalInstruction,
    EndProgram,
    StackOverflow,
    StackUnderflow,
}

#[derive(Debug, PartialEq)]
pub enum ProgramResult {
    Ok,
}
