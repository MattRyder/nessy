use crate::{cpus::mos_6502::cpu::Mos6502, interpret_result::InstructionResult};

// INX - Increment X Register
pub struct Inx {}

impl Inx {
    pub fn inx(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.registers.x < 0xFF {
            cpu.registers.x += 1;
        } else {
            cpu.registers.x = 0;
        }

        cpu.status.set_flags_for_result(cpu.registers.x);

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::cpu::Mos6502;

    #[test]
    fn test_inx_overflows() {
        let mut cpu = Mos6502 {
            ..Default::default()
        };

        cpu.registers.a = 0xFF;

        Inx::inx(&mut cpu);

        assert_eq_hex!(cpu.registers.x, 1)
    }
}
