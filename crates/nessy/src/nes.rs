use crate::{bus::Bus, cpus::mos_6502::cpu::Mos6502, ppu::PPU, roms::ROM};

#[derive(Default)]
pub struct NES {
    pub cpu: Mos6502,
    pub ppu: PPU,
}

impl NES {
    pub fn new(rom: Option<ROM>) -> Self {
        let prg_rom = if let Some(rm) = rom {
            Some(rm.prg_rom)
        } else {
            None
        };

        let cpu_bus = Box::new(Bus::new(prg_rom));

        let cpu = Mos6502::new(cpu_bus);

        Self {
            cpu,
            ..Default::default()
        }
    }
}
