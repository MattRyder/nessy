use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Display {
    pub width: u32,
    pub height: u32,
    pub clear_colour: [u8; 3],
}

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    pub title: String,
    pub display: Display,
}
