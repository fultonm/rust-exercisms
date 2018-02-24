extern crate chrono;

use chrono::{Duration, DateTime, UTC};

// Does anyone know if I can some how store the gigasecond duration in a constant?
//const GIGASECOND: Duration = Duration::seconds(1_000_000_000);

// Returns a UTC DateTime one billion seconds after start.
pub fn after(start: DateTime<UTC>) -> DateTime<UTC> {
    let gigasecond: Duration = Duration::seconds(1_000_000_000);
    return start + gigasecond;
}
