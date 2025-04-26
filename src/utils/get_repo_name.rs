use std::collections::HashMap;
use std::path::Path;

/// Get repository name
///
/// # Parameters
/// - `repo_dir`: repository path
/// - `format`: mapping from directory name to repository name
///
/// # Returns
/// - repository name (None if not found)
pub fn get_repo_name(repo_dir: &str, format: &HashMap<String, String>) -> Option<String> {
    let repo_dir = repo_dir.trim_end_matches(std::path::MAIN_SEPARATOR);
    let path = Path::new(repo_dir);
    if let Some(key_osstr) = path.file_name() {
        let key = key_osstr.to_string_lossy();
        format.get(key.as_ref()).cloned()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_repo_name_found() {
        let mut format = HashMap::new();
        format.insert("my-repo".to_string(), "My Repository".to_string());
        let repo_dir = "/Users/xxx/projects/my-repo";
        let repo_name = get_repo_name(repo_dir, &format);
        assert_eq!(repo_name, Some("My Repository".to_string()));
    }

    #[test]
    fn test_get_repo_name_not_found() {
        let format = HashMap::new();
        let repo_dir = "/Users/xxx/projects/unknown-repo";
        let repo_name = get_repo_name(repo_dir, &format);
        assert_eq!(repo_name, None);
    }

    #[test]
    fn test_get_repo_name_empty_path() {
        let format = HashMap::new();
        let repo_dir = "";
        let repo_name = get_repo_name(repo_dir, &format);
        assert_eq!(repo_name, None);
    }

    #[test]
    fn test_get_repo_name_trailing_slash() {
        let mut format = HashMap::new();
        format.insert("my-repo".to_string(), "My Repository".to_string());
        let repo_dir = "/Users/xxx/projects/my-repo/";
        let repo_name = get_repo_name(repo_dir, &format);
        assert_eq!(repo_name, Some("My Repository".to_string()));
    }
}
