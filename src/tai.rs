use core::cmp::Ordering;
use core::fmt::Display;

use parking_lot::{const_rwlock, RwLock};
use thiserror::Error;

use crate::{
	timezone::{Utc, UtcOffset},
	Date, DateTime, NaiveDateTime, Time, TimeZone,
};

static GLOBAL_LEAP_SECONDS: RwLock<LeapSeconds> = const_rwlock(LeapSeconds::empty());

#[derive(Debug)]
struct LeapSeconds(Vec<DateTime<Utc>>);

impl LeapSeconds {
	// TODO docs

	const fn empty() -> Self {
		Self(Vec::new())
	}

	fn leap_seconds_before_inclusive(&self, date_time: DateTime<Utc>) -> usize {
		let mut seconds = 0;
		for leap_second in &self.0 {
			if leap_second > &date_time {
				break;
			}
			seconds += 1;
		}

		seconds
	}

	fn add_leap_second(&mut self, day: Date) {
		let utc_datetime = NaiveDateTime::new(day, Time::MIDNIGHT);
		let exact_time = DateTime::from_utc(utc_datetime, Utc);

		let mut i = 0;
		while i < self.0.len() {
			match self.0[i].cmp(&exact_time) {
				Ordering::Greater => break, // insert the new leap second here
				Ordering::Equal => return,  // it's already here, so don't add it again
				Ordering::Less => i += 1,   // check the next leap second
			}
		}

		self.0.insert(i, exact_time);
	}
}

pub fn add_leap_second(day: Date) {
	let mut leap_seconds = GLOBAL_LEAP_SECONDS.write();
	leap_seconds.add_leap_second(day);
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Tai;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
#[error(
	"TAI cannot represent leap seconds, so a leap second cannot be converted to TAI. Recieved: {}",
	given_dt
)]
pub struct UnexpectedLeapSecond {
	given_dt: NaiveDateTime,
}

impl Display for Tai {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "TAI")
	}
}

impl TimeZone for Tai {
	type Err = UnexpectedLeapSecond;

	fn utc_offset(&self, date_time: DateTime<Utc>) -> UtcOffset {
		let leap_seconds = GLOBAL_LEAP_SECONDS.read();
		let past_leap_seconds = leap_seconds.leap_seconds_before_inclusive(date_time);
		UtcOffset::from_seconds(-(past_leap_seconds as i32 + 10))
	}

	// TODO optimize
	fn offset_from_local_naive(&self, date_time: NaiveDateTime) -> Result<UtcOffset, Self::Err> {
		// TAI times cannot have leap seconds
		if date_time.second() == 60 {
			return Err(UnexpectedLeapSecond {
				given_dt: date_time,
			});
		}

		// calculate the number of seconds that have passed since date_time in UTC
		let leap_seconds = GLOBAL_LEAP_SECONDS.read();
		let utc_dt = DateTime::from_utc(date_time, Utc);
		let mut past_leap_seconds = dbg!(leap_seconds.leap_seconds_before_inclusive(utc_dt));
		let mut prev_pls = 0; // use this to see if the number of leap seconds has been updated

		// check if any leap seconds were found because of this calculation
		// keep checking until there is no longer a change in the total leap seconds
		while past_leap_seconds != prev_pls {
			prev_pls = past_leap_seconds;
			// TODO think about this discard
			let (ndt, _) = dbg!(date_time.add_seconds_overflowing(past_leap_seconds as i64));
			let utc_dt = DateTime::from_utc(ndt, Utc);
			past_leap_seconds = dbg!(leap_seconds.leap_seconds_before_inclusive(utc_dt));
		}

		Ok(UtcOffset::from_seconds(-(past_leap_seconds as i32 + 10)))
	}
}

#[cfg(test)]
mod tests {
	use crate::{Date, Month, Time};

	use super::*;

	#[test]
	fn test_conversion_no_leap_seconds() {
		let offset = unsafe {
			Tai.offset_from_local_naive(NaiveDateTime::new(
				Date::from_ymd_unchecked(2000.into(), Month::January, 1),
				Time::from_hms_unchecked(0, 0, 0),
			))
			.unwrap()
		};

		assert_eq!(offset, UtcOffset::from_seconds(-10));
	}

	#[test]
	fn test_conversion_one_leap_second() {
		add_leap_second(unsafe { Date::from_ymd_unchecked(2000.into(), Month::January, 1) });
		let offset = unsafe {
			Tai.offset_from_local_naive(NaiveDateTime::new(
				Date::from_ymd_unchecked(2000.into(), Month::January, 2),
				Time::from_hms_unchecked(0, 0, 0),
			))
			.unwrap()
		};

		assert_eq!(offset, UtcOffset::from_seconds(-11));
	}
}
