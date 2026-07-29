use nessy::cpus::mos_6502::cpu::Mos6502;

use crate::colour::Colour;

pub struct Renderer {
    pub buffer: Vec<u8>,
}

impl Renderer {
    pub fn new(width: usize, height: usize, bytes_per_pixel: usize) -> Self {
        let byte_count = width * height * bytes_per_pixel;

        Self {
            buffer: vec![0; byte_count],
        }
    }

    pub fn handle(&mut self, cpu: &Mos6502) -> bool {
        let mut has_updated = false;
        let mut frame_index = 0;

        for i in 0x0200..0x600 {
            let pixel_byte = cpu.bus.read(i as u16);
            let (b1, b2, b3) = Colour::from_u8(pixel_byte).rgb();

            if self.buffer[frame_index] != b1
                && self.buffer[frame_index + 1] != b2
                && self.buffer[frame_index + 2] != b3
            {
                self.buffer[frame_index] = b1;
                self.buffer[frame_index + 1] = b2;
                self.buffer[frame_index + 2] = b3;
                has_updated = true;
            }
            frame_index += 3;
        }

        has_updated
    }
}
