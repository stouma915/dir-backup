use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn current_epoch_milli() -> Result<u128, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
}
