use crate::{
    cpus::mos_6502::{
        address_mode::{AddressMode, MemoryAddressing},
        cpu::Mos6502,
        instruction_set::stack::Stack,
        memory::MemoryAccess,
        opcode::OpCode,
    },
    interpret_result::InstructionResult,
};

pub enum JumpType {
    Absolute,
    Indirect,
}

// The following instructions modify the program counter causing a break to normal sequential execution.
pub struct Jump {}

impl Jump {
    // JMP - Jump
    pub fn jmp(cpu: &mut Mos6502, jump_type: JumpType) -> InstructionResult {
        match jump_type {
            JumpType::Absolute => {
                cpu.program_counter = cpu.get_address(&AddressMode::Absolute);
            }
            JumpType::Indirect => {
                // First address
                let ptr_lo = cpu.memory.read(cpu.program_counter);
                cpu.program_counter = cpu.program_counter.wrapping_add(1);
                let ptr_hi = cpu.memory.read(cpu.program_counter);
                cpu.program_counter = cpu.program_counter.wrapping_add(1);

                // Form 16-bit ptr address
                let ptr = u16::from(ptr_lo) | (u16::from(ptr_hi) << 8);

                // Get the indirect jump's lo byte of the address
                let jump_target_lo = cpu.memory.read(ptr);

                // Build up the high byte of the jump target
                let hi_address = (ptr & 0xFF00) | ((ptr.wrapping_add(1)) & 0x00FF);
                let jump_target_hi = cpu.memory.read(hi_address);

                cpu.program_counter = u16::from(jump_target_lo) | (u16::from(jump_target_hi) << 8);
            }
        }

        InstructionResult::Ok
    }

    // JSR - Jump to Subroutine
    pub fn jsr(opcode: &OpCode, cpu: &mut Mos6502) -> InstructionResult {
        if opcode.address_mode != AddressMode::Absolute {
            return InstructionResult::IllegalInstruction;
        }

        let subroutine_address = cpu.get_address(&opcode.address_mode);

        cpu.program_counter += opcode.bytes as u16;

        // Get the high / lo bytes of the current PC
        let hi_byte = (cpu.program_counter >> 8) as u8;
        let lo_byte = (cpu.program_counter & 0x00FF) as u8;

        if let Err(instruction_result) = Stack::push(cpu, hi_byte) {
            return instruction_result;
        }

        if let Err(instruction_result) = Stack::push(cpu, lo_byte) {
            return instruction_result;
        }

        cpu.program_counter = subroutine_address;

        InstructionResult::Ok
    }

    // RTS - Return from Subroutine
    pub fn rts(cpu: &mut Mos6502) -> InstructionResult {
        let lo = match Stack::pop(cpu) {
            Ok(v) => v,
            Err(err) => return err,
        };

        let hi = match Stack::pop(cpu) {
            Ok(v) => v,
            Err(err) => return err,
        };

        let address = u16::from(lo) | (u16::from(hi) << 8);
        cpu.program_counter = address;

        InstructionResult::Ok
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;
    use crate::cpus::mos_6502::{
        cpu::Mos6502,
        instruction_set::{helpers::Helpers, stack::STACK_BOTTOM},
        memory::Memory,
    };

    #[test]
    fn test_jmp_with_absolute_sets_pc_to_address() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![(0x0A, 0x00), (0x0B, 0x80)]),
            program_counter: 0x0A,
            ..Default::default()
        };

        assert_eq!(
            InstructionResult::Ok,
            Jump::jmp(&mut cpu, JumpType::Absolute)
        );

        assert_eq_hex!(0x8000, cpu.program_counter);
    }

    #[test]
    fn test_jmp_with_indirect_sets_pc_to_address() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![
                (0x0A, 0x00),
                (0x0B, 0x0F),
                (0x0F00, 0x0B),
                (0x0F01, 0x0A),
            ]),
            program_counter: 0x0A,
            ..Default::default()
        };

        assert_eq!(
            InstructionResult::Ok,
            Jump::jmp(&mut cpu, JumpType::Indirect)
        );

        assert_eq_hex!(0x0A0B, cpu.program_counter);
    }

    #[test]
    fn test_jmp_with_indirect_sets_pc_to_address_given_page_boundary_bug() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![
                (0x0A, 0xFF),
                (0x0B, 0x01),
                (0x01FF, 0x34),
                (0x0100, 0x12),
            ]),
            program_counter: 0x0A,
            ..Default::default()
        };

        assert_eq!(
            InstructionResult::Ok,
            Jump::jmp(&mut cpu, JumpType::Indirect)
        );

        assert_eq_hex!(0x1234, cpu.program_counter);
    }

    #[test]
    fn test_jsr_works() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![
                (0x1234, 0xFF),
                (0x1235, 0x01),
                (0x01FF, 0x34),
                (0x0100, 0x12),
            ]),

            program_counter: 0x1234,
            stack_pointer: 0xFF,
            ..Default::default()
        };

        let opcode = Helpers::create_opcode(2, AddressMode::Absolute);

        assert_eq!(InstructionResult::Ok, Jump::jsr(&opcode, &mut cpu));

        assert_eq_hex!(0x01FF, cpu.program_counter);

        assert_eq_hex!(0xFD, cpu.stack_pointer);
        assert_eq_hex!(0x12, cpu.memory.read(STACK_BOTTOM + 0xFF));
        assert_eq_hex!(0x35, cpu.memory.read(STACK_BOTTOM + 0xFE));
    }

    #[test]
    fn test_jsr_returns_illegal_instruction_given_bad_address_mode() {
        let opcode = Helpers::create_opcode(1, AddressMode::ZeroPageY);
        let mut cpu = Mos6502::default();

        assert_eq!(
            InstructionResult::IllegalInstruction,
            Jump::jsr(&opcode, &mut cpu)
        );
    }

    #[test]
    fn test_rts_given_valid_stack_sets_pc_to_stack_address() {
        let mut cpu = Mos6502 {
            memory: Memory::new_with_bytes(vec![(0x01FF, 0x12), (0x01FE, 0x34)]),
            stack_pointer: 0xFD,
            ..Default::default()
        };

        assert_eq!(InstructionResult::Ok, Jump::rts(&mut cpu));

        assert_eq_hex!(0x1234, cpu.program_counter);
    }

    #[test]
    fn test_rts_returns_stack_underflow_given_empty_stack() {
        let mut cpu = Mos6502 {
            stack_pointer: 0xFF,
            program_counter: 0xAA,
            ..Default::default()
        };

        assert_eq!(InstructionResult::StackUnderflow, Jump::rts(&mut cpu));

        assert_eq_hex!(0xAA, cpu.program_counter);
    }
}
