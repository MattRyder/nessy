use crate::roms::loader::Metadata;

pub mod loader;
pub mod mirroring;

#[derive(Debug)]
pub struct ROM {
    metadata: Metadata,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}
