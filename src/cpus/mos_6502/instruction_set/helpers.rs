use crate::{
    cpus::mos_6502::{address_mode::AddressMode, opcode::OpCode},
    interpret_result::InstructionResult,
};

#[macro_export]
macro_rules! assert_registers {
    ($cpu:expr, $a:expr, $x:expr, $y:expr) => {
        assert_eq!($cpu.registers.a, $a);
        assert_eq!($cpu.registers.x, $x);
        assert_eq!($cpu.registers.y, $y);
    };
}

#[macro_export]
macro_rules! assert_memory_value {
    ($memory:expr, $address:expr, $expected:expr) => {
        assert_eq!($expected, $memory.read($address));
    };
}

pub struct Helpers {}

impl Helpers {
    pub fn create_opcode(bytes: u8, address_mode: AddressMode) -> OpCode {
        OpCode::new(0xFF, "TEST", bytes, 1, address_mode, |_, _| {
            InstructionResult::Ok
        })
    }
}
