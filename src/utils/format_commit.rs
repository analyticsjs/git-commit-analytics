use crate::i18n::t;

/// Commit type and its category
pub struct CommitInfo {
    pub type_name: String,
    pub category: String,
    pub message: String,
}

/// Only used for testing
#[cfg(test)]
const TEST_CATEGORIES: &[(&str, &str)] = &[
    ("feat", "Features"),
    ("fix", "Bug Fixes"),
    ("docs", "Documentation"),
    ("style", "Styles"),
    ("refactor", "Code Refactoring"),
    ("test", "Tests"),
    ("chore", "Chores"),
];

#[cfg(test)]
fn get_commit_type_and_category(type_name: &str) -> (&str, String) {
    let (t, c) = TEST_CATEGORIES
        .iter()
        .find(|(t, _)| *t == type_name)
        .map(|&(t, c)| (t, c))
        .unwrap_or(("chore", "Chores"));
    (t, c.to_string())
}

#[cfg(not(test))]
fn get_commit_type_and_category(type_name: &str) -> (&str, String) {
    match type_name {
        "feat" => ("feat", t("commit_category_features").to_string()),
        "fix" => ("fix", t("commit_category_bug_fixes").to_string()),
        "docs" => ("docs", t("commit_category_docs").to_string()),
        "style" => ("style", t("commit_category_style").to_string()),
        "refactor" => ("refactor", t("commit_category_refactor").to_string()),
        "test" => ("test", t("commit_category_test").to_string()),
        _ => ("chore", t("commit_category_chores").to_string()),
    }
}

/// Format commit message
pub fn format_commit(commit: &str) -> CommitInfo {
    let commit_type = if let Some(index) = commit.find(':') {
        &commit[..index]
    } else {
        "chore"
    };

    let base_type = if let Some(scope_start) = commit_type.find('(') {
        &commit_type[..scope_start]
    } else {
        commit_type
    };

    let (type_name, category) = get_commit_type_and_category(base_type);

    // Extract commit message
    let mut message = commit.trim().to_string();

    // Get commit message (remove type prefix)
    if let Some(index) = commit.find(':') {
        message = commit[index + 1..].trim().to_string();

        // 提取 scope
        let action = &commit[..index];
        if let Some(scope) = action.find('(').and_then(|start| {
            action
                .find(')')
                .map(|end| action[start + 1..end].to_string())
        }) {
            message = format!("{}: {}", scope, message);
        }
    }

    CommitInfo {
        type_name: type_name.to_string(),
        category,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic_commit() {
        let commit = "feat: add new feature";
        let info = format_commit(commit);
        assert_eq!(info.type_name, "feat");
        assert_eq!(info.category, "Features");
        assert_eq!(info.message, "add new feature");
    }

    #[test]
    fn test_format_commit_with_scope() {
        let commit = "feat(component): add button component";
        let info = format_commit(commit);
        assert_eq!(info.type_name, "feat");
        assert_eq!(info.category, "Features");
        assert_eq!(info.message, "component: add button component");
    }

    #[test]
    fn test_commit_types() {
        let test_cases = [
            ("feat: new", "feat", "Features"),
            ("fix: bug", "fix", "Bug Fixes"),
            ("docs: update", "docs", "Documentation"),
            ("style: format", "style", "Styles"),
            ("refactor: code", "refactor", "Code Refactoring"),
            ("test: add", "test", "Tests"),
            ("chore: deps", "chore", "Chores"),
            ("unknown: something", "chore", "Chores"),
        ];

        for (commit, expected_type, expected_category) in test_cases {
            let info = format_commit(commit);
            assert_eq!(info.type_name, expected_type);
            assert_eq!(info.category, expected_category);
        }
    }
}
