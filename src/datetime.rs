use crate::{Date, Month, Time, Year};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NaiveDateTime {
	date: Date,
	time: Time,
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

// TODO addition
