use crate::{cpus::mos_6502::cpu::Mos6502, interpret_result::InstructionResult};

pub struct Transfer {}

impl Transfer {
    // TAX - Transfer Accumulator to X
    pub fn tax(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.x = cpu.registers.a;

        cpu.status.set_zero_flag(cpu.registers.x);
        cpu.status.set_negative_flag(cpu.registers.x);

        // cpu.program_counter += 1;

        InstructionResult::Ok
    }

    // TAY - Transfer Accumulator to Y
    pub fn tay(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.y = cpu.registers.a;

        cpu.status.set_zero_flag(cpu.registers.y);
        cpu.status.set_negative_flag(cpu.registers.y);

        // cpu.program_counter += 1;

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

        // cpu.program_counter += 1;

        InstructionResult::Ok
    }

    // TXS - Transfer X to Stack Pointer
    pub fn txs(cpu: &mut Mos6502) -> InstructionResult {
        cpu.stack_pointer = cpu.registers.x;

        cpu.status.set_zero_flag(cpu.stack_pointer);
        cpu.status.set_negative_flag(cpu.stack_pointer);

        // cpu.program_counter += 1;

        InstructionResult::Ok
    }

    // TYA - Transfer Y to Accumulator
    pub fn tya(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.a = cpu.registers.y;

        cpu.status.set_zero_flag(cpu.registers.a);
        cpu.status.set_negative_flag(cpu.registers.a);

        // cpu.program_counter += 1;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        cpu::{Mos6502, Registers},
        status::Flags,
    };

    #[test]
    fn test_tax_copies_accumulator_to_x_register() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0x05;

        Transfer::tax(&mut cpu);

        assert_eq_hex!(0x05, cpu.registers.x);
        assert_eq!(
            Flags::empty(),
            cpu.status.flags & (Flags::ZERO | Flags::NEGATIVE)
        );
    }

    #[test]
    fn test_tax_zero_flag_set() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        Transfer::tax(&mut cpu);

        assert_eq_hex!(0x00, cpu.registers.x);
        assert_eq!(Flags::ZERO, cpu.status.flags);
    }

    #[test]
    fn test_tax_negative_flag_set() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0xF0;

        Transfer::tax(&mut cpu);

        assert_eq_hex!(0xF0, cpu.registers.x);
        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn test_tsx_copies_sp_to_x() {
        let mut cpu = Mos6502 {
            stack_pointer: 0xBB,
            ..Default::default()
        };

        Transfer::tsx(&mut cpu);

        assert_eq_hex!(0xBB, cpu.registers.x);
    }

    #[test]
    fn test_txa_copies_x_to_accumulator() {
        let mut cpu = Mos6502 {
            registers: Registers {
                a: 0,
                x: 0x11,
                y: 0,
            },
            ..Default::default()
        };

        Transfer::txa(&mut cpu);

        assert_eq_hex!(0x11, cpu.registers.a);
    }

    #[test]
    fn test_txs_copies_sp_to_x() {
        let mut cpu = Mos6502 {
            registers: Registers {
                x: 0x32,
                y: 0,
                a: 0,
            },
            ..Default::default()
        };

        Transfer::txs(&mut cpu);

        assert_eq_hex!(0x32, cpu.stack_pointer);
    }

    #[test]
    fn test_tya_copies_y_to_accumulator() {
        let mut cpu = Mos6502 {
            registers: Registers {
                x: 0,
                y: 0x25,
                a: 0,
            },
            ..Default::default()
        };

        Transfer::tya(&mut cpu);

        assert_eq_hex!(0x25, cpu.registers.a);
    }
}
