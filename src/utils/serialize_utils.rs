use serde::Serializer;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use log::warn;

/// Fix for forever bans in case of retry attempts. The method should never be called with
/// until_date in case of intentional forever ban
pub fn serialize_until_date<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(timestamp) = value {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::ZERO)
            .as_secs() as i64;

        if *timestamp < now + 60 {
            warn!("until_date clamped perhaps due to long retry attemps");
            serializer.serialize_some(&(now + 60))
        } else {
            let clamped = std::cmp::max(*timestamp, now + 60);
            serializer.serialize_some(&clamped)
        }
    } else {
        serializer.serialize_none()
    }
}
