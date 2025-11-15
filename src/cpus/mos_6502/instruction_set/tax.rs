use crate::{cpus::mos_6502::cpu::Mos6502, interpret_result::InstructionResult};

pub struct Tax {}

impl Tax {
    // TAX - Transfer Accumulator to X
    pub fn tax(cpu: &mut Mos6502) -> InstructionResult {
        cpu.registers.x = cpu.registers.a;
        cpu.status.set_flags_for_result(cpu.registers.x);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{cpu::Mos6502, status::Flags};

    #[test]
    fn test_tax_copies_accumulator_to_x_register() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0x05;

        Tax::tax(&mut cpu);

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

        Tax::tax(&mut cpu);

        assert_eq_hex!(0x00, cpu.registers.x);
        assert_eq!(Flags::ZERO, cpu.status.flags);
    }

    #[test]
    fn test_tax_negative_flag_set() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0xF0;

        Tax::tax(&mut cpu);

        assert_eq_hex!(0xF0, cpu.registers.x);
        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }
}
