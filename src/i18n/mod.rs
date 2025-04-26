mod messages {
    pub mod en;
    pub mod zh;
}

use crate::config;

pub fn t<'a>(key: &'a str) -> &'a str {
    let messages = if config::is_chinese() {
        &messages::zh::MESSAGES
    } else {
        &messages::en::MESSAGES
    };

    messages.get(key).copied().unwrap_or(key)
}
