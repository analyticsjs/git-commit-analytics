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
                    "\nPlease make sure there is a config.json file in the program directory."
                )
            }
            ConfigError::ParseError => {
                write!(
                    f,
                    "\nFailed to parse config.json, please check the file format."
                )
            }
            ConfigError::InvalidConfig(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}
