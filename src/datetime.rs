use crate::{
	date::{DayGreaterThanMaximumForMonthError, LeapDayNotInLeapYearError},
	tai::Tai,
	timezone::{Utc, UtcOffset},
	Date, Month, Time, TimeZone, Timestamp, Year,
};

use core::{cmp::Ordering, fmt::Display, hash::Hash};
use std::time::SystemTime;

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

	pub fn from_local(local_datetime: NaiveDateTime, timezone: Tz) -> Result<Self, Tz::Err> {
		let offset = timezone.offset_from_local_naive(local_datetime)?;
		// TODO overflow
		let utc_datetime = local_datetime
			.add_seconds_overflowing(-offset.seconds_ahead() as i64)
			.0;

		Ok(Self::from_utc(utc_datetime, timezone))
	}

	pub fn system_time(timezone: Tz) -> Self {
		let system_time = SystemTime::now();
		let (seconds, nanoseconds) = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
			Ok(duration) => (duration.as_secs() as i64, duration.subsec_nanos()),
			Err(ste) => (
				-(ste.duration().as_secs() as i64),
				ste.duration().subsec_nanos(),
			),
		};
		let timestamp = Timestamp::new(seconds, nanoseconds);
		let naive_dt = NaiveDateTime::from_timestamp(timestamp);

		Self::from_utc(naive_dt, timezone)
	}

	pub fn offset(&self) -> UtcOffset {
		let utc = self.as_utc();
		self.timezone.utc_offset(utc)
	}

	pub fn timezone(&self) -> &Tz {
		&self.timezone
	}

	pub fn naive_utc(&self) -> NaiveDateTime {
		self.utc_datetime
	}

	pub fn to_naive_overflowing(&self) -> (NaiveDateTime, bool) {
		self.utc_datetime
			.add_seconds_overflowing(self.offset().seconds_ahead().into())
	}

	pub fn into_timezone<NewZone: TimeZone>(&self, timezone: NewZone) -> DateTime<NewZone> {
		DateTime::<NewZone>::from_utc(self.utc_datetime, timezone)
	}

	pub fn as_utc(&self) -> DateTime<Utc> {
		self.into_timezone(Utc)
	}

	pub fn as_tai(&self) -> DateTime<Tai> {
		self.into_timezone(Tai)
	}

	pub fn unix_timestamp(&self) -> Timestamp {
		self.utc_datetime.timestamp()
	}

	// TODO should this overflow?
	pub fn tai_timestamp(&self) -> Timestamp {
		self.as_tai().to_naive_overflowing().0.timestamp()
	}

	#[must_use]
	pub fn add_seconds_overflowing(self, seconds: i64) -> (Self, bool) {
		let (tai_timestamp, overflow) = self.tai_timestamp().add_seconds_overflowing(seconds);
		let tai_naive_dt = NaiveDateTime::from_timestamp(tai_timestamp);
		let tai_dt = DateTime::from_local(tai_naive_dt, Tai).unwrap();

		(tai_dt.into_timezone(self.timezone), overflow)
	}

	#[must_use]
	pub fn add_nanoseconds_overflowing(self, nanoseconds: i64) -> (Self, bool) {
		let (tai_timestamp, overflow) = self
			.tai_timestamp()
			.add_nanoseconds_overflowing(nanoseconds);
		let tai_naive_dt = NaiveDateTime::from_timestamp(tai_timestamp);
		let tai_dt = DateTime::from_local(tai_naive_dt, Tai).unwrap();

		(tai_dt.into_timezone(self.timezone), overflow)
	}
}

impl NaiveDateTime {
	// TODO docs

	#[must_use]
	pub const fn new(date: Date, time: Time) -> Self {
		Self { date, time }
	}

	pub const fn from_timestamp(timestamp: Timestamp) -> Self {
		const UNIX_EPOCH_DAYS_AFTER_CE: i64 = Date::UNIX_EPOCH.days_after_common_era();
		let days_after_unix_epoch = timestamp.total_seconds() / 86_400;
		let days_after_ce = days_after_unix_epoch + UNIX_EPOCH_DAYS_AFTER_CE as i64;
		let date = Date::from_days_after_common_era(days_after_ce);
		let seconds_after_midnight = timestamp.total_seconds() % 86_400;
		let nanoseconds = timestamp.nanosecond();
		let time = Time::MIDNIGHT
			.add_seconds_overflowing(seconds_after_midnight as isize)
			.0
			.add_nanoseconds_overflowing(nanoseconds as isize)
			.0;

		Self::new(date, time)
	}

	#[must_use]
	pub const fn date(self) -> Date {
		self.date
	}

	#[must_use]
	pub const fn time(self) -> Time {
		self.time
	}

	#[must_use]
	pub const fn year(self) -> Year {
		self.date.year()
	}

	#[must_use]
	pub const fn month(self) -> Month {
		self.date.month()
	}

	#[must_use]
	pub const fn day(self) -> u8 {
		self.date.day()
	}

	#[must_use]
	pub const fn hour(self) -> u8 {
		self.time.hour()
	}

	#[must_use]
	pub const fn minute(self) -> u8 {
		self.time.minute()
	}

	#[must_use]
	pub const fn second(self) -> u8 {
		self.time.second()
	}

	#[must_use]
	pub const fn millisecond(self) -> u16 {
		self.time.millisecond()
	}

	#[must_use]
	pub const fn microsecond(self) -> u32 {
		self.time.microsecond()
	}

	#[must_use]
	pub const fn nanosecond(self) -> u32 {
		self.time.nanosecond()
	}

	#[must_use]
	pub const fn timestamp(self) -> Timestamp {
		const UNIX_EPOCH_DAYS: i64 = Date::UNIX_EPOCH.days_after_common_era();
		// TODO don't require the .date()
		let days = (self.date.days_after_common_era() - UNIX_EPOCH_DAYS) as i64;
		let seconds = days * 86_400 + self.time().seconds_from_midnight() as i64;
		let nanoseconds = self.nanosecond();

		Timestamp::new(seconds, nanoseconds)
	}

	pub const fn add_years_overflowing(
		self,
		years: i16,
	) -> Result<(Self, bool), LeapDayNotInLeapYearError> {
		let (date, overflow) = match self.date.add_years_overflowing(years) {
			Ok(v) => v,
			Err(e) => return Err(e),
		};

		Ok((
			Self {
				date,
				time: self.time,
			},
			overflow,
		))
	}

	pub const fn add_months_overflowing(
		self,
		months: i8,
	) -> Result<(Self, bool), DayGreaterThanMaximumForMonthError> {
		let (date, overflow) = match self.date.add_months_overflowing(months) {
			Ok(v) => v,
			Err(e) => return Err(e),
		};

		Ok((
			Self {
				date,
				time: self.time,
			},
			overflow,
		))
	}

	#[must_use]
	pub const fn add_days_overflowing(self, days: i64) -> (Self, bool) {
		let (date, overflow) = self.date.add_days_overflowing(days);

		(
			Self {
				date,
				time: self.time,
			},
			overflow,
		)
	}

	#[must_use]
	pub const fn add_hours_overflowing(self, hours: i64) -> (Self, bool) {
		let timestamp: Timestamp = self.timestamp();
		let (timestamp, overflow) = timestamp.add_hours_overflowing(hours);
		let datetime: NaiveDateTime = Self::from_timestamp(timestamp);

		(datetime, overflow)
	}

	#[must_use]
	pub const fn add_minutes_overflowing(self, minutes: i64) -> (Self, bool) {
		let timestamp: Timestamp = self.timestamp();
		let (timestamp, overflow) = timestamp.add_minutes_overflowing(minutes);
		let datetime: NaiveDateTime = Self::from_timestamp(timestamp);

		(datetime, overflow)
	}

	#[must_use]
	pub const fn add_seconds_overflowing(self, seconds: i64) -> (Self, bool) {
		let timestamp: Timestamp = self.timestamp();
		let (timestamp, overflow) = timestamp.add_seconds_overflowing(seconds);
		let datetime: NaiveDateTime = Self::from_timestamp(timestamp);

		(datetime, overflow)
	}

	#[must_use]
	pub const fn add_nanoseconds_overflowing(self, nanoseconds: i64) -> (Self, bool) {
		let timestamp: Timestamp = self.timestamp();
		let (timestamp, overflow) = timestamp.add_nanoseconds_overflowing(nanoseconds);
		let datetime: NaiveDateTime = Self::from_timestamp(timestamp);

		(datetime, overflow)
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
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		self.utc_datetime.hash(state);
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

impl Display for NaiveDateTime {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{} {}", self.date, self.time)
	}
}

impl<Tz: TimeZone> Display for DateTime<Tz> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{} {}", self.utc_datetime, self.timezone)
	}
}

// TODO there's a lossy cast somewhere here or in the into(). Where is it?
impl From<Timestamp> for NaiveDateTime {
	fn from(timestamp: Timestamp) -> Self {
		const UNIX_EPOCH_DAYS_AFTER_CE: i64 = Date::UNIX_EPOCH.days_after_common_era();
		let days_after_unix_epoch = timestamp.total_seconds() / 86_400;
		let days_after_ce = days_after_unix_epoch + UNIX_EPOCH_DAYS_AFTER_CE as i64;
		let date = Date::from_days_after_common_era(days_after_ce);
		let seconds_after_midnight = timestamp.total_seconds() % 86_400;
		let nanoseconds = timestamp.nanosecond();
		let time = Time::MIDNIGHT
			.add_seconds(seconds_after_midnight as isize)
			.add_nanoseconds(nanoseconds as isize);

		Self::new(date, time)
	}
}
