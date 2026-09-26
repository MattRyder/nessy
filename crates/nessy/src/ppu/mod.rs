const PALETTE_TABLE_SIZE: usize = 32;
const PPU_VRAM_SIZE: usize = 2048;
const OBJECT_ATTR_MEMORY_SIZE: usize = 256;

// Used to mirror down the given address to the 0x2000-0x2FFF range
const VRAM_MIRRORING_MASK: u16 = 0x2FFF;

use crate::{
    ppu::{
        address_register::AddressRegister, control_register::ControlRegister,
        mask_register::MaskRegister, scroll_register::ScrollRegister,
        status_register::StatusRegister,
    },
    roms::mirroring::Mirroring,
};

pub mod address_register;
pub mod control_register;
pub mod mask_register;
pub mod scroll_register;
pub mod status_register;

#[derive(Debug, PartialEq)]
pub enum ReadError {
    // An attempt was made to read from an unused region of PPU memory.
    UnusedMemoryRegion,
    // An attempt was made to read from the mirrored region of palette memory.
    MirroredRegionRead,
}

#[derive(Debug, PartialEq)]
pub enum WriteError {
    // An attempt was made to write to Character ROM.
    WriteToCharacterRom,
    // An attempt was made to access the unused memory region.
    UnusedMemoryRegion,
    // An attempt was made to write to an unsupported memory region.
    UnexpectedWriteToMirroredSpace,
}

pub trait PictureProcessingUnitSystem {
    fn address_write(&mut self, value: u8);
    fn control_write(&mut self, value: u8);
    fn mask_write(&mut self, value: u8);
    fn oam_addr_write(&mut self, value: u8);
    fn oam_data_write(&mut self, value: u8);
    fn scroll_write(&mut self, value: u8);
    fn write(&mut self, value: u8) -> Result<(), WriteError>;
    fn increment_vram_address(&mut self);

    fn read(&mut self) -> Result<u8, ReadError>;
    fn oam_data_read(&self) -> u8;
    fn status_read(&mut self) -> u8;
}

#[derive(Debug)]
pub struct PPU {
    address_register: AddressRegister,
    control_register: ControlRegister,
    status_register: StatusRegister,
    mask_register: MaskRegister,
    scroll_register: ScrollRegister,
    character_rom: Vec<u8>,
    mirroring: Mirroring,
    palette_table: [u8; PALETTE_TABLE_SIZE],
    vram: [u8; PPU_VRAM_SIZE],

    object_attribute_memory_address: u8,
    object_attribute_memory: [u8; OBJECT_ATTR_MEMORY_SIZE],

    // As the CHR_ROM and PAL_TABLE are seen as external to the PPU
    // there's a buffer on the IC that means that for a value to be
    // correctly read, it needs to be read twice. Once to pull from
    // the bus, then one to read out of the internal buffer.
    internal_data_buffer: u8,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            address_register: Default::default(),
            control_register: Default::default(),
            status_register: Default::default(),
            mask_register: Default::default(),
            scroll_register: Default::default(),
            character_rom: Default::default(),
            mirroring: Mirroring::Horizontal,
            palette_table: Default::default(),
            vram: [0; PPU_VRAM_SIZE],
            object_attribute_memory_address: 0,
            object_attribute_memory: [0; OBJECT_ATTR_MEMORY_SIZE],
            internal_data_buffer: Default::default(),
        }
    }
}

impl PPU {
    pub fn new(character_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Self {
            character_rom,
            mirroring,
            ..Default::default()
        }
    }

    fn mirror_vram_address(&self, address: u16) -> u16 {
        const VRAM_BANK_SIZE: u16 = 0x400;

        // Mask any <= 0x3000 address down to the 0x2000-0x2FFF range
        let mirrored_down_addr = address & VRAM_MIRRORING_MASK;

        // Map the logical address space address down to a "physical" VRAM address.
        let offset = mirrored_down_addr - 0x2000;

        // Calculate the nametable index, clockwise starting 2000h.
        let nametable_index = offset / VRAM_BANK_SIZE;

        let inner_addr = offset % VRAM_BANK_SIZE;

        // Return the address for the given mirroring.
        // horizontal mirroring | vertical mirroring
        // [0] [1]              | [0] [2]
        // [2] [3]              | [1] [3]
        let physical_table = match &self.mirroring {
            Mirroring::Horizontal => nametable_index / 2,
            Mirroring::Vertical => nametable_index % 2,
            _ => panic!("Mirroring unsupported at this time."),
        };

        physical_table * VRAM_BANK_SIZE + inner_addr
    }
}

impl PictureProcessingUnitSystem for PPU {
    fn address_write(&mut self, data: u8) {
        self.address_register.update(data);
    }

    fn control_write(&mut self, data: u8) {
        self.control_register.update(data);
    }

    fn mask_write(&mut self, value: u8) {
        self.mask_register.update(value);
    }

    fn oam_addr_write(&mut self, value: u8) {
        self.object_attribute_memory_address = value;
    }

    fn oam_data_write(&mut self, value: u8) {
        self.object_attribute_memory[self.object_attribute_memory_address as usize] = value;
        self.object_attribute_memory_address = self.object_attribute_memory_address.wrapping_add(1);
    }

    fn scroll_write(&mut self, value: u8) {
        self.scroll_register.update(value);
    }

    fn write(&mut self, value: u8) -> Result<(), WriteError> {
        let address = self.address_register.get();

        let result = match address {
            0..=0x1FFF => Err(WriteError::WriteToCharacterRom),

            // Write to the VRAM
            0x2000..=0x2FFF => {
                let mirrored_vram_addr = self.mirror_vram_address(address);
                println!("Writing from {:04X} to {:04X}", address, mirrored_vram_addr);
                self.vram[mirrored_vram_addr as usize] = value;
                Ok(())
            }

            0x3000..=0x3EFF => Err(WriteError::UnexpectedWriteToMirroredSpace),

            // n.b. the nes ebook doesn't handle mirroring, weirdly. this does,
            // which is why it looks different.
            0x3F00..=0x3FFF => {
                let mut palette_table_vram_index = (address - 0x3F00) as usize;

                // Mirror the 0x3F20 - 0x3FFF range, after the "correct" addrs.
                palette_table_vram_index %= 32;

                // Handle the four mirrored addresses for the universal bg colours
                // 0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C
                if palette_table_vram_index >= 0x10 && palette_table_vram_index.is_multiple_of(4) {
                    palette_table_vram_index -= 0x10;
                }

                self.palette_table[palette_table_vram_index] = value;

                Ok(())
            }

            _ => todo!(),
        };

        self.increment_vram_address();

        result
    }

    fn increment_vram_address(&mut self) {
        self.address_register
            .increment(self.control_register.vram_increment_size());
    }

    fn read(&mut self) -> Result<u8, ReadError> {
        let address = self.address_register.get();
        self.increment_vram_address();

        match address {
            // Read from the character ROM.
            0..=0x1FFF => {
                let previous_buffer_value = self.internal_data_buffer;
                self.internal_data_buffer = self.character_rom[address as usize];
                Ok(previous_buffer_value)
            }
            // Read from the VRAM.
            0x2000..=0x2FFF => {
                let previous_buffer_value = self.internal_data_buffer;
                self.internal_data_buffer = self.vram[self.mirror_vram_address(address) as usize];
                Ok(previous_buffer_value)
            }
            // This part of the address space goes unused.
            0x3000..=0x3EFF => Err(ReadError::UnusedMemoryRegion),
            // Read from the palette_table.
            0x3F00..=0x3FFF => Ok(self.palette_table[(address - 0x3F00) as usize]),
            // Otherwise, it's <0x4000, so invalid read into the mirror.
            _ => Err(ReadError::MirroredRegionRead),
        }
    }

    fn oam_data_read(&self) -> u8 {
        self.object_attribute_memory[self.object_attribute_memory_address as usize]
    }

    fn status_read(&mut self) -> u8 {
        let register_value = self.status_register.bits();
        self.status_register.reset_vblank();
        self.address_register.reset();
        self.scroll_register.reset();
        register_value
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use sif::parameterized;

    use crate::{
        ppu::{
            OBJECT_ATTR_MEMORY_SIZE, PPU, PictureProcessingUnitSystem, ReadError, WriteError,
            address_register::AddressRegister, control_register::ControlRegister,
            mask_register::MaskRegister, status_register::StatusRegister,
        },
        roms::mirroring::Mirroring,
    };

    #[test]
    fn address_write_updates_register() {
        let mut ppu = PPU::default();
        ppu.address_write(0x41);
        assert_eq!(0x41, ppu.address_register.get());
    }

    #[test]
    fn control_write_updates_register() {
        let expected_flags = ControlRegister::MASTER_SLAVE_SELECT | ControlRegister::NAMETABLE1;

        let mut ppu = PPU::default();
        ppu.control_write(expected_flags.bits());

        assert_eq!(expected_flags, ppu.control_register);
    }

    #[test]
    fn increment_vram_address_updates_vram_increment() {
        let mut ppu = PPU::default();

        ppu.control_register
            .update(ControlRegister::VRAM_INCREMENT.bits());

        ppu.increment_vram_address();

        assert_eq!(32, ppu.address_register.get());
    }

    #[test]
    fn read_into_unused_region_returns_error() {
        let mut sut = PPU::default();
        sut.address_register.update(0x00);
        sut.address_register.update(0x30);

        let result = sut.read();
        assert!(result.is_err());
        assert_eq!(ReadError::UnusedMemoryRegion, result.err().unwrap());
    }

    #[test]
    fn read_pulls_byte_from_palette_table() {
        let mut palette_table = [0u8; 32];
        palette_table[0x05] = 0x41;

        let mut sut = PPU {
            palette_table,
            ..Default::default()
        };

        sut.address_write(0x05);
        sut.address_write(0x3F);

        let result = sut.read();

        assert!(result.is_ok());
        assert_eq_hex!(0x41, result.ok().unwrap());
        assert_eq_hex!(0x3F06, sut.address_register.get());
    }

    #[test]
    fn read_pulls_byte_from_buffer_then_character_rom() {
        let mut chr_rom = vec![0x41; 0x60];
        chr_rom[0x55] = 0xAA;

        let mut sut = PPU::new(chr_rom, Mirroring::Horizontal);
        sut.address_write(0x55);

        let result = sut.read();

        // Read the "dead" byte out from the buffer
        assert_eq_hex!(0, result.ok().unwrap());

        let result = sut.read();

        assert_eq_hex!(0xAA, result.ok().unwrap());
        assert_eq_hex!(0x57, sut.address_register.get());
    }

    #[parameterized]
    // Check the basics are working
    #[case(Mirroring::Horizontal, 0x2001, 0x1)]
    #[case(Mirroring::Vertical, 0x2001, 0x1)]
    #[case(Mirroring::Horizontal, 0x2C55, 0x455)]
    #[case(Mirroring::Vertical, 0x2C55, 0x455)]
    #[case(Mirroring::Horizontal, 0x2424, 0x24)]
    #[case(Mirroring::Vertical, 0x2424, 0x424)]
    #[case(Mirroring::Horizontal, 0x3005, 0x5)]
    fn mirror_vram_address_returns_correct_address(
        mirroring: Mirroring,
        address: u16,
        expected: u16,
    ) {
        let sut = PPU {
            mirroring,
            ..Default::default()
        };

        assert_eq_hex!(expected, sut.mirror_vram_address(address));
    }

    #[test]
    fn status_read_returns_correct_value() {
        let status_register = StatusRegister::VBLANK_STARTED;

        let mut sut = PPU {
            status_register,
            ..Default::default()
        };

        assert_eq!(StatusRegister::VBLANK_STARTED.bits(), sut.status_read());
    }

    #[test]
    fn mask_write_updates_register() {
        let mask_register = MaskRegister::default();

        let mut sut = PPU {
            mask_register,
            ..Default::default()
        };

        sut.mask_write(MaskRegister::ENABLE_BACKGROUND_RENDERING.bits());

        assert_eq!(
            MaskRegister::ENABLE_BACKGROUND_RENDERING.bits(),
            sut.mask_register.bits()
        );
    }

    #[test]
    fn test_oam_addr_write_sets_correct_address() {
        let mut sut = PPU::default();
        sut.oam_addr_write(0x41);
        assert_eq!(0x41, sut.object_attribute_memory_address);
    }

    #[test]
    fn test_oam_data_write_sets_correct_data() {
        let mut sut = PPU {
            object_attribute_memory_address: 0x55,
            ..Default::default()
        };
        sut.oam_data_write(0x41);
        assert_eq!(0x41, sut.object_attribute_memory[0x55]);
    }

    #[test]
    fn test_oam_data_read_returns_correct_data() {
        let mut object_attribute_memory = [0; OBJECT_ATTR_MEMORY_SIZE];
        object_attribute_memory[0x55] = 0xAA;

        let sut = PPU {
            object_attribute_memory_address: 0x55,
            object_attribute_memory,
            ..Default::default()
        };

        assert_eq_hex!(0xAA, sut.oam_data_read());
    }

    #[parameterized]
    #[case(0x0)]
    #[case(0x1000)]
    #[case(0x1FFF)]
    fn test_write_to_chr_rom_returns_err(address: u16) {
        let mut address_register = AddressRegister::default();

        address_register.update((address & 0xFF) as u8);
        address_register.update((address >> 8 & 0xFF) as u8);

        let mut sut = PPU {
            address_register,
            ..Default::default()
        };

        let result = sut.write(0x41);
        assert!(result.is_err());
        assert_eq!(WriteError::WriteToCharacterRom, result.unwrap_err());
    }

    #[parameterized]
    #[case(0x3000)]
    #[case(0x3AAA)]
    #[case(0x3EFF)]
    fn test_write_to_mirror_returns_err(address: u16) {
        let mut address_register = AddressRegister::default();

        address_register.update((address & 0xFF) as u8);
        address_register.update((address >> 8 & 0xFF) as u8);

        let mut sut = PPU {
            address_register,
            ..Default::default()
        };

        let result = sut.write(0x41);
        assert!(result.is_err());
        assert_eq!(
            WriteError::UnexpectedWriteToMirroredSpace,
            result.unwrap_err()
        );
    }

    #[parameterized]
    #[case(0x2000, 0)]
    #[case(0x2AAA, 0x6AA)]
    #[case(0x2FFF, 0x7FF)]
    fn test_write_to_vram_sets_value(ppu_address: u16, vram_address: usize) {
        let mut address_register = AddressRegister::default();

        address_register.update((ppu_address & 0xFF) as u8);
        address_register.update((ppu_address >> 8 & 0xFF) as u8);

        let mut sut = PPU {
            address_register,
            ..Default::default()
        };

        let result = sut.write(0x41);
        assert!(result.is_ok());

        assert_eq!(0x41, sut.vram[vram_address]);
    }

    #[parameterized]
    #[case(0x3F10, 0x3F00)]
    #[case(0x3F14, 0x3F04)]
    #[case(0x3F18, 0x3F08)]
    #[case(0x3F1C, 0x3F0C)]
    fn test_write_to_mirrored_pallete_addr_writes_correctly(
        pal_mirror_addr: u16,
        pal_table_addr: usize,
    ) {
        let mut address_register = AddressRegister::default();

        address_register.update((pal_mirror_addr & 0xFF) as u8);
        address_register.update((pal_mirror_addr >> 8 & 0xFF) as u8);

        let mut sut = PPU {
            address_register,
            ..Default::default()
        };

        let result = sut.write(0x41);
        assert!(result.is_ok());

        assert_eq!(0x41, sut.palette_table[pal_table_addr - 0x3F00]);
    }

    #[parameterized]
    #[case(0x3F00)]
    #[case(0x3FAA)]
    #[case(0x3FFF)]
    fn test_write_with_pallete_addr_writes_correctly(ppu_address: u16) {
        let mut address_register = AddressRegister::default();

        address_register.update((ppu_address & 0xFF) as u8);
        address_register.update((ppu_address >> 8 & 0xFF) as u8);

        let mut sut = PPU {
            address_register,
            ..Default::default()
        };

        let expected_table_index = ((ppu_address - 0x3F00) % 32) as usize;

        let result = sut.write(0x41);
        assert!(result.is_ok());

        assert_eq!(0x41, sut.palette_table[expected_table_index]);
    }
}
