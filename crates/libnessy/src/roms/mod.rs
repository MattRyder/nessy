use crate::roms::loader::Metadata;

pub mod loader;
pub mod mirroring;

#[derive(Debug)]
pub struct ROM {
    metadata: Metadata,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl ROM {
    pub fn program_rom(&self) -> &Vec<u8> {
        &self.prg_rom
    }
}
