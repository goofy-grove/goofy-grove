use chrono::Utc;
use gg_core::domain::prelude::Clock;

#[derive(Debug, Clone)]
pub struct ChronoClock;

impl Clock for ChronoClock {
    fn timestamp(&self) -> i64 {
        Utc::now().timestamp()
    }
}
