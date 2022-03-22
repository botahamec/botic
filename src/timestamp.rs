use crate::{Date, NaiveDateTime};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Timestamp {
	seconds: i64,
	nanoseconds: u32,
}

impl Timestamp {
	#[must_use]
	pub const fn new(seconds: i64, nanoseconds: u32) -> Self {
		Self {
			seconds,
			nanoseconds,
		}
	}

	#[must_use]
	pub const fn total_seconds(self) -> i64 {
		self.seconds
	}

	#[must_use]
	pub const fn nanosecond(self) -> u32 {
		self.nanoseconds
	}

	#[must_use]
	pub const fn add_days_overflowing(self, days: i64) -> (Self, bool) {
		let (seconds, overflowing) = self.seconds.overflowing_add(days as i64 * 3600 * 24);

		let timestamp = Self::new(seconds, self.nanoseconds);
		(timestamp, overflowing)
	}

	#[must_use]
	pub const fn add_hours_overflowing(self, hours: i64) -> (Self, bool) {
		let (seconds, overflowing) = self.seconds.overflowing_add(hours as i64 * 3600);

		let timestamp = Self::new(seconds, self.nanoseconds);
		(timestamp, overflowing)
	}

	#[must_use]
	pub const fn add_minutes_overflowing(self, minutes: i64) -> (Self, bool) {
		let (seconds, overflowing) = self.seconds.overflowing_add(minutes as i64 * 60);

		let timestamp = Self::new(seconds, self.nanoseconds);
		(timestamp, overflowing)
	}

	#[must_use]
	pub const fn add_seconds_overflowing(self, seconds: i64) -> (Self, bool) {
		// TODO overflowing goes first
		let (seconds, overflowing) = self.seconds.overflowing_add(seconds as i64);

		let timestamp = Self::new(seconds, self.nanoseconds);
		(timestamp, overflowing)
	}

	#[must_use]
	pub const fn add_nanoseconds_overflowing(self, nanoseconds: i64) -> (Self, bool) {
		let total_nanos = (self.nanoseconds as i64 + nanoseconds) % 1_000_000_000;
		let total_nanos = total_nanos + (1_000_000_000 * total_nanos.is_negative() as i64);
		let added_seconds = (self.nanoseconds as i64 + nanoseconds) / 1_000_000_000;
		let total_seconds = (self.seconds as i64 + added_seconds) % 60;
		let overflow = 0 > total_seconds;
		let total_seconds = total_seconds + (60 * total_seconds.is_negative() as i64);

		let timestamp = Self::new(total_seconds, total_nanos as u32);
		(timestamp, overflow)
	}
}

impl From<NaiveDateTime> for Timestamp {
	fn from(ndt: NaiveDateTime) -> Self {
		const UNIX_EPOCH_DAYS: i64 = Date::UNIX_EPOCH.days_after_common_era();
		// TODO don't require the .date()
		let days = (ndt.date().days_after_common_era() - UNIX_EPOCH_DAYS) as i64;
		let seconds = days * 86_400 + i64::from(ndt.time().seconds_from_midnight());
		let nanoseconds = ndt.nanosecond();

		Self::new(seconds, nanoseconds)
	}
}

impl PartialOrd for Timestamp {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		match self.seconds.partial_cmp(&other.seconds) {
			Some(core::cmp::Ordering::Equal) => self.nanoseconds.partial_cmp(&other.nanoseconds),
			ord => ord,
		}
	}
}

impl Ord for Timestamp {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		match self.seconds.cmp(&other.seconds) {
			core::cmp::Ordering::Equal => self.nanoseconds.cmp(&other.nanoseconds),
			ord => ord,
		}
	}
}
