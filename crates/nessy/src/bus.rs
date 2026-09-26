use crate::{
    bus_error::BusError,
    ppu::{PPU, PictureProcessingUnitSystem},
};

// Constants
pub const CPU_MEMORY_SIZE: usize = 2048;

// Records how large a 16KB block is, for PRG ROM mirroring.
const SIXTEEN_KILOBYTES: usize = 0x4000;

// NES RAM Mirroring:
//
// The NES CPU RAM has 2k KiB available, by nature of the 11 lines attached from CPU to RAM.
// So reading any 12+ bit address results in those extra msb bits being ignored, effectively
// causing an 11-bit address read, at most.
//
// Given this wrapping, the memory is thus mirrored a few times after the 11-bits.
// Addressing 0x0000, 0x8000, and 0x1000 all return the exact same physical byte in RAM.
//
// [0x0000 - 0x07FF] 'original' memory address
// [0x0800 - 0x0FFF] mirrored
// [0x1000 - 0x17FF] mirrored
// [0x1800 - 0x1FFF] mirrored
const CPU_RAM_START: u16 = 0x0000;
const CPU_RAM_MIRROR_RANGE_END: u16 = 0x1FFF;
const CPU_RAM_ADDRESS_MASK: u16 = 0x07FF;

// PPU Addressing
const PPU_CONTROLLER_ADDR: u16 = 0x2000;
const PPU_MASK_ADDR: u16 = 0x2001;
const PPU_STATUS_ADDR: u16 = 0x2002;
const PPU_OAM_ADDRESS_ADDR: u16 = 0x2003;
const PPU_OAM_DATA_ADDR: u16 = 0x2004;
const PPU_SCROLL_ADDR: u16 = 0x2005;
const PPU_ADDRESS_ADDR: u16 = 0x2006;
const PPU_DATA_ADDR: u16 = 0x2007;
const PPU_OAM_DMA_ADDR: u16 = 0x4014;

const PPU_MIRROR_START: u16 = 0x2008;
const PPU_MIRROR_END: u16 = 0x3FFF;

const CART_RAM_START: u16 = 0x6000;
const CART_RAM_END: u16 = 0x7FFF;
const CART_ROM_START: u16 = 0x8000;
const CART_ROM_END: u16 = 0xFFFF;

pub trait BusSystem {
    fn read(&mut self, address: u16) -> Result<u8, BusError>;
    fn write(&mut self, address: u16, data: u8) -> Result<(), BusError>;
    fn write_slice(&mut self, start_address: u16, data: &[u8]);

    fn read_u16(&mut self, address: u16) -> Result<u16, BusError>;
    fn write_u16(&mut self, address: u16, data: u16);
}

pub struct Bus {
    cpu_memory: [u8; CPU_MEMORY_SIZE],
    program_rom: Option<Vec<u8>>,
    ppu: Box<dyn PictureProcessingUnitSystem>,
}

impl Default for Bus {
    fn default() -> Self {
        // Zero inits the RAM but NES state could be garbage on hardware.
        Self {
            cpu_memory: [0; CPU_MEMORY_SIZE],
            program_rom: None,
            ppu: Box::new(PPU::default()),
        }
    }
}

impl Bus {
    pub fn new_with_cpu_memory(
        cpu_memory: [u8; CPU_MEMORY_SIZE],
        program_rom: Option<Vec<u8>>,
    ) -> Self {
        Self {
            cpu_memory,
            program_rom,
            ..Default::default()
        }
    }

    pub fn new(program_rom: Option<Vec<u8>>) -> Self {
        Self {
            program_rom,
            ..Default::default()
        }
    }
}

impl BusSystem for Bus {
    fn read(&mut self, address: u16) -> Result<u8, BusError> {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                Ok(self.cpu_memory[addr])
            }

            PPU_CONTROLLER_ADDR | PPU_MASK_ADDR | PPU_OAM_ADDRESS_ADDR | PPU_SCROLL_ADDR
            | PPU_ADDRESS_ADDR | PPU_OAM_DMA_ADDR => Err(BusError::ReadFromWriteOnlyAddress),

            PPU_STATUS_ADDR => Ok(self.ppu.status_read()),

            PPU_OAM_DATA_ADDR => Ok(self.ppu.oam_data_read()),

            PPU_DATA_ADDR => match self.ppu.read() {
                Ok(v) => Ok(v),
                Err(err) => Err(err.into()),
            },

            // Mirror it down, then read from the proper address.
            PPU_MIRROR_START..=PPU_MIRROR_END => {
                let mirror_down_addr = address & 0b00100000_00000111;
                self.read(mirror_down_addr)
            }

            CART_RAM_START..=CART_RAM_END => todo!("Cart RAM read not implemented!"),

            CART_ROM_START..=CART_ROM_END => {
                // PRG ROM size can be 16KB or 32KB.
                // As there's a 32KB addressible space here, a 16KB ROM basically
                // means that it's mirrored (i.e. mapped to the lower 16KB.)
                // If we've got a 16KB PRG ROM, and the address is over 16KB, map it down.
                //
                match &self.program_rom {
                    Some(rom) => {
                        let mut addr = address as usize;

                        if rom.len() == SIXTEEN_KILOBYTES && addr >= SIXTEEN_KILOBYTES {
                            addr %= SIXTEEN_KILOBYTES;
                        }

                        Ok(rom[addr])
                    }
                    None => Err(BusError::NoCartridgePresent),
                }
            }
            _ => Err(BusError::UnsupportedAddressAccessed),
        }
    }

    fn write(&mut self, address: u16, data: u8) -> Result<(), BusError> {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                self.cpu_memory[addr] = data;
            }

            PPU_CONTROLLER_ADDR => self.ppu.control_write(data),

            PPU_MASK_ADDR => self.ppu.mask_write(data),

            PPU_STATUS_ADDR => return Err(BusError::WriteFromReadOnlyAddress),

            PPU_OAM_ADDRESS_ADDR => self.ppu.oam_addr_write(data),

            PPU_OAM_DATA_ADDR => self.ppu.oam_data_write(data),

            PPU_SCROLL_ADDR => self.ppu.scroll_write(data),

            PPU_ADDRESS_ADDR => self.ppu.address_write(data),

            PPU_DATA_ADDR => {
                if let Err(error) = self.ppu.write(data) {
                    return Err(error.into());
                }
            }

            PPU_MIRROR_START..=PPU_MIRROR_END => {
                let mirrored_down_address = address & 0x2007;
                return self.write(mirrored_down_address, data);
            }

            CART_ROM_START..=CART_ROM_END => return Err(BusError::WriteToCartridgeROM),

            _ => return Err(BusError::UnsupportedAddressAccessed),
        }

        Ok(())
    }

    fn write_slice(&mut self, start_address: u16, data: &[u8]) {
        // TODO: this should check the entire range is valid for the ram segment being written.
        match start_address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (start_address & CPU_RAM_ADDRESS_MASK) as usize;
                self.cpu_memory[addr..(addr + data.len())].copy_from_slice(data);
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    start_address
                );
            }
        }
    }

    fn read_u16(&mut self, address: u16) -> Result<u16, BusError> {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                let addr_u16 = addr as u16;
                let lo_byte = self.read(addr_u16)? as u16;
                let hi_byte = self.read(addr_u16 + 1)? as u16;
                Ok((hi_byte << 8) | lo_byte)
            }
            CART_ROM_START..=CART_ROM_END => {
                if self.program_rom.is_none() {
                    return Err(BusError::NoCartridgePresent);
                }

                let lo_byte = self.read(address).unwrap() as u16;
                let hi_byte = self.read(address + 1).unwrap() as u16;
                Ok((hi_byte << 8) | lo_byte)
            }
            _ => Err(BusError::UnsupportedAddressAccessed),
        }
    }

    fn write_u16(&mut self, address: u16, data: u16) {
        match address {
            CPU_RAM_START..=CPU_RAM_MIRROR_RANGE_END => {
                let addr = (address & CPU_RAM_ADDRESS_MASK) as usize;
                let addr_u16 = addr as u16;
                let hi_byte = (data >> 8) as u8;
                let lo_byte = (data & 0xFF) as u8;
                self.write(addr_u16, lo_byte);
                self.write(addr_u16 + 1, hi_byte);
            }
            _ => {
                println!(
                    "Can't access address `0x{:x}`. This needs to be an error returned!",
                    address
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;
    use mockall::mock;
    use sif::parameterized;

    use crate::ppu::ReadError;

    use super::*;

    mock! {
        pub PPU {}

        impl PictureProcessingUnitSystem for PPU {
            fn address_write(&mut self, value: u8);
            fn control_write(&mut self, value: u8);
            fn mask_write(&mut self, value: u8);
            fn oam_addr_write(&mut self, value: u8);
            fn oam_data_write(&mut self, value: u8);
            fn scroll_write(&mut self, value: u8);
            fn write(&mut self, value: u8) -> Result<(), crate::ppu::WriteError>;
            fn increment_vram_address(&mut self);

            fn read(&mut self) -> Result<u8, crate::ppu::ReadError>;
            fn oam_data_read(&self) -> u8;
            fn status_read(&mut self) -> u8;
        }
    }

    fn setup_memory(values: Vec<(u8, u8)>) -> [u8; CPU_MEMORY_SIZE] {
        let mut memory = [0; CPU_MEMORY_SIZE];
        for (addr, value) in values {
            memory[addr as usize] = value;
        }
        memory
    }

    #[test]
    fn test_read_returns_correct_value() {
        let memory = setup_memory(vec![(0x10, 0xAA)]);
        let mut bus = Bus::new_with_cpu_memory(memory, None);

        assert_eq_hex!(0xAA, bus.read(0x10).unwrap());
    }

    #[parameterized]
    #[case(PPU_CONTROLLER_ADDR)]
    #[case(PPU_MASK_ADDR)]
    #[case(PPU_OAM_ADDRESS_ADDR)]
    #[case(PPU_SCROLL_ADDR)]
    #[case(PPU_ADDRESS_ADDR)]
    #[case(PPU_OAM_DMA_ADDR)]
    fn test_read_with_write_only_address_returns_write_to_read_address_error(address: u16) {
        let result = Bus::default().read(address);
        assert!(result.is_err());
        assert_eq!(BusError::ReadFromWriteOnlyAddress, result.err().unwrap())
    }

    #[parameterized]
    #[case(PPU_STATUS_ADDR, BusError::WriteFromReadOnlyAddress)]
    #[case(CART_ROM_START, BusError::WriteToCartridgeROM)]
    #[case(CART_ROM_END, BusError::WriteToCartridgeROM)]
    fn test_write_with_address_return_err(address: u16, expected_err: BusError) {
        let result = Bus::default().write(address, 0x41);
        assert_eq!(expected_err, result.err().unwrap());
    }

    #[test]
    fn test_read_with_ppu_data_address_returns_ok_given_valid_ppu_read() {
        let mut ppu = Box::new(MockPPU::default());

        ppu.expect_read().returning(|| Ok(0x55));

        let mut bus = Bus {
            ppu,
            ..Default::default()
        };

        assert_eq!(0x55, bus.read(PPU_DATA_ADDR).unwrap());
    }

    #[test]
    fn test_read_on_status_returns_status_register() {
        let mut ppu = Box::new(MockPPU::default());

        ppu.expect_status_read().returning(|| 0x32);

        let mut bus = Bus {
            ppu,
            ..Default::default()
        };

        assert_eq_hex!(0x32, bus.read(PPU_STATUS_ADDR).unwrap());
    }

    #[test]
    fn test_read_with_ppu_data_address_returns_err_given_invalid_ppu_read() {
        let mut ppu = Box::new(MockPPU::default());

        ppu.expect_read()
            .returning(|| Err(crate::ppu::ReadError::UnusedMemoryRegion));

        let mut bus = Bus {
            ppu,
            ..Default::default()
        };

        let result = bus.read(PPU_DATA_ADDR);

        assert!(result.is_err());

        assert_eq!(
            BusError::PPUReadError(ReadError::UnusedMemoryRegion),
            result.err().unwrap()
        );
    }

    #[test]
    fn test_read_with_mirrored_ppu_data_address_returns_ok_given_valid_ppu_read() {
        let mut ppu = Box::new(MockPPU::default());

        ppu.expect_read().returning(|| Ok(0x55));

        let mut bus = Bus {
            ppu,
            ..Default::default()
        };

        assert_eq!(0x55, bus.read(0x2017).unwrap());
    }

    #[test]
    fn test_write_sets_correct_value() {
        let memory = setup_memory(vec![]);
        let mut bus = Bus::new_with_cpu_memory(memory, None);

        bus.write(0x400, 0xAA);

        assert_eq_hex!(0xAA, bus.cpu_memory[0x400]);
    }

    #[parameterized]
    #[case(vec![(0x50, 0xAA), (0x51, 0xBB)], 0x50, 0xBBAA)]
    fn test_read_u16_returns_correct_value(
        memory: Vec<(u8, u8)>,
        address: u16,
        expected_value: u16,
    ) {
        let memory = setup_memory(memory);
        let mut bus = Bus::new_with_cpu_memory(memory, None);

        assert_eq_hex!(expected_value, bus.read_u16(address).unwrap());
    }

    #[test]
    fn test_write_u16_set_correct_value() {
        let memory = setup_memory(vec![]);
        let mut bus = Bus::new_with_cpu_memory(memory, None);

        bus.write_u16(0x0001, 0xAABB);

        assert_eq!(0xBB, bus.cpu_memory[0x0001]);
        assert_eq!(0xAA, bus.cpu_memory[0x0002]);
    }
}
