use nessy::cpus::mos_6502::cpu::Mos6502;

pub trait TracingPolicy {
    fn trace(cpu: &Mos6502) -> String;
}
