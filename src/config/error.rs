use crate::config::constants::CONFIG_FILE_NAME;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    ParseError,
    InvalidConfig(String),
}

impl std::error::Error for ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => {
                write!(
                    f,
                    "\nPlease make sure there is a {} file in the program directory.",
                    CONFIG_FILE_NAME
                )
            }
            ConfigError::ParseError => {
                write!(
                    f,
                    "\nFailed to parse {}, please check the file format.",
                    CONFIG_FILE_NAME
                )
            }
            ConfigError::InvalidConfig(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}
