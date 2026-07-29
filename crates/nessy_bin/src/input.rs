use bitflags::bitflags;
use nessy::cpus::mos_6502::cpu::Mos6502;
use sdl2::{
    event::{Event, EventPollIterator},
    keyboard::Keycode,
};

bitflags! {
    pub struct InputFlags: u8 {
        const Quit = 1 << 0;
    }
}

pub struct Input {}

impl Input {
    pub fn handle(cpu: &mut Mos6502, event_iter: &mut EventPollIterator) -> InputFlags {
        const INPUT_MEMORY_ADDRESS: u16 = 0xFF;
        const KEY_W_KEYCODE: u8 = 0x77;
        const KEY_A_KEYCODE: u8 = 0x61;
        const KEY_S_KEYCODE: u8 = 0x73;
        const KEY_D_KEYCODE: u8 = 0x64;

        let mut input_flags = InputFlags::empty();

        for event in event_iter {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => input_flags.insert(InputFlags::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => cpu.bus.write(INPUT_MEMORY_ADDRESS, KEY_W_KEYCODE),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => cpu.bus.write(INPUT_MEMORY_ADDRESS, KEY_S_KEYCODE),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => cpu.bus.write(INPUT_MEMORY_ADDRESS, KEY_A_KEYCODE),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => cpu.bus.write(INPUT_MEMORY_ADDRESS, KEY_D_KEYCODE),
                _ => {}
            }
        }

        input_flags
    }
}
