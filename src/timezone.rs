use crate::{DateTime, NaiveDateTime};
use core::convert::Infallible;
use core::fmt::Display;

/// A type that can be used to represent a `TimeZone`
pub trait TimeZone: Sized + Eq + Display {
	/// The error to return in case of a failure to convert the local time to UTC
	type Err;

	/// Given the time in the UTC timezone, determine the `UtcOffset`
	fn utc_offset(&self, date_time: DateTime<Utc>) -> UtcOffset;

	/// Given the local date and time, figure out the offset from UTC
	///
	/// # Errors
	///
	/// This returns an Err if the given `NaiveDateTime` cannot exist in this timezone.
	/// For example, the time may have been skipped because of daylight savings time.
	fn offset_from_local_time(&self, date_time: NaiveDateTime) -> Result<UtcOffset, Self::Err>;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// The UTC time zone
pub struct Utc;

impl TimeZone for Utc {
	type Err = Infallible;

	fn utc_offset(&self, _: DateTime<Utc>) -> UtcOffset {
		UtcOffset::UTC
	}

	fn offset_from_local_time(&self, _: NaiveDateTime) -> Result<UtcOffset, Self::Err> {
		Ok(UtcOffset::UTC)
	}
}

impl Display for Utc {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "UTC")
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// A timezone with a fixed offset from UTC
pub struct UtcOffset {
	offset_seconds: i32,
}

impl UtcOffset {
	/// The UTC Timezone, represented as an offset
	pub const UTC: Self = Self { offset_seconds: 0 };

	/// Makes a new `UtcOffset` timezone with the given timezone difference.
	/// A positive number is the Eastern hemisphere. A negative number is the
	/// Western hemisphere.
	#[must_use]
	pub const fn from_seconds(seconds: i32) -> Self {
		Self {
			offset_seconds: seconds,
		}
	}

	/// Makes a new `UtcOffset` timezone with the given timezone difference.
	/// A positive number is the Eastern hemisphere. A negative number is the
	/// Western hemisphere.
	#[must_use]
	pub const fn from_hours(hours: i32) -> Self {
		Self::from_seconds(hours * 3600)
	}

	/// The number of hours this timezone is ahead of UTC. This number is
	/// negative if the timezone is in the Western hemisphere
	#[must_use]
	pub fn hours_ahead(self) -> f32 {
		self.offset_seconds as f32 / 3600.0
	}

	/// The number of seconds this timezone is ahead of UTC. This number is
	/// negative if the timezone is in the Western hemisphere
	#[must_use]
	pub const fn seconds_ahead(self) -> i32 {
		self.offset_seconds
	}
}

impl Display for UtcOffset {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let hours = self.offset_seconds / 3600;
		let minutes = ((self.offset_seconds % 3600) / 60).abs();
		let seconds = (self.offset_seconds % 60).abs();
		let sign = if self.offset_seconds.is_negative() {
			'-'
		} else {
			'+'
		};

		if self.offset_seconds == 0 {
			write!(f, "UTC")
		} else if self.offset_seconds % 3600 == 0 {
			write!(f, "UTC{:+}", hours)
		} else if self.offset_seconds % 60 == 0 {
			write!(f, "UTC{}{:02}:{:02}", sign, hours.abs(), minutes)
		} else {
			write!(
				f,
				"UTC{}{:02}:{:02}:{:02}",
				sign,
				hours.abs(),
				minutes,
				seconds
			)
		}
	}
}

impl TimeZone for UtcOffset {
	type Err = Infallible;

	fn utc_offset(&self, _: DateTime<Utc>) -> UtcOffset {
		*self
	}

	fn offset_from_local_time(&self, _: NaiveDateTime) -> Result<UtcOffset, Self::Err> {
		Ok(*self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn utc_offset_display_no_offset() {
		let offset = UtcOffset::UTC;
		let offset_str = offset.to_string();
		assert_eq!(offset_str, "UTC");
	}

	#[test]
	fn utc_offset_display_positive_offset() {
		let offset = UtcOffset::from_hours(1);
		let offset_str = offset.to_string();
		assert_eq!(offset_str, "UTC+1");
	}

	#[test]
	fn utc_offset_display_minute_offset() {
		let offset = UtcOffset::from_seconds(60);
		let offset_str = offset.to_string();
		assert_eq!(offset_str, "UTC+00:01");
	}

	#[test]
	fn utc_offset_display_second_offset() {
		let offset = UtcOffset::from_seconds(-32);
		let offset_str = offset.to_string();
		assert_eq!(offset_str, "UTC-00:00:32");
	}
}
