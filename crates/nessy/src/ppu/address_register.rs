// Bit mask for mirroring down to the 0x0-0x3FFF
// range for the value in the register
const MIRROR_BIT_MASK: u16 = 0b11111111111111;

// The value before the address value must be
// masked down to the 0-MAX range
const VALUE_MAX: u16 = 0x3FFF;

/// Used by the PPU to form the Address register 0x2006
#[derive(Default, Debug)]
pub struct AddressRegister {
    // hi byte - lo byte
    value: (u8, u8),
    // if the register is pointing to the hi byte or not
    hi_ptr: bool,
}

impl AddressRegister {
    pub fn get(&self) -> u16 {
        (self.value.0 as u16) << 8 | (self.value.1 as u16)
    }

    pub fn reset(&mut self) {
        self.hi_ptr = false;
    }

    // Sets the data in the hi/lo byte depending on hi_ptr state
    // then checks if the value is over 0x3FFF, if so, mirrors it down
    // then toggles hi_ptr for the next call.
    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        self.try_mirror();

        // flip latch
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, increment_value: u8) {
        let last_lo = self.value.1;

        self.value.1 = self.value.1.wrapping_add(increment_value);

        if last_lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }

        self.try_mirror();
    }

    // If the value is over the VALUE_MAX then we wrap it back to 0, 0
    // This function compares the current value and performs the wrap if needed.
    fn try_mirror(&mut self) {
        let current_value = self.get();

        // check value, mirror down if greater
        if current_value > VALUE_MAX {
            self.set(current_value & MIRROR_BIT_MASK);
        }
    }

    // Sets a 16-bit value into the register, in big endian.
    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xFF) as u8;
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use crate::ppu::address_register::AddressRegister;

    #[test]
    fn get_returns_the_correct_value() {
        let sut = AddressRegister {
            value: (0x50, 0x51),
            ..Default::default()
        };

        assert_eq_hex!(sut.get(), 0x5051);
        assert!(!sut.hi_ptr);
    }

    #[test]
    fn set_updates_the_value() {
        let mut sut = AddressRegister::default();

        sut.set(0x4142);

        assert_eq_hex!(sut.value, (0x41, 0x42));
        assert!(!sut.hi_ptr);
    }

    #[test]
    fn reset_sets_the_hi_ptr_to_false() {
        let mut sut = AddressRegister {
            hi_ptr: true,
            ..Default::default()
        };

        sut.reset();

        assert!(!sut.hi_ptr);
    }

    #[parameterized]
    #[case(false, (0, 0), 1, (0, 1))]
    #[case(true, (0, 0), 2, (2, 0))]
    #[case(true, (0, 0xFF), 0x40, (0, 0xFF))]
    fn update_sets_internal_state(
        hi_ptr: bool,
        init_value: (u8, u8),
        data: u8,
        expected_value: (u8, u8),
    ) {
        let mut sut = AddressRegister {
            value: init_value,
            hi_ptr,
        };

        sut.update(data);

        assert_eq!(sut.hi_ptr, !hi_ptr);
        assert_eq_hex!(sut.value, expected_value);
    }

    #[parameterized]
    #[case((0, 0), 1, (0, 1))]
    #[case((0, 0xFF), 1, (1, 0))]
    #[case((0x3F, 0xFF), 1, (0, 0))]
    fn increment_increments_the_value(
        init_value: (u8, u8),
        increment_value: u8,
        expected_value: (u8, u8),
    ) {
        let mut sut = AddressRegister {
            value: init_value,
            ..Default::default()
        };

        sut.increment(increment_value);

        assert_eq_hex!(sut.value, expected_value);
    }
}
