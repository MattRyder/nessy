use std::fmt::Display;

use libnessy::cpus::mos_6502::{bus::MemoryAccess, cpu::Mos6502, opcode::OPCODES};

pub struct CpuState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
}

pub struct OpcodeState {
    pub opcode_bytes: Vec<u8>,
    pub opcode_string: String,
}

pub struct State {
    pub cpu: CpuState,
    pub opcode: OpcodeState,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<5X} {:<9} {:<31} A:{:02X} X:{:02X} Y:{:02X} SP:{:02X}",
            self.cpu.pc,
            self.opcode
                .opcode_bytes
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(" "),
            self.opcode.opcode_string,
            self.cpu.a,
            self.cpu.x,
            self.cpu.y,
            self.cpu.sp
        )
    }
}

pub fn trace(cpu: &Mos6502) -> State {
    let opcode_byte = cpu.bus.read(cpu.program_counter);

    let opcode = OPCODES.get(&opcode_byte).unwrap_or_else(|| {
        panic!(
            "Unknown opcode `0x{:02X}` at address `0x{:04X}`.",
            opcode_byte, cpu.program_counter
        )
    });

    let opcode_state = {
        let opcode_bytes = (0..=opcode.bytes)
            .map(|i| cpu.bus.read(cpu.program_counter + i as u16))
            .collect();

        OpcodeState {
            opcode_bytes,
            opcode_string: opcode.mnemonic.to_string(),
        }
    };

    let cpu = CpuState {
        a: cpu.registers.a,
        x: cpu.registers.x,
        y: cpu.registers.y,
        sp: cpu.stack_pointer,
        pc: cpu.program_counter,
    };

    State {
        cpu,
        opcode: opcode_state,
    }
}
