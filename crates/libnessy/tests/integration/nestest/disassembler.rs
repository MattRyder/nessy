use libnessy::cpus::mos_6502::{address_mode::AddressMode, cpu::Mos6502, opcode::OpCode};

use crate::integration::nestest::opcode_behaviour::OpcodeBehaviour;

pub struct Disassembler {}

impl Disassembler {
    fn relative(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let offset = cpu.bus.read(cpu.program_counter.wrapping_add(1)) as i8;
        let target = ((cpu.program_counter + 2) as i32 + offset as i32) as u16;
        Some(format!("{} ${:04X}", opcode.mnemonic, target))
    }

    fn zeropage(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let addr = cpu.bus.read(cpu.program_counter.wrapping_add(1));
        let value = cpu.bus.read(addr as u16);
        Some(format!("{} ${:02X} = {:02X}", opcode.mnemonic, addr, value))
    }

    fn zeropage_x(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let memory_address = cpu.bus.read(cpu.program_counter.wrapping_add(1));
        let target_address = memory_address.wrapping_add(cpu.registers.x);
        let target_value = cpu.bus.read(target_address as u16);

        Some(format!(
            "{} ${:02X},X @ {:02X} = {:02X}",
            opcode.mnemonic, memory_address, target_address, target_value
        ))
    }

    fn zeropage_y(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let memory_address = cpu.bus.read(cpu.program_counter.wrapping_add(1));
        let target_address = memory_address.wrapping_add(cpu.registers.y);
        let target_value = cpu.bus.read(target_address as u16);

        Some(format!(
            "{} ${:02X},Y @ {:02X} = {:02X}",
            opcode.mnemonic, memory_address, target_address, target_value
        ))
    }

    fn absolute(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let address = cpu.bus.read_u16(cpu.program_counter.wrapping_add(1));
        let mnemonic_address = format!("{} ${:04X}", opcode.mnemonic, address);

        match OpcodeBehaviour::from_mnemonic(opcode.mnemonic) {
            Some(opcode_behaviour) => {
                if let OpcodeBehaviour::Read
                | OpcodeBehaviour::Write
                | OpcodeBehaviour::ReadModifyWrite = opcode_behaviour
                {
                    let memory_value = cpu.bus.read(address);
                    Some(format!("{} = {:02X}", mnemonic_address, memory_value))
                } else {
                    Some(mnemonic_address)
                }
            }
            _ => Some(mnemonic_address),
        }
    }

    fn absolute_x(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let memory_address = cpu.bus.read_u16(cpu.program_counter.wrapping_add(1));
        let target_address = memory_address.wrapping_add(cpu.registers.x as u16);
        let target_value = cpu.bus.read(target_address);

        Some(format!(
            "{} ${:04X},X @ {:04X} = {:02X}",
            opcode.mnemonic, memory_address, target_address, target_value
        ))
    }

    fn absolute_y(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let memory_address = cpu.bus.read_u16(cpu.program_counter.wrapping_add(1));
        let target_address = memory_address.wrapping_add(cpu.registers.y as u16);
        let target_value = cpu.bus.read(target_address);

        Some(format!(
            "{} ${:04X},Y @ {:04X} = {:02X}",
            opcode.mnemonic, memory_address, target_address, target_value
        ))
    }

    fn indirect_x(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let mnemonic_str = String::from(opcode.mnemonic);

        match OpcodeBehaviour::from_mnemonic(opcode.mnemonic) {
            Some(opcode_behaviour) => {
                if let OpcodeBehaviour::Read | OpcodeBehaviour::Write = opcode_behaviour {
                    // Get the byte of this opcode to add as the indirect value - ($FF,X)
                    let opcode_byte = cpu.bus.read(cpu.program_counter.wrapping_add(1));

                    // Memory value is the indirect byte plus the register involved. - @ FF
                    let memory_address = opcode_byte.wrapping_add(cpu.registers.x);

                    // Read the target address from the memory address generated. - 0400
                    let target_address_lo = cpu.bus.read(memory_address as u16);
                    let target_address_hi = cpu.bus.read(memory_address.wrapping_add(1) as u16);

                    // Then get the target value from that address. - 5D
                    let target_address =
                        ((target_address_hi as u16) << 8) | (target_address_lo as u16);
                    let target_value = cpu.bus.read(target_address);

                    Some(format!(
                        "{} (${:02X},X) @ {:02X} = {:04X} = {:02X}",
                        mnemonic_str, opcode_byte, memory_address, target_address, target_value
                    ))
                } else {
                    Some(mnemonic_str)
                }
            }
            _ => Some(mnemonic_str),
        }
    }

    fn indirect_y(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let mnemonic_str = String::from(opcode.mnemonic);

        match OpcodeBehaviour::from_mnemonic(opcode.mnemonic) {
            Some(opcode_behaviour) => {
                if let OpcodeBehaviour::Read | OpcodeBehaviour::Write = opcode_behaviour {
                    // Get the byte of this opcode to add as the indirect value - ($FF),Y
                    let base_address = cpu.bus.read(cpu.program_counter.wrapping_add(1));

                    let lo_byte = cpu.bus.read(base_address as u16);
                    let hi_byte = cpu.bus.read(base_address.wrapping_add(1) as u16);

                    let deref_base = (hi_byte as u16) << 8 | (lo_byte as u16);
                    let memory_address = deref_base.wrapping_add(cpu.registers.y as u16);

                    let target_value = cpu.bus.read(memory_address);

                    Some(format!(
                        "{} (${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                        mnemonic_str, base_address, deref_base, memory_address, target_value
                    ))
                } else {
                    Some(mnemonic_str)
                }
            }
            _ => Some(mnemonic_str),
        }
    }

    fn no_address_mode(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        let mnemonic_str = String::from(opcode.mnemonic);

        match OpcodeBehaviour::from_mnemonic(opcode.mnemonic) {
            Some(opcode_behaviour) => {
                if let OpcodeBehaviour::Control = opcode_behaviour {
                    // First address
                    let ptr_lo = cpu.bus.read(cpu.program_counter.wrapping_add(1));
                    let ptr_hi = cpu.bus.read(cpu.program_counter.wrapping_add(2));

                    // Form 16-bit ptr address
                    let ptr = u16::from(ptr_lo) | (u16::from(ptr_hi) << 8);

                    // Get the indirect jump's lo byte of the address
                    let jump_target_lo = cpu.bus.read(ptr);

                    // Build up the high byte of the jump target
                    let hi_address = (ptr & 0xFF00) | ((ptr.wrapping_add(1)) & 0x00FF);
                    let jump_target_hi = cpu.bus.read(hi_address);

                    let jump_target_address =
                        u16::from(jump_target_lo) | (u16::from(jump_target_hi) << 8);

                    Some(format!(
                        "{} (${:04X}) = {:04X}",
                        mnemonic_str, ptr, jump_target_address
                    ))
                } else {
                    Some(mnemonic_str)
                }
            }
            None => Some(mnemonic_str),
        }
    }

    pub fn generate_disassembly(cpu: &Mos6502, opcode: &OpCode) -> Option<String> {
        match opcode.address_mode {
            AddressMode::Accumulator => Some(format!("{} A", opcode.mnemonic)),
            AddressMode::Implied => Some(String::from(opcode.mnemonic)),
            AddressMode::Immediate => Some(format!(
                "{} #${:02X}",
                opcode.mnemonic,
                cpu.bus.read(cpu.program_counter.wrapping_add(1))
            )),
            AddressMode::Relative => Disassembler::relative(cpu, opcode),
            AddressMode::ZeroPage => Disassembler::zeropage(cpu, opcode),
            AddressMode::Absolute => Disassembler::absolute(cpu, opcode),
            AddressMode::ZeroPageX => Disassembler::zeropage_x(cpu, opcode),
            AddressMode::ZeroPageY => Disassembler::zeropage_y(cpu, opcode),
            AddressMode::AbsoluteX => Disassembler::absolute_x(cpu, opcode),
            AddressMode::AbsoluteY => Disassembler::absolute_y(cpu, opcode),
            AddressMode::IndirectX => Disassembler::indirect_x(cpu, opcode),
            AddressMode::IndirectY => Disassembler::indirect_y(cpu, opcode),
            AddressMode::None => Disassembler::no_address_mode(cpu, opcode),
        }
    }
}

#[cfg(test)]
mod test {
    use libnessy::{
        cpus::mos_6502::{address_mode::AddressMode, bus::MemoryBus, cpu::Mos6502, opcode::OpCode},
        interpret_result::InstructionResult,
    };
    use mockall::{mock, predicate};
    use sif::parameterized;

    use crate::integration::nestest::Disassembler;

    const TEST_MNEMONIC: &str = "TEST";
    const LDA_MNEMONIC: &str = "LDA";

    fn create_opcode(mnemonic: &'static str, address_mode: AddressMode) -> OpCode {
        OpCode::new(0xAA, mnemonic, 1, 1, address_mode, |_, _| {
            InstructionResult::Ok
        })
    }

    mock! {
        pub Bus {}

        impl MemoryBus for Bus {
            fn read(&self, address: u16) -> u8;
            fn write(&mut self, address: u16, data: u8);
            fn write_slice(&mut self, start_address: u16, data: &[u8]);

            fn read_u16(&self, address: u16) -> u16;
            fn write_u16(&mut self, address: u16, data: u16);

            fn insert_rom(&mut self, rom: libnessy::roms::ROM);
        }
    }

    #[test]
    fn test_implied() {
        let mock_bus = MockBus::default();

        let cpu = Mos6502::new(Box::new(mock_bus));

        let opcode = create_opcode(TEST_MNEMONIC, AddressMode::Implied);

        let result = Disassembler::generate_disassembly(&cpu, &opcode);

        assert_eq!("TEST", result.unwrap());
    }

    #[test]
    fn test_absolute() {
        let mut mock_bus = MockBus::default();

        mock_bus.expect_read_u16().returning(move |_| 0xC5F5);

        let cpu = Mos6502::new(Box::new(mock_bus));

        let opcode = create_opcode(TEST_MNEMONIC, AddressMode::Absolute);

        let result = Disassembler::generate_disassembly(&cpu, &opcode);

        assert_eq!("TEST $C5F5", result.unwrap());
    }

    #[test]
    fn test_zeropage() {
        let mut mock_bus = MockBus::default();

        #[rustfmt::skip]
        mock_bus
            .expect_read()
            .with(predicate::eq(1))
            .returning(move |_| 0x10);

        mock_bus
            .expect_read()
            .with(predicate::eq(0x10))
            .returning(move |_| 0x41);

        let cpu = Mos6502::new(Box::new(mock_bus));

        let opcode = create_opcode(TEST_MNEMONIC, AddressMode::ZeroPage);

        let result = Disassembler::generate_disassembly(&cpu, &opcode);

        assert_eq!("TEST $10 = 41", result.unwrap());
    }

    #[test]
    fn test_relative() {
        let mut mock_bus = MockBus::default();

        #[rustfmt::skip]
        mock_bus
            .expect_read()
            .with(predicate::eq(0x2))
            .returning(move |_| 0x10);

        let mut cpu = Mos6502::new(Box::new(mock_bus));

        cpu.program_counter = 0x1;

        let opcode = create_opcode(TEST_MNEMONIC, AddressMode::Relative);

        let result = Disassembler::generate_disassembly(&cpu, &opcode);

        assert_eq!("TEST $0013", result.unwrap());
    }

    #[test]
    fn test_immediate() {
        let mut mock_bus = MockBus::default();

        #[rustfmt::skip]
        mock_bus
            .expect_read()
            .with(predicate::eq(0x1))
            .returning(move |_| 0x10);

        let mut cpu = Mos6502::new(Box::new(mock_bus));

        cpu.program_counter = 0x0;

        let opcode = create_opcode(TEST_MNEMONIC, AddressMode::Immediate);

        let result = Disassembler::generate_disassembly(&cpu, &opcode);

        assert_eq!("TEST #$10", result.unwrap());
    }

    #[test]
    fn test_indirect_x() {
        let mut mock_bus = MockBus::default();

        mock_bus
            .expect_read()
            .with(predicate::eq(0x1))
            .returning(move |_| 0x10);

        mock_bus
            .expect_read()
            .with(predicate::eq(0x11))
            .returning(move |_| 0x0);

        mock_bus
            .expect_read()
            .with(predicate::eq(0x12))
            .returning(move |_| 0x02);

        mock_bus
            .expect_read()
            .with(predicate::eq(0x0200))
            .returning(move |_| 0xAA);

        let mut cpu = Mos6502::new(Box::new(mock_bus));

        cpu.program_counter = 0x0;
        cpu.registers.x = 0x1;

        let opcode = create_opcode(LDA_MNEMONIC, AddressMode::IndirectX);

        let result = Disassembler::generate_disassembly(&cpu, &opcode);

        assert_eq!("LDA ($10,X) @ 11 = 0200 = AA", result.unwrap());
    }

    #[test]
    fn test_accumulator_returns_correctly() {
        let cpu = Mos6502::default();

        let opcode = create_opcode(TEST_MNEMONIC, AddressMode::Accumulator);

        assert_eq!(
            Some(String::from("TEST A")),
            Disassembler::generate_disassembly(&cpu, &opcode)
        );
    }

    #[parameterized]
    #[case(AddressMode::None)]
    fn test_inaccessible_methods_return_none(address_mode: AddressMode) {
        let cpu = Mos6502::default();

        let opcode = create_opcode(TEST_MNEMONIC, address_mode);

        assert_eq!(
            Some(String::from("TEST")),
            Disassembler::generate_disassembly(&cpu, &opcode)
        );
    }
}
