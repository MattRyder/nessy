use crate::cpus::mos_6502::cpu::Mos6502;

#[derive(Default)]
pub struct NES {
    pub cpu: Mos6502,
}
