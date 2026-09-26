use bitflags::bitflags;

// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: greyscale)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Enable background rendering
// |||+------ 1: Enable sprite rendering
// ||+------- Emphasize red (green on PAL/Dendy)
// |+-------- Emphasize green (red on PAL/Dendy)
// +--------- Emphasize blue
// source: https://www.nesdev.org/wiki/PPU_registers#PPUMASK_-_Rendering_settings_($2001_write)
bitflags! {
    #[derive(Default, Debug)]
    pub struct MaskRegister : u8 {
        const GRAYSCALE = 1 << 0;
        const SHOW_LEFTMOST_BACKGROUND = 1 << 1;
        const SHOW_LEFTMOST_SPRITES = 1 << 2;
        const ENABLE_BACKGROUND_RENDERING = 1 << 3;
        const ENABLE_SPRITE_RENDERING = 1 << 4;
        const EMPHASISE_RED = 1 << 5;
        const EMPHASISE_GREEN = 1 << 6;
        const EMPHASISE_BLUE = 1 << 7;
    }
}

impl MaskRegister {
    pub fn update(&mut self, value: u8) {
        *self = MaskRegister::from_bits_retain(value);
    }

    pub fn grayscale(&self) -> bool {
        self.contains(Self::GRAYSCALE)
    }

    pub fn show_leftmost_background(&self) -> bool {
        self.contains(Self::SHOW_LEFTMOST_BACKGROUND)
    }

    pub fn show_leftmost_sprites(&self) -> bool {
        self.contains(Self::SHOW_LEFTMOST_SPRITES)
    }

    pub fn enable_background_rendering(&self) -> bool {
        self.contains(Self::ENABLE_BACKGROUND_RENDERING)
    }

    pub fn enable_sprite_rendering(&self) -> bool {
        self.contains(Self::ENABLE_SPRITE_RENDERING)
    }

    pub fn emphasise_red(&self) -> bool {
        self.contains(Self::EMPHASISE_RED)
    }

    pub fn emphasise_blue(&self) -> bool {
        self.contains(Self::EMPHASISE_BLUE)
    }

    pub fn emphasise_green(&self) -> bool {
        self.contains(Self::EMPHASISE_GREEN)
    }
}

#[cfg(test)]
mod test {
    use crate::ppu::mask_register::MaskRegister;

    #[test]
    fn test_update() {
        let registers = MaskRegister::GRAYSCALE | MaskRegister::EMPHASISE_GREEN;
        let mut sut = MaskRegister::default();
        sut.update(registers.bits());
        assert_eq!(registers.bits(), sut.bits());
    }

    #[test]
    fn test_grayscale() {
        let sut = MaskRegister::GRAYSCALE;
        assert!(sut.grayscale());
    }

    #[test]
    fn test_show_leftmost_background() {
        let sut = MaskRegister::SHOW_LEFTMOST_BACKGROUND;
        assert!(sut.show_leftmost_background());
    }

    #[test]
    fn test_show_leftmost_sprites() {
        let sut = MaskRegister::SHOW_LEFTMOST_SPRITES;
        assert!(sut.show_leftmost_sprites());
    }

    #[test]
    fn test_enable_background_rendering() {
        let sut = MaskRegister::ENABLE_BACKGROUND_RENDERING;
        assert!(sut.enable_background_rendering());
    }

    #[test]
    fn test_enable_sprite_rendering() {
        let sut = MaskRegister::ENABLE_SPRITE_RENDERING;
        assert!(sut.enable_sprite_rendering());
    }

    #[test]
    fn test_emphasise_red() {
        let sut = MaskRegister::EMPHASISE_RED;
        assert!(sut.emphasise_red());
    }

    #[test]
    fn test_emphasise_green() {
        let sut = MaskRegister::EMPHASISE_GREEN;
        assert!(sut.emphasise_green());
    }

    #[test]
    fn test_emphasise_blue() {
        let sut = MaskRegister::EMPHASISE_BLUE;
        assert!(sut.emphasise_blue());
    }
}
