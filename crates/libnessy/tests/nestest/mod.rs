use std::fs::{self, File};
use std::io::Write;

use libnessy::{nes::NES, roms::loader::Loader};

use crate::nestest::{tracer::trace, utils::get_asset_file_path};

mod tracer;
mod utils;

// nestest loads the PC from 0xFFFC as 0xC004 by default.
// This is usually fine for the interactive test but we need
// the non-interactive test, which actually kicks off at 0xC000]
// So we'll manually set the PC to the automated value after reset().
const NESTEST_INITIAL_PC: u16 = 0xC000;

#[test]
#[ignore = "nestest requires more work in the disasm first..."]
fn test_nestest_against_libnessy() {
    println!("Booting test_harnessy.");

    let rom_file_path_buf = get_asset_file_path("nestest/nestest.nes");
    let rom_file_path = rom_file_path_buf.to_str().unwrap();

    // let nestest_reference_path_buf = get_asset_file_path("nestest/nestest_without_cycles.log");
    // let nestest_reference_file_path = nestest_reference_path_buf.to_str().unwrap();
    //
    // let nestest_reference_log = load_reference_log(nestest_reference_file_path);

    let mut nes = build_nes(rom_file_path);

    let mut output_file = File::create("./nestest_run.log").expect("Failed to create output file.");

    nes.cpu.run_with_callback(|cpu| {
        let trace = trace(cpu);

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

// fn load_reference_log(reference_log_file_path: &str) -> Vec<String> {
//     let log_data = match fs::read_to_string(reference_log_file_path) {
//         Ok(data) => data,
//         Err(reason) => panic!("Failed to read reference log: {}.", reason),
//     };
//
//     log_data.lines().map(|line| line.to_string()).collect()
// }
