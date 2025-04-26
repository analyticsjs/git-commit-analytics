mod error;
mod schema;
mod store;
mod types;

use error::ConfigError;
use schema::ConfigFile;
use std::path::Path;
pub use store::{init, is_chinese, print_config};
pub use types::{Config, Language};

const CONFIG_FILE_PATH: &str = "config.json";

/// Initialize configuration from file or use default settings
pub fn init_config() -> Result<Config, ConfigError> {
    if !Path::new(CONFIG_FILE_PATH).exists() {
        return Err(ConfigError::FileNotFound(
            "config.json not found".to_string(),
        ));
    }

    ConfigFile::from_file(CONFIG_FILE_PATH)
        .map_err(|e| ConfigError::ParseError(e.to_string()))
        .and_then(|file_config| {
            let config = Config::new_from_file(&file_config)?;
            init(config.clone());
            Ok(config)
        })
}
