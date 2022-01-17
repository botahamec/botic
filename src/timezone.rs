use crate::NaiveDateTime;
use core::convert::Infallible;

/// A type that can be used to represent a TimeZone
pub trait TimeZone {
	type Err;

	fn utc_offset(&self, date_time: NaiveDateTime) -> Result<UtcOffset, Self::Err>;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// The UTC time zone
pub struct Utc;

impl TimeZone for Utc {
	type Err = Infallible;

	fn utc_offset(&self, _: NaiveDateTime) -> Result<UtcOffset, Self::Err> {
		Ok(UtcOffset::UTC)
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

	// TODO validation

	/// Makes a new `UtcOffset` timezone with the given timezone difference.
	/// A positive number is the Eastern hemisphere. A negative number is the
	/// Western hemisphere.
	///
	/// # Safety
	///
	/// A value with an absolute value greater than or equal to 86,400 results
	/// in undefined behavior
	pub const unsafe fn from_seconds_unchecked(seconds: i32) -> Self {
		Self {
			offset_seconds: seconds,
		}
	}

	/// Makes a new `UtcOffset` timezone with the given timezone difference.
	/// A positive number is the Eastern hemisphere. A negative number is the
	/// Western hemisphere.
	///
	/// # Safety
	///
	/// A value with an absolute value greater than or equal to 24 results
	/// in undefined behavior
	pub const unsafe fn from_hours_unchecked(hours: i8) -> Self {
		Self::from_seconds_unchecked(hours as i32 * 3600)
	}

	/// The number of hours this timezone is ahead of UTC. THis number is
	/// negative if the timezone is in the Western hemisphere
	pub fn hours_ahead(self) -> f32 {
		self.offset_seconds as f32 / 3600.0
	}

	/// The number of seconds this timezone is ahead of UTC. This number is
	/// negative if the timezone is in the Western hemisphere
	pub const fn seconds_ahead(self) -> i32 {
		self.offset_seconds
	}
}

impl TimeZone for UtcOffset {
	type Err = Infallible;

	fn utc_offset(&self, _: NaiveDateTime) -> Result<UtcOffset, Self::Err> {
		Ok(*self)
	}
}
