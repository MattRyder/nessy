pub enum OpcodeBehaviour {
    // Read the value from memory.
    Read,
    // Writes a value to memory.
    Write,
    // Reads from memory, modifies the value, writes it back to memory.
    ReadModifyWrite,
    // Performs a control flow operation.
    Control,
    // Branches off depending on a condition.
    Branch,
}

impl OpcodeBehaviour {
    pub fn from_mnemonic(mnemonic: &str) -> Option<Self> {
        match mnemonic {
            "ADC" | "AND" | "BIT" | "CMP" | "CPX" | "CPY" | "EOR" | "LDA" | "LDX" | "LDY"
            | "ORA" | "SBC" => Some(OpcodeBehaviour::Read),
            "STA" | "STX" | "STY" => Some(OpcodeBehaviour::Write),
            "ASL" | "DEC" | "INC" | "LSR" | "ROL" | "ROR" => Some(OpcodeBehaviour::ReadModifyWrite),
            "BCC" | "BCS" | "BEQ" | "BMI" | "BNE" | "BPL" | "BVC" | "BVS" => {
                Some(OpcodeBehaviour::Branch)
            }
            "JMP" | "JSR" | "RTS" | "RTI" | "BRK" => Some(OpcodeBehaviour::Control),
            _ => None,
        }
    }
}
