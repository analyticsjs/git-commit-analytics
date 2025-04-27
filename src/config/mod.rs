pub mod constants;
mod error;
mod schema;
mod store;
mod types;

use constants::CONFIG_FILE_NAME;
use error::ConfigError;
use schema::ConfigFile;
use std::path::PathBuf;

pub use store::{init, is_chinese, print_config};
pub use types::Config;

fn find_config_file(root_path: &PathBuf) -> Option<PathBuf> {
    let config_file_path = root_path.join(CONFIG_FILE_NAME);
    if config_file_path.exists() {
        return Some(config_file_path);
    }

    // 3. If both are not found, return None
    None
}

/// Initialize configuration from file or use default settings
pub fn init_config(root_path: &PathBuf) -> Result<Config, ConfigError> {
    let config_path = find_config_file(root_path).ok_or(ConfigError::FileNotFound)?;

    ConfigFile::from_file(config_path.to_str().unwrap())
        .map_err(|_e| ConfigError::ParseError)
        .and_then(|file_config| {
            let config = Config::new_from_file(&file_config)?;
            init(config.clone());
            Ok(config)
        })
}
