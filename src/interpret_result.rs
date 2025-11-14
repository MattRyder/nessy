#[derive(Debug, PartialEq)]
pub enum InstructionResult {
    Ok,
    IllegalInstruction,
    EndProgram,
}

#[derive(Debug, PartialEq)]
pub enum ProgramResult {
    Ok,
}
