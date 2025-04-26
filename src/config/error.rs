use crate::i18n::t;

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    ParseError(String),
    InvalidConfig(String),
}

impl std::error::Error for ConfigError {}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(msg) => {
                write!(f, "{}", t("config_file_not_found").replace("{}", msg))
            }
            ConfigError::ParseError(msg) => {
                write!(f, "{}", t("failed_parse_config").replace("{}", msg))
            }
            ConfigError::InvalidConfig(msg) => {
                write!(f, "{}", t("config_invalid").replace("{}", msg))
            }
        }
    }
}
