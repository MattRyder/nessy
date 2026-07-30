use std::fmt::Display;

pub struct CpuState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub pc: u16,
}

pub struct OpcodeState {
    pub opcode_bytes: Vec<u8>,
    pub opcode_string: String,
    pub undocumented: bool,
}

pub struct State {
    pub cpu: CpuState,
    pub opcode: OpcodeState,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<04X}  {:<8} {}{:<31} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.cpu.pc,
            self.opcode
                .opcode_bytes
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(" "),
            if self.opcode.undocumented { "*" } else { " " },
            self.opcode.opcode_string,
            self.cpu.a,
            self.cpu.x,
            self.cpu.y,
            self.cpu.p,
            self.cpu.sp
        )
    }
}
