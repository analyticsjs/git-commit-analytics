use crate::utils::format_commit::{format_commit, CommitInfo};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

#[allow(dead_code)]
#[derive(Debug)]
pub struct LogInfo {
    pub repo: String,
    pub author: String,
    pub email: String,
    pub commit: String,
    pub type_name: String,
    pub category: String,
    pub message: String,
    pub hash: String,
    pub time: String,
    pub unix: i64,
}

fn parse_time(time_str: &str) -> (String, i64) {
    // Try RFC2822 first
    if let Ok(dt) = DateTime::parse_from_rfc2822(time_str) {
        let local_dt: DateTime<Local> = dt.with_timezone(&Local);
        (
            local_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            local_dt.timestamp_millis(),
        )
    }
    // Try common format
    else if let Ok(naive_dt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        // Use local timezone
        let local_dt = Local.from_local_datetime(&naive_dt).unwrap();
        (
            local_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            local_dt.timestamp_millis(),
        )
    }
    // Parse failed
    else {
        (time_str.to_string(), 0)
    }
}

/// Format log
pub fn format_log(log: &str) -> LogInfo {
    let arr: Vec<&str> = log.split("|||").collect();

    let repo = arr.get(0).unwrap_or(&"").to_string();
    let author = arr.get(1).unwrap_or(&"").to_string();
    let email = arr.get(2).unwrap_or(&"").to_string();
    let commit = arr.get(3).unwrap_or(&"").to_string();
    let hash = arr.get(4).unwrap_or(&"").replace("'", "#");
    let time_str = arr.get(5).unwrap_or(&"").to_string();

    let (time, unix) = parse_time(&time_str);

    let CommitInfo {
        type_name,
        category,
        message,
    } = format_commit(&commit);

    LogInfo {
        repo,
        author,
        email,
        commit,
        type_name,
        category,
        message,
        hash,
        time,
        unix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_log_basic() {
        let log =
            "repo1|||Alice|||alice@example.com|||feat: add feature|||'abc123|||2024-06-01 12:00:00";
        let info = format_log(log);
        assert_eq!(info.repo, "repo1");
        assert_eq!(info.author, "Alice");
        assert_eq!(info.email, "alice@example.com");
        assert_eq!(info.commit, "feat: add feature");
        assert_eq!(info.hash, "#abc123");
        assert_eq!(info.time, "2024-06-01 12:00:00");
        assert_eq!(info.type_name, "feat");
        assert_eq!(info.message, "add feature");
    }
}
