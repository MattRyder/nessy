pub trait InstructionSet6502 {
    // LDA - Load Accumulator
    fn lda(&mut self, param: u8);

    // TAX - Transfer Accumulator to X
    fn tax(&mut self);

    // INX - Increment X Register
    fn inx(&mut self);
}
