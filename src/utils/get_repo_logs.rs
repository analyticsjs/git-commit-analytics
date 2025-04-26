use crate::i18n::t;
use std::path::Path;
use std::process::Command;

/// Get repository logs
pub fn get_repo_logs(repo_dir: &str) -> anyhow::Result<Vec<String>> {
    let path = Path::new(repo_dir);
    if !path.exists() || !path.is_dir() {
        anyhow::bail!(t("err_repo_not_found").replace("{}", &repo_dir));
    }

    // Git log format reference
    // https://git-scm.com/docs/pretty-formats
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%an|||%ae|||%s|||'%h|||%ad")
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "{} {}",
            t("err_git_log_failed").replace("{}", &repo_dir),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_git_log() {
        let repo_dir = "/Users/xxx/projects/my-repo";
        let logs = get_repo_logs(repo_dir).unwrap();
        assert!(!logs.is_empty());
    }
}
