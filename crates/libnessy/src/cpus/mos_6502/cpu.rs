use crate::{
    cpus::mos_6502::{
        bus::Bus,
        memory::{MemoryAccess, PROGRAM_ROM_START},
        opcode::OPCODES,
        status::Flags,
    },
    interpret_result::{InstructionResult, ProgramResult},
};

#[derive(Debug, Default, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

// Memory Addresses
pub const RESET_VECTOR: u16 = 0xFFFC;

pub const STACK_POINTER_RESET: u8 = 0xFF;

#[derive(Default)]
pub struct Mos6502 {
    pub registers: Registers,
    pub status: Flags,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub bus: Bus,
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
        self.status = Flags::empty();
        self.program_counter = self.bus.read_u16(RESET_VECTOR);
        self.stack_pointer = STACK_POINTER_RESET;
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.bus.write_slice(PROGRAM_ROM_START, program);
        self.bus.write_u16(RESET_VECTOR, PROGRAM_ROM_START);
    }

    pub fn run(&mut self) -> ProgramResult {
        self.run_with_callback(|_| {})
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) -> ProgramResult
    where
        F: FnMut(&mut Mos6502),
    {
        loop {
            callback(self);

            let opcode_byte = self.bus.read(self.program_counter);
            self.program_counter += 1;

            match OPCODES.get(&opcode_byte) {
                Some(opcode) => {
                    // println!(
                    //     "Exec: {} (0x{:x}) | Bytes: {} | PC: 0x{:x}",
                    //     opcode.mnemonic,
                    //     opcode.opcode,
                    //     opcode.bytes,
                    //     self.program_counter - 1
                    // );
                    //
                    match (opcode.execute)(opcode, self) {
                        InstructionResult::Ok => (),
                        InstructionResult::IllegalInstruction => {
                            panic!("Illlegal instruction! Opcode: {:?}.", opcode);
                        }
                        InstructionResult::StackOverflow => {
                            panic!("Stack overflow occurred! Opcode: {:?}.", opcode);
                        }
                        InstructionResult::StackUnderflow => {
                            panic!("Stack underflow occurred! Opcode: {:?}.", opcode);
                        }
                        InstructionResult::EndProgram => return ProgramResult::Ok,
                    }
                }
                None => panic!("Opcode not implemented: 0x{:x}.", opcode_byte),
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
        cpu.status = Flags::all();

        cpu.reset();

        assert_eq!(0, cpu.registers.a);
        assert_eq!(0, cpu.registers.x);
        assert_eq!(0, cpu.registers.y);

        assert_eq!(Flags::empty(), cpu.status);

        assert_eq!(0, cpu.program_counter);
    }
}
