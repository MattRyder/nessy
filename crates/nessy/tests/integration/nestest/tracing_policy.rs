use nessy::cpus::mos_6502::cpu::Mos6502;

use crate::integration::{nestest::Nestest, tracing_policy::TracingPolicy};

impl TracingPolicy for Nestest {
    fn trace(cpu: &Mos6502) -> String {
        Nestest::generate_state(cpu).to_string()
    }
}
