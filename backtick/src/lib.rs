pub mod send;
pub mod thread;
pub mod file;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// I literally just want to pump this out so I can get continue working on other things.
// So there might not be many comments, but worry not - this will get updated as it needs to be.

// However, I am still commited to making sure it is of quality.