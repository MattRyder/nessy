use bitflags::bitflags;

use crate::cpus::mos_6502::instruction_set::helpers::MSB_MASK;

bitflags! {
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Flags: u8 {
    const CARRY = 0b0000_0001;
    const ZERO = 0b0000_0010;
    const INTERRUPT_DISABLE = 0b0000_0100;
    const DECIMAL_MODE = 0b0000_1000;
    const BREAK_COMMAND = 0b001_0000;
    const UNUSED = 0b0010_0000; // Not used, should be set high.
    const OVERFLOW = 0b0100_0000;
    const NEGATIVE = 0b1000_0000;
}
}

impl Flags {
    pub fn set_status_flag(&mut self, flag: Flags, predicate: bool) {
        self.set(flag, predicate);
    }

    pub fn set_zero_flag(&mut self, value: u8) {
        self.set_status_flag(Flags::ZERO, value == 0);
    }

    pub fn set_negative_flag(&mut self, value: u8) {
        self.set_status_flag(Flags::NEGATIVE, value & MSB_MASK != 0);
    }
}
