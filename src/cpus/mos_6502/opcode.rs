use std::collections::HashMap;

use crate::{
    cpus::mos_6502::{address_mode::AddressMode, cpu::Mos6502, instruction_set::InstructionSet},
    interpret_result::InstructionResult,
};
use lazy_static::lazy_static;

macro_rules! generate_opcodes {
    ( $( ($opcode:expr, $instruction:expr, $bytes:expr, $cycles:expr, $address_mode:expr, $execute:expr) ),* $(,)? ) => {{
        let mut hash_map = HashMap::new();
        $(
            hash_map.insert($opcode, OpCode::new($opcode, $instruction, $bytes, $cycles, $address_mode, $execute));
        )*
        hash_map
}}
}

pub enum Instruction {
    Brk,
    Lda,
    Tax,
    Inx,
}

// Opcode, Instruction, Bytes, Cycles, AddressMode
lazy_static! {
    #[rustfmt::skip]
    pub static ref OPCODES: HashMap<u8, OpCode> = generate_opcodes!(
        (0x00, "BRK", 1, 7, AddressMode::Implied, |_opcode: &OpCode, _cpu: &mut Mos6502| { InstructionSet::brk() }),
        (0xA9, "LDA", 2, 2, AddressMode::Immediate, |opcode: &OpCode, cpu: &mut Mos6502| { InstructionSet::lda(opcode, cpu) }),
        (0xAA, "TAX", 1, 2, AddressMode::Implied, |_opcode: &OpCode, cpu: &mut Mos6502| { InstructionSet::tax(cpu) }),
        (0xE8, "INX", 1, 2, AddressMode::Implied, |_opcode: &OpCode, cpu: &mut Mos6502| { InstructionSet::inx(cpu) }),
    );
}

#[derive(Debug)]
pub struct OpCode {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub address_mode: AddressMode,
    pub execute: fn(&OpCode, &mut Mos6502) -> InstructionResult,
}

impl OpCode {
    pub fn new(
        opcode: u8,
        mnemonic: &'static str,
        bytes: u8,
        cycles: u8,
        address_mode: AddressMode,
        execute: fn(&OpCode, &mut Mos6502) -> InstructionResult,
    ) -> Self {
        OpCode {
            opcode,
            mnemonic,
            bytes,
            cycles,
            address_mode,
            execute,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cpus::mos_6502::address_mode::AddressMode;

    #[test]
    fn test_new_creates_opcode() {
        fn fun_name(_opcode: &OpCode, _cpu: &mut Mos6502) -> InstructionResult {
            InstructionResult::Ok
        }

        let opcode = OpCode::new(
            0x00,
            "BRK",
            1,
            7,
            AddressMode::Implied,
            |opcode: &OpCode, cpu: &mut Mos6502| fun_name(opcode, cpu),
        );
        assert_eq!(0x00, opcode.opcode);
        assert_eq!("BRK", opcode.mnemonic);
        assert_eq!(1, opcode.bytes);
        assert_eq!(7, opcode.cycles);
        assert_eq!(AddressMode::Implied, opcode.address_mode);
    }
}
