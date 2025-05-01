use super::error::ConfigError;
use serde::Deserialize;
use std::{collections::HashMap, fs};

// The schema of the config file
#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    /// Language setting: 'en' or 'zh'
    #[serde(default)]
    pub lang: String,

    /// List of author names
    /// Vec<String> is equivalent to string[] in TypeScript
    pub authors: Vec<String>,

    /// Date range: [start_date, end_date]
    /// Vec<String> here represents a fixed-length array of 2 strings
    #[serde(rename = "dateRange", default)]
    pub date_range: Vec<String>,

    /// List of Git repository paths
    pub repos: Vec<String>,

    /// Repository name formatting map
    /// HashMap<K,V> is equivalent to Record<K,V> in TypeScript
    /// or { [key: string]: string }
    #[serde(default)]
    pub format: HashMap<String, String>,

    /// List of commit types to include
    #[serde(default)]
    pub includes: Vec<String>,

    /// List of keywords to exclude
    #[serde(default)]
    pub excludes: Vec<String>,
}

impl ConfigFile {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Reading file contents
        let content = fs::read_to_string(path)?;

        // Parsing JSON
        let mut config: ConfigFile = serde_json::from_str(&content)?;

        // Allow missing fields
        config.fill_defaults();

        // Validate the config
        config.validate()?;

        Ok(config)
    }

    fn fill_defaults(&mut self) {
        use chrono::Local;

        // lang default en
        if self.lang.is_empty() {
            self.lang = "en".to_string();
        }

        // dateRange default today
        if self.date_range.len() != 2 {
            let today = Local::now().format("%Y-%m-%d").to_string();
            self.date_range = vec![today.clone(), today];
        }

        // includes default the day the program is running
        if self.includes.is_empty() {
            self.includes = vec![
                "feat".into(),
                "fix".into(),
                "docs".into(),
                "style".into(),
                "refactor".into(),
                "test".into(),
                "chore".into(),
            ];
        }

        // excludes default empty
        if self.excludes.is_empty() {
            self.excludes = vec![];
        }

        // format default empty
        if self.format.is_empty() {
            self.format = std::collections::HashMap::new();
        }
    }

    fn validate(&self) -> Result<(), ConfigError> {
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
