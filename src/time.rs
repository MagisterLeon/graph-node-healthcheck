use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_as_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}