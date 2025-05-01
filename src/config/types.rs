use super::{error::ConfigError, schema::ConfigFile};
use crate::i18n::t;
use std::fmt;

/// Language options for the application
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Language {
    English,
    Chinese,
}

// Implement the Display trait
// e.g. `println!("{}", Language::English);` will print `en`
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::Chinese => write!(f, "zh"),
        }
    }
}

/// Parsing from string to enumeration implementation
impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" => Some(Language::English),
            "zh" => Some(Language::Chinese),
            _ => None,
        }
    }
}

/// Global configuration structure
#[derive(Debug, Clone)]
pub struct Config {
    pub language: Language,
    pub authors: Vec<String>,
    pub date_range: Vec<String>,
    pub repos: Vec<String>,
    pub format: std::collections::HashMap<String, String>,
    pub includes: Vec<String>,
    pub excludes: Vec<String>,
}

impl Config {
    /// Create a new configuration instance with default values
    pub fn new_from_file(file_config: &ConfigFile) -> Result<Self, ConfigError> {
        let language = Language::from_str(&file_config.lang)
            .ok_or_else(|| ConfigError::InvalidConfig("Invalid language setting".to_string()))?;

        Ok(Self {
            language,
            authors: file_config.authors.clone(),
            date_range: file_config.date_range.clone(),
            repos: file_config.repos.clone(),
            format: file_config.format.clone(),
            includes: file_config.includes.clone(),
            excludes: file_config.excludes.clone(),
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "{}", t("config_details"))?;
        writeln!(f, "")?;
        writeln!(
            f,
            "{}: {}",
            t("language"),
            if matches!(self.language, Language::Chinese) {
                "Chinese"
            } else {
                "English"
            }
        )?;
        writeln!(f, "{}: {:?}", t("authors"), self.authors)?;
        writeln!(
            f,
            "{}: {} - {}",
            t("date_range"),
            self.date_range.get(0).unwrap_or(&"N/A".to_string()),
            self.date_range.get(1).unwrap_or(&"N/A".to_string())
        )?;
        writeln!(f, "{}:", t("repos"))?;
        for repo in &self.repos {
            if let Some(display_name) = self.format.get(repo) {
                writeln!(f, "  - {} ({})", display_name, repo)?;
            } else {
                writeln!(f, "  - {}", repo)?;
            }
        }
        writeln!(f, "{}:", t("includes"))?;
        for inc in &self.includes {
            writeln!(f, "  - {}", inc)?;
        }
        if !self.excludes.is_empty() {
            writeln!(f, "{}:", t("excludes"))?;
            for exc in &self.excludes {
                writeln!(f, "  - {}", exc)?;
            }
        }
        writeln!(f, "")?;
        writeln!(f, "--------------------------------")?;
        Ok(())
    }
}
