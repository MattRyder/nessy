use crate::{
    cpus::mos_6502::{cpu::Mos6502, memory::MemoryAccess, status::Flags},
    interpret_result::InstructionResult,
};

pub struct Bcc {}

// BCC - Branch if Carry Clear
impl Bcc {
    pub fn bcc(cpu: &mut Mos6502) -> InstructionResult {
        if cpu.status.flags.contains(Flags::CARRY) {
            let offset = cpu.memory.read(cpu.program_counter);
            cpu.program_counter += offset as u16;
        }

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{cpu::Mos6502, memory::Memory, status::Status};

    #[test]
    fn test_bcc_branches_if_carry() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![(0xAA, 0x02), (0xAB, 0x00)]),
            program_counter: 0xAA,
            status: Status {
                flags: Flags::CARRY,
            },
            ..Default::default()
        };

        Bcc::bcc(&mut cpu);

        assert_eq_hex!(0xAC, cpu.program_counter);
    }
}
