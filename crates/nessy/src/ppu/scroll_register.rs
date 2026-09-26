#[derive(Default, Debug)]
pub struct ScrollRegister {
    x: u8,
    y: u8,
    latch: bool,
}

impl ScrollRegister {
    pub fn update(&mut self, value: u8) {
        if !self.latch {
            self.x = value;
        } else {
            self.y = value;
        }

        self.latch = !self.latch;
    }

    pub fn reset(&mut self) {
        self.latch = false;
    }
}

#[cfg(test)]
mod test {
    use crate::ppu::scroll_register::ScrollRegister;

    #[test]
    fn test_update_for_x() {
        let mut sut = ScrollRegister {
            latch: false,
            ..Default::default()
        };

        sut.update(0x55);

        assert_eq!(0x55, sut.x);
    }

    #[test]
    fn test_update_for_y() {
        let mut sut = ScrollRegister {
            latch: true,
            ..Default::default()
        };

        sut.update(0x55);

        assert_eq!(0x55, sut.y);
    }

    #[test]
    fn test_reset() {
        let mut sut = ScrollRegister {
            latch: true,
            ..Default::default()
        };
        sut.reset();
        assert!(!sut.latch);
    }
}
