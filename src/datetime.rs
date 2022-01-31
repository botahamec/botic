use crate::{
	timezone::{Utc, UtcOffset},
	Date, Month, Time, TimeZone, Year,
};

use core::{cmp::Ordering, hash::Hash};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NaiveDateTime {
	date: Date,
	time: Time,
}

#[derive(Copy, Clone, Eq, Debug)]
pub struct DateTime<Tz: TimeZone> {
	utc_datetime: NaiveDateTime,
	timezone: Tz,
}

impl<Tz: TimeZone> DateTime<Tz> {
	// TODO unix epoch constant
	// TODO docs

	pub fn from_utc(utc_datetime: NaiveDateTime, timezone: Tz) -> Self {
		Self {
			utc_datetime,
			timezone,
		}
	}

	pub fn offset(&self) -> UtcOffset {
		let utc = DateTime::<Utc>::from_utc(self.utc_datetime, Utc);
		self.timezone.utc_offset(utc)
	}

	pub fn timezone(&self) -> &Tz {
		&self.timezone
	}

	pub fn naive_utc(&self) -> NaiveDateTime {
		self.utc_datetime
	}
}

impl NaiveDateTime {
	// TODO docs

	pub const fn new(date: Date, time: Time) -> Self {
		Self { date, time }
	}

	pub const fn date(self) -> Date {
		self.date
	}

	pub const fn time(self) -> Time {
		self.time
	}

	pub const fn year(self) -> Year {
		self.date.year()
	}

	pub const fn month(self) -> Month {
		self.date.month()
	}

	pub const fn day(self) -> u8 {
		self.date.day()
	}

	pub const fn hour(self) -> u8 {
		self.time.hour()
	}

	pub const fn minute(self) -> u8 {
		self.time.minute()
	}

	pub const fn second(self) -> u8 {
		self.time.second()
	}

	pub const fn millisecond(self) -> u16 {
		self.time.millisecond()
	}

	pub const fn microsecond(self) -> u32 {
		self.time.microsecond()
	}

	pub const fn nanosecond(self) -> u32 {
		self.time.nanosecond()
	}
}

impl PartialOrd for NaiveDateTime {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let date_ordering = self.date.cmp(&other.date);
		let time_ordering = self.time.cmp(&other.time);

		if date_ordering != Ordering::Equal {
			Some(date_ordering)
		} else if time_ordering != Ordering::Equal {
			Some(time_ordering)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl Ord for NaiveDateTime {
	fn cmp(&self, other: &Self) -> Ordering {
		let date_ordering = self.date.cmp(&other.date);
		let time_ordering = self.time.cmp(&other.time);

		if date_ordering != Ordering::Equal {
			date_ordering
		} else if time_ordering != Ordering::Equal {
			time_ordering
		} else {
			Ordering::Equal
		}
	}
}

// TODO think harder about the fact that we don't consider timezone (how will UtcOffset work)
impl<Tz: TimeZone, Other: TimeZone> PartialEq<DateTime<Other>> for DateTime<Tz> {
	fn eq(&self, other: &DateTime<Other>) -> bool {
		self.utc_datetime == other.utc_datetime
	}
}

impl<Tz: TimeZone> Hash for DateTime<Tz> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.utc_datetime.hash(state)
	}
}

impl<Tz: TimeZone, Other: TimeZone> PartialOrd<DateTime<Other>> for DateTime<Tz> {
	fn partial_cmp(&self, other: &DateTime<Other>) -> Option<Ordering> {
		self.utc_datetime.partial_cmp(&other.utc_datetime)
	}
}

impl<Tz: TimeZone> Ord for DateTime<Tz> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.utc_datetime.cmp(&other.utc_datetime)
	}
}

// TODO addition
