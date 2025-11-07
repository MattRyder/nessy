use crate::cpu::memory::{MemoryAccess, PROGRAM_ROM_START};

use super::instruction_set::InstructionSet6502;
use super::memory::Memory;
use super::opcode::OpCode;
use super::status::Status;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub status: Status,
    pub program_counter: u16,
    pub memory: Memory,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    OK,
    EmptyProgram,
}

impl InstructionSet6502 for CPU {
    fn lda(&mut self, param: u8) {
        self.registers.a = param;
        self.status.set_flags_for_result(self.registers.a);
    }

    fn tax(&mut self) {
        self.registers.x = self.registers.a;
        self.status.set_flags_for_result(self.registers.x);
    }

    fn inx(&mut self) {
        if self.registers.x < 0xFF {
            self.registers.x += 1;
        } else {
            self.registers.x = 0;
        }

        self.status.set_flags_for_result(self.registers.x);
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU::default()
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.write_slice(PROGRAM_ROM_START, program);
        self.program_counter = PROGRAM_ROM_START;
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let opcode_byte = self.memory.read(self.program_counter);
            self.program_counter += 1;

            // println!("OpCode: {:x}", opcode_byte);

            match OpCode::try_from(opcode_byte) {
                Ok(opcode) => match opcode {
                    OpCode::Break => return InterpretResult::OK,
                    OpCode::LoadAccumulator => {
                        let param = self.memory.read(self.program_counter);
                        self.program_counter += 1;
                        self.lda(param);
                    }
                    OpCode::TransferAccumulatorToX => self.tax(),
                    OpCode::IncrementX => self.inx(),
                },
                Err(_) => panic!("OpCode not implemented: {:x}", opcode_byte),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::status::Flags;

    fn create_cpu_with_program(program: &[u8]) -> CPU {
        let mut cpu = CPU::default();
        cpu.load_program(program);

        cpu
    }

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = create_cpu_with_program(&[0xA9, 0x05, 0x00]);

        cpu.run();

        assert_eq!(0x05, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }

    #[test]
    fn test_lda_zero_flag_set() {
        let mut cpu = create_cpu_with_program(&[0xA9, 0x00, 0x00]);

        cpu.run();

        assert_eq!(Flags::ZERO, cpu.status.flags & Flags::ZERO);
    }

    #[test]
    fn test_tax_copies_accumulator_to_x_register() {
        let mut cpu = create_cpu_with_program(&[0xA9, 0x05, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(0x05, cpu.registers.x);
        assert_eq!(
            Flags::empty(),
            cpu.status.flags & (Flags::ZERO | Flags::NEGATIVE)
        );
    }

    #[test]
    fn test_tax_zero_flag_set() {
        let mut cpu = create_cpu_with_program(&[0xA9, 0x00, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(0x00, cpu.registers.x);
        assert_eq!(Flags::ZERO, cpu.status.flags);
    }

    #[test]
    fn test_tax_negative_flag_set() {
        let mut cpu = create_cpu_with_program(&[0xAA, 0x00]);
        cpu.registers.a = 0xF0;

        cpu.run();

        assert_eq!(0xF0, cpu.registers.x);
        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn lda_tax_and_inx_set_register_x() {
        // Load 0xA9 into A, copy to X, increment X = 0xC1
        let mut cpu = create_cpu_with_program(&[0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        cpu.run();

        assert_eq!(0xC1, cpu.registers.x);
    }

    #[test]
    fn test_inx_overflows() {
        let mut cpu = create_cpu_with_program(&[0xE8, 0xE8, 0x00]);
        cpu.registers.x = 0xFF;

        cpu.run();

        assert_eq!(cpu.registers.x, 1)
    }
}
