use nessy::cpus::mos_6502::{cpu::Mos6502, opcode::OPCODES};

use crate::integration::nestest::{
    disassembler::Disassembler,
    state::{CpuState, OpcodeState, State},
};

pub mod disassembler;
pub mod opcode_behaviour;
pub mod state;
pub mod tracing_policy;

use std::fs::{self, File};
use std::io::Write;

use nessy::nes::NES;
use nessy::roms::loader::Loader;

use crate::integration::tracing_policy::TracingPolicy;
use crate::integration::utils::get_asset_file_path;

// nestest loads the PC from 0xFFFC as 0xC004 by default.
// This is usually fine for the interactive test but we need
// the non-interactive test, which actually kicks off at 0xC000]
// So we'll manually set the PC to the automated value after reset().
const NESTEST_INITIAL_PC: u16 = 0xC000;

pub struct Nestest {}

impl Nestest {
    fn generate_state(cpu: &Mos6502) -> State {
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
                opcode_string: Disassembler::generate_disassembly(cpu, opcode).unwrap_or_default(),
                undocumented: opcode.undocumented,
            }
        };

        let cpu = CpuState {
            a: cpu.registers.a,
            x: cpu.registers.x,
            y: cpu.registers.y,
            p: cpu.status.bits(),
            sp: cpu.stack_pointer,
            pc: cpu.program_counter,
        };

        State {
            cpu,
            opcode: opcode_state,
        }
    }
}

#[test]
// #[ignore = "nestest requires more work in the illegal opcodes first..."]
fn test_nestest_against_libnessy() {
    let rom_file_path_buf = get_asset_file_path("nestest/nestest.nes");
    let rom_file_path = rom_file_path_buf.to_str().unwrap();

    let nestest_reference_path_buf = get_asset_file_path("nestest/nestest_without_cycles.log");
    let nestest_reference_file_path = nestest_reference_path_buf.to_str().unwrap();

    let nestest_reference_log = load_reference_log(nestest_reference_file_path);

    let mut nes = build_nes(rom_file_path);

    let mut output_file = File::create("./nestest_run.log").expect("Failed to create output file.");

    let mut line_idx = 0;

    nes.cpu.run_with_callback(|cpu| {
        let nestest_line_text = &nestest_reference_log[line_idx];

        if nestest_line_text.starts_with("C68B") && line_idx == 8980 {
            return;
        }

        let trace = Nestest::trace(cpu);

        assert_eq!(&trace, nestest_line_text);

        line_idx += 1;

        let write_result = writeln!(output_file, "{}", trace);

        match write_result {
            Ok(_) => (),
            Err(reason) => panic!("Failed to write to output file: {}.", reason),
        }
    });
}

fn build_nes(rom_file_path: &str) -> NES {
    // Load ROM file
    let rom_data = match fs::read(rom_file_path) {
        Ok(data) => data,
        Err(reason) => panic!("Failed to read ROM: {}.", reason),
    };

    let rom = Loader::load(&rom_data);

    if rom.is_err() {
        panic!("ROM load error: {}", rom.err().unwrap());
    }

    let mut nes = NES::default();

    nes.cpu.load_program(rom.unwrap());
    nes.cpu.reset();

    nes.cpu.program_counter = NESTEST_INITIAL_PC;

    nes
}

fn load_reference_log(reference_log_file_path: &str) -> Vec<String> {
    let log_data = match fs::read_to_string(reference_log_file_path) {
        Ok(data) => data,
        Err(reason) => panic!("Failed to read reference log: {}.", reason),
    };

    log_data.lines().map(|line| line.to_string()).collect()
}
