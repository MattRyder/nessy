use crate::roms::loader::Metadata;

pub mod loader;
pub mod mirroring;

#[derive(Debug)]
pub struct ROM {
    #[allow(dead_code)]
    metadata: Metadata,

    prg_rom: Vec<u8>,

    #[allow(dead_code)]
    chr_rom: Vec<u8>,
}

impl ROM {
    pub fn program_rom(&self) -> &Vec<u8> {
        &self.prg_rom
    }
}
