extern crate chrono;
use chrono::*;

pub fn after(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt + Duration::seconds(1_000_000_000)
}
