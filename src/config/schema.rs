use super::error::ConfigError;
use serde::Deserialize;
use std::{collections::HashMap, fs};

// The schema of the config file
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// Language setting: 'en' or 'zh'
    pub lang: String,

    /// List of author names
    /// Vec<String> is equivalent to string[] in TypeScript
    pub authors: Vec<String>,

    /// Date range: [start_date, end_date]
    /// Vec<String> here represents a fixed-length array of 2 strings
    #[serde(rename = "dateRange")]
    pub date_range: Vec<String>,

    /// List of Git repository paths
    pub repos: Vec<String>,

    /// Repository name formatting map
    /// HashMap<K,V> is equivalent to Record<K,V> in TypeScript
    /// or { [key: string]: string }
    pub format: HashMap<String, String>,

    /// List of commit types to include
    pub includes: Vec<String>,

    /// List of keywords to exclude
    pub excludes: Vec<String>,
}

impl ConfigFile {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Reading file contents
        let content = fs::read_to_string(path)?;

        // Parsing JSON
        let config: ConfigFile = serde_json::from_str(&content)?;

        // Validate the config
        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.date_range.len() != 2 {
            return Err(ConfigError::InvalidConfig(
                "date_range must contain exactly 2 dates".to_string(),
            ));
        }
        if self.authors.is_empty() {
            return Err(ConfigError::InvalidConfig(
                "authors list cannot be empty".to_string(),
            ));
        }
        if self.repos.is_empty() {
            return Err(ConfigError::InvalidConfig(
                "repos list cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
