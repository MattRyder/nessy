use bitflags::bitflags;

// 7  bit  0
// ---- ----
// VSOx xxxx
// |||| ||||
// |||+-++++- (PPU open bus or 2C05 PPU identifier)
// ||+------- Sprite overflow flag
// |+-------- Sprite 0 hit flag (an oam sprite is colliding with the background)
// +--------- Vblank flag, cleared on read. Unreliable; see below.
// source: https://www.nesdev.org/wiki/PPU_registers#PPUSTATUS_-_Rendering_events_($2002_read)
bitflags! {
#[derive(Default, Debug)]
pub struct StatusRegister: u8 {
        const UNUSED_0 = 1 << 0;
        const UNUSED_1 = 1 << 1;
        const UNUSED_2 = 1 << 2;
        const UNUSED_3 = 1 << 3;
        const UNUSED_4 = 1 << 4;
        const SPRITE_OVERFLOW = 1 << 5;
        const SPRITE_ZERO_HIT = 1 << 6;
        const VBLANK_STARTED = 1 << 7;
    }
}

impl StatusRegister {
    pub fn set_sprite_overflow(&mut self, value: bool) {
        self.set(Self::SPRITE_OVERFLOW, value);
    }

    pub fn set_sprite_zero_hit(&mut self, value: bool) {
        self.set(Self::SPRITE_ZERO_HIT, value);
    }

    pub fn set_vblank_started(&mut self, value: bool) {
        self.set(Self::VBLANK_STARTED, value);
    }

    pub fn reset_vblank(&mut self) {
        self.set(Self::VBLANK_STARTED, false);
    }

    pub fn read(&self) -> u8 {
        self.bits()
    }
}

#[cfg(test)]
mod test {
    use crate::ppu::status_register::StatusRegister;

    #[test]
    fn test_set_sprite_overflow() {
        let mut sut = StatusRegister::default();
        sut.set_sprite_overflow(true);
        assert!(sut.contains(StatusRegister::SPRITE_OVERFLOW));
    }

    #[test]
    fn test_set_sprite_zero_hit() {
        let mut sut = StatusRegister::default();
        sut.set_sprite_zero_hit(true);
        assert!(sut.contains(StatusRegister::SPRITE_ZERO_HIT));
    }

    #[test]
    fn test_vblank_started() {
        let mut sut = StatusRegister::default();
        sut.set_vblank_started(true);
        assert!(sut.contains(StatusRegister::VBLANK_STARTED));
    }

    #[test]
    fn test_reset_vblank() {
        let mut sut = StatusRegister::from_bits_truncate(StatusRegister::VBLANK_STARTED.bits());
        sut.reset_vblank();
        assert!(!sut.contains(StatusRegister::VBLANK_STARTED));
    }
}
