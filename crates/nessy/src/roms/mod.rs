use crate::roms::loader::Metadata;

pub mod loader;
pub mod mirroring;

#[derive(Debug)]
pub struct ROM {
    #[allow(dead_code)]
    pub metadata: Metadata,

    pub prg_rom: Vec<u8>,

    #[allow(dead_code)]
    pub chr_rom: Vec<u8>,
}
