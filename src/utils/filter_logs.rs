use anyhow::Result;
use regex::Regex;

const AUTHOR_IDX: usize = 1;
const MSG_IDX: usize = 3;

/// Build a case-insensitive regex, return None if patterns is empty
fn build_regex(patterns: &[String]) -> Option<Regex> {
    let patterns: Vec<&str> = patterns
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.as_str())
        .collect();
    if patterns.is_empty() {
        None
    } else {
        Some(Regex::new(&format!("(?i){}", patterns.join("|"))).unwrap())
    }
}

pub fn filter_logs(
    logs: &[String],
    authors: &[String],
    includes: &[String],
    excludes: &[String],
) -> Result<Vec<String>> {
    // Build regex by configuration
    let author_re = build_regex(authors);
    let include_re = build_regex(includes);
    let exclude_re = build_regex(excludes);

    let filtered: Vec<String> = logs
        .iter()
        .filter(|log| {
            let fields: Vec<&str> = log.split("|||").collect();
            let author = fields.get(AUTHOR_IDX).unwrap_or(&"");
            author_re.as_ref().map_or(true, |re| re.is_match(author))
        })
        .filter(|log| {
            let fields: Vec<&str> = log.split("|||").collect();
            let msg = fields.get(MSG_IDX).unwrap_or(&"");
            include_re.as_ref().map_or(true, |re| re.is_match(msg))
        })
        .filter(|log| {
            let fields: Vec<&str> = log.split("|||").collect();
            let msg = fields.get(MSG_IDX).unwrap_or(&"");
            !exclude_re.as_ref().map_or(false, |re| re.is_match(msg))
        })
        .cloned()
        .collect();

    Ok(filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logs() -> Vec<String> {
        vec![
            "repo1|||Alice|||alice@example.com|||feat: add feature|||'abc123|||2024-06-01 12:00:00"
                .to_string(),
            "repo1|||Bob|||bob@example.com|||fix: bug|||'def456|||2024-06-01 13:00:00".to_string(),
            "repo2|||Alice|||alice@example.com|||docs: update|||'ghi789|||2024-06-01 14:00:00"
                .to_string(),
            "repo2|||Eve|||eve@example.com|||chore: deps|||'jkl012|||2024-06-01 15:00:00"
                .to_string(),
        ]
    }

    #[test]
    fn test_filter_by_author() {
        let logs = logs();
        let authors = vec!["Alice".to_string()];
        let includes = vec!["".to_string()]; // 匹配所有
        let excludes = vec!["".to_string()]; // 不排除
        let filtered = filter_logs(&logs, &authors, &includes, &excludes).unwrap();
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|log| log.contains("Alice")));
    }

    #[test]
    fn test_filter_by_include() {
        let logs = logs();
        let authors = vec!["".to_string()]; // 匹配所有
        let includes = vec!["feat".to_string()];
        let excludes = vec!["".to_string()];
        let filtered = filter_logs(&logs, &authors, &includes, &excludes).unwrap();
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("feat: add feature"));
    }

    #[test]
    fn test_filter_by_exclude() {
        let logs = logs();
        let authors = vec!["".to_string()];
        let includes = vec!["".to_string()];
        let excludes = vec!["chore".to_string()];
        let filtered = filter_logs(&logs, &authors, &includes, &excludes).unwrap();
        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().all(|log| !log.contains("chore")));
    }

    #[test]
    fn test_filter_combined() {
        let logs = logs();
        let authors = vec!["Alice".to_string()];
        let includes = vec!["docs".to_string()];
        let excludes = vec!["".to_string()];
        let filtered = filter_logs(&logs, &authors, &includes, &excludes).unwrap();
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("docs: update"));
    }

    #[test]
    fn test_filter_none() {
        let logs = logs();
        let authors = vec!["NonExist".to_string()];
        let includes = vec!["feat".to_string()];
        let excludes = vec!["".to_string()];
        let filtered = filter_logs(&logs, &authors, &includes, &excludes).unwrap();
        assert_eq!(filtered.len(), 0);
    }
}
