use super::cpu::Mos6502;

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Accumulator,
    Relative,
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    None,
}

pub trait MemoryAddressing {
    fn get_address(&self, address_mode: &AddressMode) -> u16;
}

impl MemoryAddressing for Mos6502 {
    fn get_address(&self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            AddressMode::Immediate => self.program_counter,
            AddressMode::ZeroPage => self.bus.read(self.program_counter) as u16,
            AddressMode::ZeroPageX => {
                let pc_address = self.bus.read(self.program_counter);
                pc_address.wrapping_add(self.registers.x) as u16
            }
            AddressMode::ZeroPageY => {
                let pc_address = self.bus.read(self.program_counter);
                pc_address.wrapping_add(self.registers.y) as u16
            }
            AddressMode::Absolute => self.bus.read_u16(self.program_counter),
            AddressMode::AbsoluteX => {
                let pc_address = self.bus.read_u16(self.program_counter);
                pc_address.wrapping_add(self.registers.x as u16)
            }
            AddressMode::AbsoluteY => {
                let pc_address = self.bus.read_u16(self.program_counter);
                pc_address.wrapping_add(self.registers.y as u16)
            }
            AddressMode::IndirectX => {
                let base_address = self.bus.read(self.program_counter);

                let pointer = base_address.wrapping_add(self.registers.x);
                let lo = self.bus.read(pointer as u16);
                let hi = self.bus.read(pointer.wrapping_add(1) as u16);

                (hi as u16) << 8 | (lo as u16)
            }
            AddressMode::IndirectY => {
                let base_address = self.bus.read(self.program_counter);

                let lo = self.bus.read(base_address as u16);
                let hi = self.bus.read((base_address).wrapping_add(1) as u16);

                let dereference_base = (hi as u16) << 8 | (lo as u16);
                dereference_base.wrapping_add(self.registers.y as u16)
            }
            AddressMode::None
            | AddressMode::Implied
            | AddressMode::Relative
            | &AddressMode::Accumulator => {
                panic!("Unsupported address mode: {:?}", &address_mode)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sif::parameterized;

    use super::*;
    use crate::cpus::mos_6502::cpu::Registers;
    use crate::cpus::mos_6502::instruction_set::helpers::Helpers;
    use crate::cpus::mos_6502::{address_mode::MemoryAddressing, cpu::Mos6502};

    #[parameterized]
    #[case(AddressMode::None)]
    #[case(AddressMode::Implied)]
    #[case(AddressMode::Relative)]
    #[case(AddressMode::Accumulator)]
    #[should_panic]
    fn test_get_address_with_none_return_err(address_mode: AddressMode) {
        Mos6502::default().get_address(&address_mode);
    }

    #[parameterized]
    #[case(0xFF, None, None, AddressMode::Immediate, 0xFF)]
    #[case(0x10, Some(vec![(0x10, 0xAA)]), None, AddressMode::ZeroPage, 0xAA)]
    #[case(0x05, Some(vec![(0x05, 0xBB)]), Some(Registers{a:0, x:0x01, y:0}), AddressMode::ZeroPageX, 0xBC)]
    #[case(0xAA, Some(vec![(0xAA, 0xC5)]), Some(Registers{a:0, x:0, y:0x01}), AddressMode::ZeroPageY, 0xC6)]
    #[case(0xFF, Some(vec![(0xAA, 0x02), (0x02, 0x01)]), None, AddressMode::Immediate, 0xFF)]
    #[case(0xAA, Some(vec![(0xAA, 0x34), (0xAB, 0x12)]), None, AddressMode::Absolute, 0x1234)]
    #[case(0xAA, Some(vec![(0xAA, 0x34), (0xAB, 0x12)]), Some(Registers{a:0, x:0x01, y:0}), AddressMode::AbsoluteX, 0x1235)]
    #[case(0xAA, Some(vec![(0xAA, 0x34), (0xAB, 0x12)]), Some(Registers{a:0, x:0, y:0x01}), AddressMode::AbsoluteY, 0x1235)]
    #[case(0xAA, Some(vec![(0xAA, 0x13), (0x14, 0xFC), (0x15, 0xBA)]), Some(Registers{a:0, x:0x1, y:0}), AddressMode::IndirectX, 0xBAFC)]
    #[case(0xAA, Some(vec![(0xAA, 0x50), (0x50, 0xFB), (0x51, 0xFF)]), Some(Registers{a:0, x:0, y:0x1}), AddressMode::IndirectY, 0xFFFC)]
    fn test_get_address_with_valid(
        program_counter: u16,
        memory: Option<Vec<(u16, u8)>>,
        registers: Option<Registers>,
        address_mode: AddressMode,
        expected_result: u16,
    ) {
        let cpu = Helpers::create_cpu(program_counter, 0x0, memory, registers, None);

        let result = cpu.get_address(&address_mode);

        assert_eq!(expected_result, result);
    }
}
