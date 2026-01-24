use std::collections::HashMap;

use crate::{
    cpus::mos_6502::{
        address_mode::AddressMode,
        cpu::Mos6502,
        instruction_set::{
            arithmetic::Arithmetic,
            branch::Branch,
            clear::Clear,
            compare::Compare,
            decrement::Decrement,
            increment::Increment,
            jump::{Jump, JumpType},
            load::Load,
            logical::Logical,
            rotate::{Direction, Rotate},
            set::Set,
            shift::Shift,
            stack::Stack,
            store::Store,
            system::System,
            transfer::Transfer,
        },
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

lazy_static! {
    #[rustfmt::skip]
    // Opcode, Instruction, Bytes, Cycles, AddressMode
    pub static ref OPCODES: HashMap<u8, OpCode> = generate_opcodes!(
        (0x69, "ADC", 2, 2, AddressMode::Immediate, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x65, "ADC", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x75, "ADC", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x6D, "ADC", 3, 4, AddressMode::Absolute, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x7D, "ADC", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x79, "ADC", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x61, "ADC", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),
        (0x71, "ADC", 2, 5, AddressMode::IndirectY, |opcode, cpu| { Arithmetic::adc(opcode, cpu) }),

        (0x29, "AND", 2, 2, AddressMode::Immediate, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x25, "AND", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x35, "AND", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x2D, "AND", 3, 4, AddressMode::Absolute, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x3D, "AND", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x39, "AND", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x21, "AND", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Logical::and(opcode, cpu) }),
        (0x31, "AND", 2, 5, AddressMode::IndirectY, |opcode, cpu| { Logical::and(opcode, cpu) }),

        (0x0A, "ASL", 1, 2, AddressMode::Accumulator,|_, cpu| { Shift::asl_accumulator(cpu) }),
        (0x06, "ASL", 2, 5, AddressMode::ZeroPage,|opcode, cpu| { Shift::asl_memory(opcode, cpu) }),
        (0x16, "ASL", 2, 6, AddressMode::ZeroPageX,|opcode, cpu| { Shift::asl_memory(opcode, cpu) }),
        (0x0E, "ASL", 3, 6, AddressMode::Absolute,|opcode, cpu| { Shift::asl_memory(opcode, cpu) }),
        (0x1E, "ASL", 3, 7, AddressMode::AbsoluteX,|opcode, cpu| { Shift::asl_memory(opcode, cpu) }),

        (0x90, "BCC", 2, 2, AddressMode::Relative,|_, cpu| { Branch::bcc(cpu) }),

        (0xB0, "BCS", 2, 2, AddressMode::Relative,|_, cpu| { Branch::bcs(cpu) }),

        (0xF0, "BEQ", 2, 2, AddressMode::Relative,|_, cpu| { Branch::beq(cpu) }),

        (0x24, "BIT", 2, 3, AddressMode::ZeroPage,|opcode, cpu| { Logical::bit(opcode, cpu) }),
        (0x2C, "BIT", 3, 4, AddressMode::Absolute,|opcode, cpu| { Logical::bit(opcode, cpu) }),

        (0x30, "BMI", 2, 2, AddressMode::Relative,|_, cpu| { Branch::bmi(cpu) }),

        (0xD0, "BNE", 2, 2, AddressMode::Relative,|_, cpu| { Branch::bne(cpu) }),

        (0x10, "BPL", 2, 2, AddressMode::Relative,|_, cpu| { Branch::bpl(cpu) }),

        (0x00, "BRK", 1, 7, AddressMode::Implied, |_, cpu| { System::brk(cpu) }),

        (0x50, "BVC", 2, 2, AddressMode::Relative, |_, cpu| { Branch::bvc(cpu) }),

        (0x70, "BVS", 2, 2, AddressMode::Relative, |_, cpu| { Branch::bvs(cpu) }),

        (0x18, "CLC", 1, 2, AddressMode::Implied, |_, cpu| { Clear::clc(cpu) }),

        (0xD8, "CLD", 1, 2, AddressMode::Implied, |_, cpu| { Clear::cld(cpu) }),

        (0x58, "CLI", 1, 2, AddressMode::Implied, |_, cpu| { Clear::cli(cpu) }),

        (0xB8, "CLV", 1, 2, AddressMode::Implied, |_, cpu| { Clear::clv(cpu) }),

        (0xC9, "CMP", 2, 2, AddressMode::Immediate, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xC5, "CMP", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xD5, "CMP", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xCD, "CMP", 3, 4, AddressMode::Absolute, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xDD, "CMP", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xD9, "CMP", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xC1, "CMP", 2, 2, AddressMode::IndirectX, |opcode, cpu| { Compare::cmp(opcode, cpu) }),
        (0xD1, "CMP", 2, 2, AddressMode::IndirectY, |opcode, cpu| { Compare::cmp(opcode, cpu) }),

        (0xE0, "CPX", 2, 2, AddressMode::Immediate, |opcode, cpu| { Compare::cpx(opcode, cpu) }),
        (0xE4, "CPX", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Compare::cpx(opcode, cpu) }),
        (0xEC, "CPX", 2, 4, AddressMode::Absolute, |opcode, cpu| { Compare::cpx(opcode, cpu) }),

        (0xC0, "CPY", 2, 2, AddressMode::Immediate, |opcode, cpu| { Compare::cpy(opcode, cpu) }),
        (0xC4, "CPY", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Compare::cpy(opcode, cpu) }),
        (0xCC, "CPY", 2, 4, AddressMode::Absolute, |opcode, cpu| { Compare::cpy(opcode, cpu) }),

        (0xC6, "DEC", 2, 5, AddressMode::ZeroPage, |opcode, cpu| { Decrement::dec(opcode, cpu) }),
        (0xD6, "DEC", 2, 6, AddressMode::ZeroPageX, |opcode, cpu| { Decrement::dec(opcode, cpu) }),
        (0xCE, "DEC", 3, 6, AddressMode::Absolute, |opcode, cpu| { Decrement::dec(opcode, cpu) }),
        (0xDE, "DEC", 3, 7, AddressMode::AbsoluteX, |opcode, cpu| { Decrement::dec(opcode, cpu) }),

        (0xCA, "DEX", 2, 4, AddressMode::Implied, |opcode, cpu| { Decrement::dex(opcode, cpu) }),

        (0x88, "DEY", 2, 4, AddressMode::Implied, |opcode, cpu| { Decrement::dey(opcode, cpu) }),

        (0x49, "EOR", 2, 2, AddressMode::Immediate, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x45, "EOR", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x55, "EOR", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x4D, "EOR", 3, 4, AddressMode::Absolute, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x5D, "EOR", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x59, "EOR", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x41, "EOR", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Logical::eor(opcode, cpu) }),
        (0x51, "EOR", 2, 5, AddressMode::IndirectY, |opcode, cpu| { Logical::eor(opcode, cpu) }),

        (0xE6, "INC", 2, 5, AddressMode::ZeroPage, |opcode, cpu| { Increment::inc(opcode, cpu) }),
        (0xF6, "INC", 2, 6, AddressMode::ZeroPageX, |opcode, cpu| { Increment::inc(opcode, cpu) }),
        (0xEE, "INC", 3, 6, AddressMode::Absolute, |opcode, cpu| { Increment::inc(opcode, cpu) }),
        (0xFE, "INC", 3, 7, AddressMode::AbsoluteX, |opcode, cpu| { Increment::inc(opcode, cpu) }),

        (0xE8, "INX", 1, 2, AddressMode::Implied, |_, cpu| { Increment::inx(cpu) }),

        (0xC8, "INY", 1, 2, AddressMode::Implied, |_, cpu| { Increment::iny(cpu) }),

        (0x4C, "JMP", 3, 2, AddressMode::Absolute, |_, cpu| { Jump::jmp(cpu, JumpType::Absolute) }),
        (0x6C, "JMP", 3, 5, AddressMode::None, |_, cpu| { Jump::jmp(cpu, JumpType::Indirect) }),

        (0x20, "JSR", 3, 6, AddressMode::Absolute, |opcode, cpu| { Jump::jsr(opcode, cpu) }),

        (0xA9, "LDA", 2, 2, AddressMode::Immediate, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xA5, "LDA", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xB5, "LDA", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xAD, "LDA", 3, 4, AddressMode::Absolute, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xBD, "LDA", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xB9, "LDA", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xA1, "LDA", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Load::lda(opcode, cpu) }),
        (0xB1, "LDA", 2, 5, AddressMode::IndirectY, |opcode, cpu| { Load::lda(opcode, cpu) }),

        (0xA2, "LDX", 2, 2, AddressMode::Immediate, |opcode, cpu| { Load::ldx(opcode, cpu) }),
        (0xA6, "LDX", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Load::ldx(opcode, cpu) }),
        (0xB6, "LDX", 2, 4, AddressMode::ZeroPageY, |opcode, cpu| { Load::ldx(opcode, cpu) }),
        (0xAE, "LDX", 3, 4, AddressMode::Absolute, |opcode, cpu| { Load::ldx(opcode, cpu) }),
        (0xBE, "LDX", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Load::ldx(opcode, cpu) }),

        (0xA0, "LDY", 2, 2, AddressMode::Immediate, |opcode, cpu| { Load::ldy(opcode, cpu) }),
        (0xA4, "LDY", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Load::ldy(opcode, cpu) }),
        (0xB4, "LDY", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Load::ldy(opcode, cpu) }),
        (0xAC, "LDY", 3, 4, AddressMode::Absolute, |opcode, cpu| { Load::ldy(opcode, cpu) }),
        (0xBC, "LDY", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Load::ldy(opcode, cpu) }),

        (0x4A, "LSR", 1, 2, AddressMode::Accumulator,|_, cpu| { Shift::lsr_accumulator(cpu) }),
        (0x46, "LSR", 2, 5, AddressMode::ZeroPage,|opcode, cpu| { Shift::lsr_memory(opcode, cpu) }),
        (0x56, "LSR", 2, 6, AddressMode::ZeroPageX,|opcode, cpu| { Shift::lsr_memory(opcode, cpu) }),
        (0x4E, "LSR", 3, 6, AddressMode::Absolute,|opcode, cpu| { Shift::lsr_memory(opcode, cpu) }),
        (0x5E, "LSR", 3, 7, AddressMode::AbsoluteX,|opcode, cpu| { Shift::lsr_memory(opcode, cpu) }),

        (0xEA, "NOP", 1, 2, AddressMode::Implied, |_, _| { System::nop() }),

        (0x09, "ORA", 2, 2, AddressMode::Immediate, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x05, "ORA", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x15, "ORA", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x0D, "ORA", 3, 4, AddressMode::Absolute, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x1D, "ORA", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x19, "ORA", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x01, "ORA", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Logical::ora(opcode, cpu) }),
        (0x11, "ORA", 2, 5, AddressMode::IndirectY, |opcode, cpu| { Logical::ora(opcode, cpu) }),

        (0x48, "PHA", 1, 3, AddressMode::Implied, |_, cpu| { Stack::pha(cpu) }),

        (0x08, "PHP", 1, 3, AddressMode::Implied, |_, cpu| { Stack::php(cpu) }),

        (0x28, "PLP", 1, 4, AddressMode::Implied, |_, cpu| { Stack::plp(cpu) }),

        (0x68, "PLA", 1, 4, AddressMode::Implied, |_, cpu| { Stack::pla(cpu) }),

        (0x60, "RTS", 1, 6, AddressMode::Implied, |_, cpu| { Jump::rts(cpu) }),

        (0x40, "RTI", 1, 6, AddressMode::Implied, |_, cpu| { System::rti(cpu) }),

        (0x85, "STA", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Store::sta(opcode, cpu) }),
        (0x95, "STA", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Store::sta(opcode, cpu) }),
        (0x8D, "STA", 3, 4, AddressMode::Absolute, |opcode, cpu| { Store::sta(opcode, cpu) }),
        (0x9D, "STA", 3, 5, AddressMode::AbsoluteX, |opcode, cpu| { Store::sta(opcode, cpu) }),
        (0x99, "STA", 3, 5, AddressMode::AbsoluteY, |opcode, cpu| { Store::sta(opcode, cpu) }),
        (0x81, "STA", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Store::sta(opcode, cpu) }),
        (0x91, "STA", 2, 6, AddressMode::IndirectY, |opcode, cpu| { Store::sta(opcode, cpu) }),

        (0x86, "STX", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Store::stx(opcode, cpu) }),
        (0x96, "STX", 2, 4, AddressMode::ZeroPageY, |opcode, cpu| { Store::stx(opcode, cpu) }),
        (0x8E, "STX", 3, 4, AddressMode::Absolute, |opcode, cpu| { Store::stx(opcode, cpu) }),

        (0x84, "STY", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Store::sty(opcode, cpu) }),
        (0x94, "STY", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Store::sty(opcode, cpu) }),
        (0x8C, "STY", 3, 4, AddressMode::Absolute, |opcode, cpu| { Store::sty(opcode, cpu) }),

        (0xE9, "SBC", 2, 2, AddressMode::Immediate, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xE5, "SBC", 2, 3, AddressMode::ZeroPage, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xF5, "SBC", 2, 4, AddressMode::ZeroPageX, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xED, "SBC", 3, 4, AddressMode::Absolute, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xFD, "SBC", 3, 4, AddressMode::AbsoluteX, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xF9, "SBC", 3, 4, AddressMode::AbsoluteY, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xE1, "SBC", 2, 6, AddressMode::IndirectX, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),
        (0xF1, "SBC", 2, 5, AddressMode::IndirectY, |opcode, cpu| { Arithmetic::sbc(opcode, cpu) }),

        (0x38, "SEC", 1, 2, AddressMode::Implied, |_, cpu| { Set::sec(cpu) }),
        (0xF1, "SED", 1, 2, AddressMode::Implied, |_, cpu| { Set::sed(cpu) }),
        (0x78, "SEI", 1, 2, AddressMode::Implied, |_, cpu| { Set::sei(cpu) }),

        (0x2A, "ROL", 1, 2, AddressMode::Accumulator, |_, cpu| { Rotate::rotate_accumulator(cpu, Direction::Left) }),
        (0x26, "ROL", 2, 5, AddressMode::ZeroPage, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Left) }),
        (0x36, "ROL", 2, 6, AddressMode::ZeroPageX, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Left) }),
        (0x2E, "ROL", 3, 6, AddressMode::Absolute, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Left) }),
        (0x3E, "ROL", 3, 7, AddressMode::AbsoluteX, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Left) }),

        (0x6A, "ROR", 1, 2, AddressMode::Accumulator, |_, cpu| { Rotate::rotate_accumulator(cpu, Direction::Right) }),
        (0x66, "ROR", 2, 5, AddressMode::ZeroPage, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Right) }),
        (0x76, "ROR", 2, 6, AddressMode::ZeroPageX, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Right) }),
        (0x6E, "ROR", 3, 6, AddressMode::Absolute, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Right) }),
        (0x7E, "ROR", 3, 7, AddressMode::AbsoluteX, |opcode, cpu| { Rotate::rotate_memory(opcode, cpu, Direction::Right) }),

        (0xAA, "TAX", 1, 2, AddressMode::Implied, |_, cpu| { Transfer::tax(cpu) }),

        (0xA8, "TAY", 1, 2, AddressMode::Implied, |_, cpu| { Transfer::tay(cpu) }),

        (0xBA, "TSX", 1, 2, AddressMode::Implied, |_, cpu| { Transfer::tsx(cpu) }),

        (0x8A, "TXA", 1, 2, AddressMode::Implied, |_, cpu| { Transfer::txa(cpu) }),

        (0x9A, "TXS", 1, 2, AddressMode::Implied, |_, cpu| { Transfer::txs(cpu) }),

        (0x98, "TYA", 1, 2, AddressMode::Implied, |_, cpu| { Transfer::tya(cpu) }),
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
            bytes: bytes - 1,
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

        let opcode = OpCode::new(0x00, "BRK", 1, 7, AddressMode::Implied, |opcode, cpu| {
            fun_name(opcode, cpu)
        });
        assert_eq!(0x00, opcode.opcode);
        assert_eq!("BRK", opcode.mnemonic);
        assert_eq!(0, opcode.bytes);
        assert_eq!(7, opcode.cycles);
        assert_eq!(AddressMode::Implied, opcode.address_mode);
    }

    #[test]
    fn test_opcodes_count() {
        assert_eq!(150, OPCODES.len());
    }
}
