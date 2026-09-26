use bitflags::bitflags;

const VRAM_INCREMENT_ACROSS: u8 = 1;
const VRAM_INCREMENT_DOWN: u8 = 32;

// 7  bit  0
// ---- ----
// VPHB SINN
// |||| ||||
// |||| ||++- Base nametable address
// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
// |||| |     (0: add 1, going across; 1: add 32, going down)
// |||| +---- Sprite pattern table address for 8x8 sprites
// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
// |||+------ Background pattern table address (0: $0000; 1: $1000)
// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
// |+-------- PPU master/slave select
// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
// +--------- Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
// source: https://www.nesdev.org/wiki/PPU_registers#PPUCTRL_-_Miscellaneous_settings_($2000_write)
bitflags! {
    #[derive(Default, Debug, PartialEq)]
    pub struct ControlRegister: u8 {
        const NAMETABLE1                    = 1 << 0;
        const NAMETABLE2                    = 1 << 1;
        const VRAM_INCREMENT                = 1 << 2;
        const SPRITE_PATTERN_ADDRESS        = 1 << 3;
        const BACKGROUND_PATTERN_ADDRESS    = 1 << 4;
        const SPRITE_SIZE                   = 1 << 5;
        const MASTER_SLAVE_SELECT           = 1 << 6;
        const GENERATE_NMI                  = 1 << 7;
    }
}

impl ControlRegister {
    pub fn vram_increment_size(&self) -> u8 {
        if self.contains(ControlRegister::VRAM_INCREMENT) {
            VRAM_INCREMENT_DOWN
        } else {
            VRAM_INCREMENT_ACROSS
        }
    }

    pub fn update(&mut self, data: u8) {
        *self = ControlRegister::from_bits_retain(data);
    }
}

#[cfg(test)]
mod test {
    use sif::parameterized;

    use crate::ppu::control_register::{
        ControlRegister, VRAM_INCREMENT_ACROSS, VRAM_INCREMENT_DOWN,
    };

    #[parameterized]
    #[case(ControlRegister::empty(), VRAM_INCREMENT_ACROSS)]
    #[case(ControlRegister::VRAM_INCREMENT, VRAM_INCREMENT_DOWN)]
    fn vram_increment_size_returns_the_correct_size(register: ControlRegister, size: u8) {
        assert_eq!(register.vram_increment_size(), size);
    }

    #[test]
    fn update_updates_value() {
        let mut sut = ControlRegister::empty();
        sut.update(ControlRegister::VRAM_INCREMENT.bits());
        assert_eq!(ControlRegister::VRAM_INCREMENT, sut);
    }
}
