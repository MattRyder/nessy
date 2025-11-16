use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        opcode::OpCode,
    },
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
    pub fn accumulator_rule(
        opcode: &OpCode,
        cpu: &mut Mos6502,
        operation: fn(cpu: &mut Mos6502, address: u16),
    ) -> InstructionResult {
        let address = cpu.get_address(&opcode.address_mode);

        operation(cpu, address);

        cpu.status.set_flags_for_result(cpu.registers.a);

        cpu.program_counter += opcode.bytes as u16;

        InstructionResult::Ok
    }

    pub fn create_opcode(bytes: u8, address_mode: AddressMode) -> OpCode {
        OpCode::new(0xFF, "TEST", bytes, 1, address_mode, |_, _| {
            InstructionResult::Ok
        })
    }
}
