use sdl2::pixels::Color;

pub struct Colour {}

impl Colour {
    pub fn from_u8(value: u8) -> Color {
        match value {
            0 => Color::BLACK,
            1 => Color::WHITE,
            2 | 9 => Color::GREY,
            3 | 10 => Color::RED,
            4 | 11 => Color::GREEN,
            5 | 12 => Color::BLUE,
            6 | 13 => Color::MAGENTA,
            7 | 14 => Color::YELLOW,
            _ => Color::CYAN,
        }
    }
}
