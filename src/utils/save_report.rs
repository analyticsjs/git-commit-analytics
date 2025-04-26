use crate::i18n::t;
use crate::utils::format_log::LogInfo;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const CATEGORY_ORDER: &[&str] = &["feat", "fix", "docs", "style", "refactor", "test", "chore"];

fn get_category_label(type_name: &str) -> &str {
    match type_name {
        "feat" => t("commit_category_features"),
        "fix" => t("commit_category_bug_fixes"),
        "docs" => t("commit_category_docs"),
        "style" => t("commit_category_style"),
        "refactor" => t("commit_category_refactor"),
        "test" => t("commit_category_test"),
        "chore" => t("commit_category_chores"),
        _ => type_name,
    }
}

/// Save Markdown report
pub fn save_report_markdown(
    result: &HashMap<String, HashMap<String, Vec<LogInfo>>>,
    path: &str,
) -> std::io::Result<()> {
    // Check if result is empty
    if result.is_empty() {
        println!("{}", t("no_report_generated"));
        return Ok(());
    }

    let mut md = String::new();
    let mut repo_titles = vec![];

    for (repo, type_map) in result {
        if !repo_titles.contains(repo) {
            md.push_str(&format!("## {}\n\n", repo));
            repo_titles.push(repo.to_string());
        }

        for &cat in CATEGORY_ORDER {
            if let Some(logs) = type_map.get(cat) {
                let label = get_category_label(cat);
                md.push_str(&format!("### {}\n\n", label));
                for (index, log) in logs.iter().enumerate() {
                    md.push_str(&format!("{}. {}\n", index + 1, log.message));
                }
                md.push('\n');
            }
        }
        md.push('\n');
    }

    let mut file = File::create(Path::new(path))?;
    file.write_all(md.as_bytes())?;

    println!("{}", t("report_saved").replace("{}", path));
    println!("");

    Ok(())
}
