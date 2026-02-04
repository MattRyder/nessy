use crate::roms::{ROM, mirroring::Mirroring};

// The NES magic - NES^Z
const NES_MAGIC: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

// Size of a 16KB block for PRG ROM data.
const PRG_ROM_BLOCK_SIZE_KB: usize = 16384;

// Size of an 8KB block for CHR ROM data.
const CHR_ROM_BLOCK_SIZE_KB: usize = 8192;

// Size of the header in bytes.
const HEADER_SIZE_BYTES: usize = 16;

// Size of the trainer (if present), in bytes.
const TRAINER_SIZE_BYTES: usize = 512;

// The collective info gleaned from F6 and F7 of the header bytes.
#[derive(Debug, PartialEq)]
pub struct Metadata {
    mapper: u8,
    mirroring: Mirroring,
    has_trainer: bool,
    has_battery_ram: bool,
    version: u8,
}

pub struct Loader {}

impl Loader {
    pub fn parse_metadata(flag_byte_6: u8, flag_byte_7: u8) -> Metadata {
        let mapper = (flag_byte_7 & 0b1111_0000) | (flag_byte_6 >> 4);

        // Alt NT layout is varied use, but typically to indicate a 4screen variation.
        let alternative_nametable_layout = flag_byte_6 & 0b1000 != 0;

        let has_trainer = flag_byte_6 & 0b0100 != 0;

        let has_battery_ram = flag_byte_6 & 0b0010 != 0;

        // Whether there's a vertical or Horizontal nametable arrangement in use.
        let nametable_arrangement = flag_byte_6 & 0b1 != 0;

        let mirroring = match (alternative_nametable_layout, nametable_arrangement) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        // Whether we're loading an iNES v1 or v2 format ROM.
        let version = (flag_byte_7 >> 2) & 0b11;

        Metadata {
            mapper,
            mirroring,
            has_trainer,
            has_battery_ram,
            version,
        }
    }

    pub fn load(data: &[u8]) -> Result<ROM, &str> {
        if data[0..4] != NES_MAGIC {
            return Err("Incorrect NES magic in ROM.");
        }

        let prg_rom_length = data[4] as usize * PRG_ROM_BLOCK_SIZE_KB;
        let chr_rom_length = data[5] as usize * CHR_ROM_BLOCK_SIZE_KB;

        let metadata = Self::parse_metadata(data[6], data[7]);

        if metadata.version != 1 {
            return Err("Cannot load the iNES version 2. Please use iNES version 1.");
        }

        let prg_rom_trainer_offset = if metadata.has_trainer {
            TRAINER_SIZE_BYTES
        } else {
            0
        };

        let prg_rom_start = HEADER_SIZE_BYTES + prg_rom_trainer_offset;
        let prg_rom_end = prg_rom_start + prg_rom_length;

        let chr_rom_start = prg_rom_start + prg_rom_length;
        let chr_rom_end = chr_rom_start + chr_rom_length;

        let prg_rom = data[prg_rom_start..prg_rom_end].to_vec();
        let chr_rom = data[chr_rom_start..chr_rom_end].to_vec();

        Ok(ROM {
            metadata,
            prg_rom,
            chr_rom,
        })
    }
}

#[cfg(test)]
mod test {
    use sif::parameterized;

    use crate::roms::{
        loader::{CHR_ROM_BLOCK_SIZE_KB, Loader, Metadata, NES_MAGIC, PRG_ROM_BLOCK_SIZE_KB},
        mirroring::Mirroring,
    };

    #[test]
    fn test_load_returns_error_given_invalid_magic() {
        let rom_data = vec![0x55, 0x55, 0xAA, 0xAA];
        let result = Loader::load(&rom_data);
        assert!(result.is_err_and(|err| err == "Incorrect NES magic in ROM."));
    }

    #[test]
    fn test_load_returns_error_given_invalid_version() {
        let rom_data = vec![0x55, 0x55, 0xAA, 0xAA, 0x00, 0x08];
        let result = Loader::load(&rom_data);
        assert!(result.is_err_and(|err| err == "Incorrect NES magic in ROM."));
    }

    #[test]
    fn test_load_returns_rom() {
        let data_size: usize = 1;
        let flag_byte_6 = 0b0;
        let flag_byte_7 = 0b0000_0100;

        let mut header = Vec::<u8>::new();
        header.extend(NES_MAGIC);
        header.extend([
            data_size as u8,
            data_size as u8,
            flag_byte_6,
            flag_byte_7,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]);

        let prg_data = vec![0xAA; data_size * PRG_ROM_BLOCK_SIZE_KB];
        let chr_data = vec![0x55; data_size * CHR_ROM_BLOCK_SIZE_KB];

        let mut rom_data = vec![];
        rom_data.extend(header);
        rom_data.extend(prg_data);
        rom_data.extend(chr_data);

        let expected_metadata = Metadata {
            mapper: 0,
            mirroring: Mirroring::Horizontal,
            has_trainer: false,
            has_battery_ram: false,
            version: 1,
        };

        let result = Loader::load(&rom_data);

        assert!(result.is_ok());

        let rom = result.unwrap();

        assert_eq!(expected_metadata, rom.metadata);

        assert!(rom.chr_rom.iter().all(|x| x == &0x55));
        assert_eq!(PRG_ROM_BLOCK_SIZE_KB, rom.prg_rom.len());

        assert!(rom.prg_rom.iter().all(|x| x == &0xAA));
        assert_eq!(CHR_ROM_BLOCK_SIZE_KB, rom.chr_rom.len());
    }

    #[test]
    fn test_parse_metadata() {
        let flag_byte_6 = 0b1101_1111;
        let flag_byte_7 = 0b0010_0000;

        let result = Loader::parse_metadata(flag_byte_6, flag_byte_7);

        assert_eq!(0b0010_1101, result.mapper);
        assert_eq!(Mirroring::FourScreen, result.mirroring);
        assert!(result.has_trainer);
        assert!(result.has_battery_ram);
    }

    #[parameterized]
    #[case(0b0000_0000, Mirroring::Horizontal)]
    #[case(0b0000_0001, Mirroring::Vertical)]
    #[case(0b0000_1000, Mirroring::FourScreen)]
    #[case(0b0000_1001, Mirroring::FourScreen)]
    fn test_parse_metadata_sets_mirroring(flag_byte_6: u8, expected_mirroring: Mirroring) {
        assert_eq!(
            expected_mirroring,
            Loader::parse_metadata(flag_byte_6, 0b0).mirroring
        );
    }

    #[parameterized]
    #[case(0b0000_0100, 1)]
    #[case(0b0000_1000, 2)]
    fn test_parse_metadata_sets_version(flag_byte_7: u8, expected_version: u8) {
        assert_eq!(
            expected_version,
            Loader::parse_metadata(0b0, flag_byte_7).version
        );
    }
}
