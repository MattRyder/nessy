#[cfg(test)]
use crate::cpus::mos_6502::{
    address_mode::AddressMode,
    cpu::{Mos6502, Registers},
    opcode::OpCode,
    status::Flags,
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

pub const MSB_MASK: u8 = 0b1000_0000;

#[cfg(test)]
pub struct Helpers {}

#[cfg(test)]
impl Helpers {
    pub fn create_opcode(bytes: u8, address_mode: AddressMode) -> OpCode {
        use crate::cpus::mos_6502::opcode::OpCode;

        OpCode::new(0xFF, "TEST", bytes, 1, address_mode, |_, _| {
            use crate::interpret_result::InstructionResult;

            InstructionResult::Ok
        })
    }

    pub fn create_cpu(
        program_counter: u16,
        stack_pointer: u8,
        memory_values: Option<Vec<(u16, u8)>>,
        registers: Option<Registers>,
        status: Option<Flags>,
    ) -> Mos6502 {
        use crate::cpus::mos_6502::cpu::Mos6502;

        let mut cpu = Mos6502 {
            program_counter,
            stack_pointer,
            registers: registers.unwrap_or_default(),
            status: status.unwrap_or_default(),
            ..Default::default()
        };

        if let Some(mem_values) = memory_values {
            for (addr, value) in mem_values {
                use crate::cpus::mos_6502::memory::MemoryAccess;

                cpu.bus.write(addr, value);
            }
        }

        cpu
    }
}
