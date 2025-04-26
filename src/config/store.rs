use super::error::ConfigError;
use super::types::Config;
use crate::i18n::t;
use std::sync::OnceLock;

// Global Configuration Instance
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

/// Initialize the global configuration
pub fn init(config: Config) {
    let _ = GLOBAL_CONFIG.set(config);
}

/// Get the global configuration instance
/// Returns an error if the configuration is not initialized
pub fn global() -> Result<&'static Config, ConfigError> {
    GLOBAL_CONFIG
        .get()
        .ok_or_else(|| ConfigError::InvalidConfig(t("global_config_not_initialized").to_string()))
}

/// Check if the current language is Chinese
pub fn is_chinese() -> bool {
    global()
        .map(|config| matches!(config.language, super::types::Language::Chinese))
        .unwrap_or(false)
}

pub fn print_config() {
    match global() {
        Ok(config) => {
            println!();
            println!("{}", config);
            println!();
        }
        Err(e) => {
            eprintln!("{}", t("failed_print_config").replace("{}", &e.to_string()));
        }
    }
}
