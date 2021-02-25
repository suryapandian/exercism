use chrono::Duration;
use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
const GIGA: i64 = 1000000000;
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(GIGA)
}
