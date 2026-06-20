use crate::{cpus::mos_6502::cpu::Mos6502, interpret_result::InstructionResult};

pub struct Transfer {}

impl Transfer {
    // TAX - Transfer Accumulator to X
    pub fn tax(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.x = cpu.registers.a;

        cpu.status.set_zero_flag(cpu.registers.x);
        cpu.status.set_negative_flag(cpu.registers.x);

        InstructionResult::Ok
    }

    // TAY - Transfer Accumulator to Y
    pub fn tay(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.y = cpu.registers.a;

        cpu.status.set_zero_flag(cpu.registers.y);
        cpu.status.set_negative_flag(cpu.registers.y);

        InstructionResult::Ok
    }

    // TSX - Transfer Stack Pointer to X
    pub fn tsx(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.x = cpu.stack_pointer;

        cpu.status.set_zero_flag(cpu.registers.x);
        cpu.status.set_negative_flag(cpu.registers.x);

        InstructionResult::Ok
    }

    // TXA - Transfer X to Accumulator
    pub fn txa(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = cpu.registers.x;

        cpu.status.set_zero_flag(cpu.registers.a);
        cpu.status.set_negative_flag(cpu.registers.a);

        InstructionResult::Ok
    }

    // TXS - Transfer X to Stack Pointer
    pub fn txs(cpu: &mut Mos6502) -> InstructionResult {
        cpu.stack_pointer = cpu.registers.x;

        InstructionResult::Ok
    }

    // TYA - Transfer Y to Accumulator
    pub fn tya(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = cpu.registers.y;

        cpu.status.set_zero_flag(cpu.registers.a);
        cpu.status.set_negative_flag(cpu.registers.a);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::{
        cpu::{Mos6502, Registers},
        status::Flags,
    };

    #[parameterized]
    #[case(0x05, Flags::empty())]
    #[case(0x00, Flags::ZERO)]
    #[case(0xF0, Flags::NEGATIVE)]
    fn test_tax_copies_accumulator_to_x_register(acca: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            status: Flags::empty(),
            ..Default::default()
        };

        cpu.registers.a = acca;

        Transfer::tax(&mut cpu);

        assert_eq_hex!(acca, cpu.registers.x);
        assert_eq!(flags, cpu.status);
    }

    #[parameterized]
    #[case(0x05, Flags::empty())]
    #[case(0xAA, Flags::NEGATIVE)]
    #[case(0x00, Flags::ZERO)]
    fn test_tsx_copies_sp_to_x(stack_pointer: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            stack_pointer,
            status: Flags::empty(),
            ..Default::default()
        };

        Transfer::tsx(&mut cpu);

        assert_eq_hex!(stack_pointer, cpu.registers.x);
        assert_eq!(flags, cpu.status);
    }

    #[parameterized]
    #[case(0x05, Flags::empty())]
    #[case(0xAA, Flags::NEGATIVE)]
    #[case(0x00, Flags::ZERO)]
    fn test_txa_copies_x_to_accumulator(x_register: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: x_register,
                y: 0,
            },
            status: Flags::empty(),
            ..Default::default()
        };

        Transfer::txa(&mut cpu);

        assert_eq_hex!(x_register, cpu.registers.a);
        assert_eq_hex!(flags, cpu.status);
    }

    #[test]
    fn test_txs_copies_sp_to_x() {
        let mut cpu = Mos6502 {
            registers: Registers {
                x: 0x32,
                y: 0,
                a: 0,
            },
            status: Flags::empty(),
            ..Default::default()
        };

        Transfer::txs(&mut cpu);

        assert_eq_hex!(0x32, cpu.stack_pointer);
        assert_eq_hex!(Flags::empty(), cpu.status);
    }

    #[parameterized]
    #[case(0x05, Flags::empty())]
    #[case(0xAA, Flags::NEGATIVE)]
    #[case(0x00, Flags::ZERO)]
    fn test_tya_copies_y_to_accumulator(y_register: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            registers: Registers {
                x: 0,
                y: y_register,
                a: 0,
            },
            status: Flags::empty(),
            ..Default::default()
        };

        Transfer::tya(&mut cpu);

        assert_eq_hex!(y_register, cpu.registers.a);
        assert_eq_hex!(flags, cpu.status);
    }

    #[parameterized]
    #[case(0x05, Flags::empty())]
    #[case(0x00, Flags::ZERO)]
    #[case(0xF0, Flags::NEGATIVE)]
    fn test_tay_copies_accumulator_to_y_register(acca: u8, flags: Flags) {
        let mut cpu = Mos6502 {
            status: Flags::empty(),
            ..Default::default()
        };

        cpu.registers.a = acca;

        Transfer::tay(&mut cpu);

        assert_eq_hex!(acca, cpu.registers.y);
        assert_eq!(flags, cpu.status);
    }
}
