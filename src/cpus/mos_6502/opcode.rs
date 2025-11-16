use std::collections::HashMap;

use crate::{
    cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::Mos6502,
        instruction_set::{and::And, asl::Asl, brk::Brk, inx::Inx, lda::Lda, ora::Ora, tax::Tax},
    },
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

lazy_static! {
    #[rustfmt::skip]
    // Opcode, Instruction, Bytes, Cycles, AddressMode
    pub static ref OPCODES: HashMap<u8, OpCode> = generate_opcodes!(
        (0x29, "AND", 2, 2, AddressMode::Immediate, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x25, "AND", 2, 3, AddressMode::ZeroPage, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x35, "AND", 2, 4, AddressMode::ZeroPageX, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x2D, "AND", 3, 4, AddressMode::Absolute, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x3D, "AND", 3, 4, AddressMode::AbsoluteX, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x39, "AND", 3, 4, AddressMode::AbsoluteY, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x21, "AND", 2, 6, AddressMode::IndirectX, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),
        (0x31, "AND", 2, 5, AddressMode::IndirectX, |opcode: &OpCode, cpu: &mut Mos6502| { And::and(opcode, cpu) }),

        (0x0A, "ASL", 1, 2, AddressMode::Accumulator,|_opcode: &OpCode, cpu: &mut Mos6502| { Asl::asl_accumulator(cpu) }),
        (0x06, "ASL", 2, 5, AddressMode::ZeroPage,|opcode: &OpCode, cpu: &mut Mos6502| { Asl::asl_memory(opcode, cpu) }),
        (0x16, "ASL", 2, 6, AddressMode::ZeroPageX,|opcode: &OpCode, cpu: &mut Mos6502| { Asl::asl_memory(opcode, cpu) }),
        (0x0E, "ASL", 3, 6, AddressMode::Absolute,|opcode: &OpCode, cpu: &mut Mos6502| { Asl::asl_memory(opcode, cpu) }),
        (0x1E, "ASL", 3, 7, AddressMode::AbsoluteX,|opcode: &OpCode, cpu: &mut Mos6502| { Asl::asl_memory(opcode, cpu) }),

        (0x00, "BRK", 1, 7, AddressMode::Implied, |_opcode: &OpCode, _cpu: &mut Mos6502| { Brk::brk() }),

        (0x09, "ORA", 2, 2, AddressMode::Immediate, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x05, "ORA", 2, 3, AddressMode::ZeroPage, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x15, "ORA", 2, 4, AddressMode::ZeroPageX, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x0D, "ORA", 3, 4, AddressMode::Absolute, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x1D, "ORA", 3, 4, AddressMode::AbsoluteX, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x19, "ORA", 3, 4, AddressMode::AbsoluteY, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x01, "ORA", 2, 6, AddressMode::IndirectX, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0x11, "ORA", 2, 5, AddressMode::IndirectY, |opcode: &OpCode, cpu: &mut Mos6502| { Ora::ora(opcode, cpu) }),
        (0xA9, "LDA", 2, 2, AddressMode::Immediate, |opcode: &OpCode, cpu: &mut Mos6502| { Lda::lda(opcode, cpu) }),

        (0xAA, "TAX", 1, 2, AddressMode::Implied, |_opcode: &OpCode, cpu: &mut Mos6502| { Tax::tax(cpu) }),

        (0xE8, "INX", 1, 2, AddressMode::Implied, |_opcode: &OpCode, cpu: &mut Mos6502| { Inx::inx(cpu) }),
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
