use super::cpu::Mos6502;

pub trait InstructionSet {
    // LDA - Load Accumulator
    fn lda(&mut self, param: u8);

    // TAX - Transfer Accumulator to X
    fn tax(&mut self);

    // INX - Increment X Register
    fn inx(&mut self);
}

impl InstructionSet for Mos6502 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpus::mos_6502::status::Flags;

    #[test]
    fn test_lda_immediate_load_data() {
        let mut cpu = Mos6502::create_cpu_with_program(&[0xA9, 0x05, 0x00]);
        cpu.run();

        assert_eq!(0x05, cpu.registers.a);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::ZERO);
        assert_eq!(Flags::empty(), cpu.status.flags & Flags::NEGATIVE);
    }

    #[test]
    fn test_lda_zero_flag_set() {
        let mut cpu = Mos6502::create_cpu_with_program(&[0xA9, 0x00, 0x00]);

        cpu.run();

        assert_eq!(Flags::ZERO, cpu.status.flags & Flags::ZERO);
    }

    #[test]
    fn test_tax_copies_accumulator_to_x_register() {
        let mut cpu = Mos6502::create_cpu_with_program(&[0xA9, 0x05, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(0x05, cpu.registers.x);
        assert_eq!(
            Flags::empty(),
            cpu.status.flags & (Flags::ZERO | Flags::NEGATIVE)
        );
    }

    #[test]
    fn test_tax_zero_flag_set() {
        let mut cpu = Mos6502::create_cpu_with_program(&[0xA9, 0x00, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(0x00, cpu.registers.x);
        assert_eq!(Flags::ZERO, cpu.status.flags);
    }

    #[test]
    fn test_tax_negative_flag_set() {
        let mut cpu = Mos6502::create_cpu_with_program(&[0xAA, 0x00]);
        cpu.registers.a = 0xF0;

        cpu.run();

        assert_eq!(0xF0, cpu.registers.x);
        assert_eq!(Flags::NEGATIVE, cpu.status.flags);
    }

    #[test]
    fn lda_tax_and_inx_set_register_x() {
        // Load 0xA9 into A, copy to X, increment X = 0xC1
        let mut cpu = Mos6502::create_cpu_with_program(&[0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

        cpu.run();

        assert_eq!(0xC1, cpu.registers.x);
    }

    #[test]
    fn test_inx_overflows() {
        let mut cpu = Mos6502::create_cpu_with_program(&[0xE8, 0xE8, 0x00]);
        cpu.registers.x = 0xFF;

        cpu.run();

        assert_eq!(cpu.registers.x, 1)
    }
}
