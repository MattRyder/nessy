use bitflags::bitflags;

bitflags! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Flags: u8 {
    const CARRY = 0b0000_0001;
    const ZERO = 0b0000_0010;
    const INTERRUPT_DISABLE = 0b0000_0100;
    const DECIMAL_MODE = 0b0000_1000;
    const BREAK_COMMAND = 0b001_0000;
    const OVERFLOW = 0b0100_0000;
    const NEGATIVE = 0b1000_0000;
}
}

#[derive(Debug)]
pub struct Status {
    pub flags: Flags,
}

impl Default for Status {
    fn default() -> Self {
        Status {
            flags: Flags::empty(),
        }
    }
}

impl Status {
    pub fn set_flags_for_result(&mut self, result: u8) {
        if result == 0 {
            self.flags |= Flags::ZERO;
        } else {
            self.flags &= !Flags::ZERO;
        }

        if result & 0b1000_0000 != 0 {
            self.flags |= Flags::NEGATIVE;
        } else {
            self.flags &= !Flags::NEGATIVE;
        }
    }
}
