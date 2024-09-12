use chrono::{DateTime, TimeZone, Utc};


/// A trait representing the internal clock for timekeeping requirements.
/// Note that for some testing environments, clocks may be stubs.
pub trait Clock<Tz: TimeZone> {
    fn now(tz: Tz) -> DateTime<Tz>;
}

pub struct SystemClock<Tz: TimeZone> {
    time_zone: std::marker::PhantomData<*const Tz>,
}

impl<Tz: TimeZone> Clock<Tz> for SystemClock<Tz> {
    fn now(tz: Tz) -> DateTime<Tz> {
        Utc::now().with_timezone(&tz)
    }
}