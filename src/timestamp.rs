use crate::{Date, NaiveDateTime};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct UnixTimestamp {
	seconds: i64,
	nanoseconds: u32,
}

impl UnixTimestamp {
	#[must_use]
	pub const fn new(seconds: i64, nanoseconds: u32) -> Self {
		Self {
			seconds,
			nanoseconds,
		}
	}

	#[must_use]
	pub const fn seconds_since_unix_epoch(self) -> i64 {
		self.seconds
	}

	#[must_use]
	pub const fn nanosecond(self) -> u32 {
		self.nanoseconds
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

impl From<NaiveDateTime> for UnixTimestamp {
	fn from(ndt: NaiveDateTime) -> Self {
		const UNIX_EPOCH_DAYS: i64 = Date::UNIX_EPOCH.days_after_common_era();
		// TODO don't require the .date()
		let days = (ndt.date().days_after_common_era() - UNIX_EPOCH_DAYS) as i64;
		let seconds = days * 86_400 + i64::from(ndt.time().seconds_from_midnight());
		let nanoseconds = ndt.nanosecond();

		Self::new(seconds, nanoseconds)
	}
}

impl PartialOrd for UnixTimestamp {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		match self.seconds.partial_cmp(&other.seconds) {
			Some(core::cmp::Ordering::Equal) => self.nanoseconds.partial_cmp(&other.nanoseconds),
			ord => ord,
		}
	}
}

impl Ord for UnixTimestamp {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		match self.seconds.cmp(&other.seconds) {
			core::cmp::Ordering::Equal => self.nanoseconds.cmp(&other.nanoseconds),
			ord => ord,
		}
	}
}
