use crate::i18n::t;
use std::process::Command;

pub fn check_git_available() -> anyhow::Result<()> {
    let output = Command::new("git").arg("--version").output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => anyhow::bail!("\n{}\n", t("err_git_not_working")),
        Err(_) => anyhow::bail!("\n{}\n", t("err_git_not_found")),
    }
}
