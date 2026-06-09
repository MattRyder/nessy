use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Display {
    pub width: u32,
    pub height: u32,
    pub clear_colour: [u8; 3],
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub title: String,
    pub display: Display,
}

#[derive(Debug)]
pub enum SettingsError {
    FileError(std::io::Error),
    DeserialiseError(toml::de::Error),
}

impl Settings {
    pub fn new(settings_file_path: &Path) -> Result<Self, SettingsError> {
        let settings_file_content = match fs::read_to_string(settings_file_path) {
            Ok(v) => v,
            Err(error) => return Err(SettingsError::FileError(error)),
        };

        match toml::from_str(&settings_file_content) {
            Ok(settings) => Ok(settings),
            Err(error) => Err(SettingsError::DeserialiseError(error)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    fn create_settings_tempfile() -> NamedTempFile {
        let settings_text = r#"
        title = "Nessy Tests"
        [display]
        width = 800
        height = 600
        clear_colour = [100, 149, 237]
        "#;

        let mut tempfile = tempfile::NamedTempFile::new().unwrap();

        let _ = tempfile.write_all(settings_text.as_bytes());

        tempfile
    }

    #[test]
    fn test_new_returns_err_given_invalid_file() {
        let path = Path::new("./bogus_path_239o899sd0fv.toml");
        let settings = Settings::new(path);

        assert!(settings.is_err());
    }

    #[test]
    fn test_new_returns_settings_given_valid_file() {
        let settings_file = create_settings_tempfile();

        let settings = Settings::new(settings_file.path());

        assert!(settings.is_ok());

        let unwrapped = settings.unwrap();
        assert_eq!(800, unwrapped.display.width);
        assert_eq!(600, unwrapped.display.height);
    }
}
