use super::instruction_set::InstructionSet;
use super::memory::Memory;
use super::memory::{MemoryAccess, PROGRAM_ROM_START};
use super::opcode::OpCode;
use super::status::Status;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

// Memory Addresses
pub const RESET_VECTOR: u16 = 0xFFFC;

#[derive(Default)]
pub struct Mos6502 {
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

impl Mos6502 {
    pub fn create_cpu_with_program(program: &[u8]) -> Mos6502 {
        let mut cpu = Mos6502::default();
        cpu.load_program(program);
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        self.registers = Registers::default();
        self.status = Status::default();
        self.program_counter = self.memory.read_u16(RESET_VECTOR);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.write_slice(PROGRAM_ROM_START, program);
        self.memory.write_u16(RESET_VECTOR, PROGRAM_ROM_START);
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
    use crate::cpus::mos_6502::status::Flags;

    #[test]
    fn reset_resets_everything() {
        let mut cpu = Mos6502::default();
        cpu.registers.a = 1;
        cpu.registers.x = 2;
        cpu.registers.y = 3;
        cpu.status.flags = Flags::all();

        cpu.reset();

        assert_eq!(0, cpu.registers.a);
        assert_eq!(0, cpu.registers.x);
        assert_eq!(0, cpu.registers.y);

        assert_eq!(Flags::empty(), cpu.status.flags);

        assert_eq!(0, cpu.program_counter);
    }
}
