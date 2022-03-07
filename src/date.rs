use crate::{Month, Year};

use core::cmp::Ordering;
use core::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Date {
	year: Year,
	month: Month,
	day: u8,
}

impl Date {
	/// The earliest date which can be represented
	pub const MIN: Self = unsafe { Self::from_ymd_unchecked(Year::MIN, Month::January, 1) };

	/// The latest date which can be represented
	pub const MAX: Self = unsafe { Self::from_ymd_unchecked(Year::MAX, Month::December, 31) };

	pub const UNIX_EPOCH: Self =
		unsafe { Self::from_ymd_unchecked(Year::from_i16(1970), Month::January, 1) };

	// TODO validated from_calendar_date

	/// Creates a date without checking to make sure that it's valid.
	///
	/// # Example
	///
	/// ```
	/// use botic::{Date, Month, Year};
	///
	/// let y2k = unsafe {
	///     Date::from_ymd_unchecked(Year::from(2000), Month::January, 1)
	/// };
	/// ```
	///
	/// # Safety
	///
	/// This function results in undefined behavior if the given date is not a real date
	#[must_use]
	pub const unsafe fn from_ymd_unchecked(year: Year, month: Month, day: u8) -> Self {
		Self { year, month, day }
	}

	// TODO docs

	#[must_use]
	pub const fn year(self) -> Year {
		self.year
	}

	#[must_use]
	pub const fn month(self) -> Month {
		self.month
	}

	#[must_use]
	pub const fn day(self) -> u8 {
		self.day
	}

	#[must_use]
	pub const fn is_leap_year(self) -> bool {
		self.year.is_leap_year()
	}

	// TODO handle BCE properly
	#[must_use]
	pub const fn days_after_common_era(self) -> isize {
		let year = self.year.wrapping_sub(1);
		let leap_years = (year.as_i16() / 4 - year.as_i16() / 100 + year.as_i16() / 400) as isize;
		let month_last_day_ordinal = self.month.last_day_ordinal(self.is_leap_year()) as isize;

		year.as_i16() as isize * 365 + leap_years + month_last_day_ordinal + self.day as isize - 1
	}

	// TODO test
	#[must_use]
	pub const fn from_days_after_common_era(days: isize) -> Self {
		let era = days / 146097; // an era is a period of 400 year
		let day_of_era = days - (era * 146097);
		let year_of_era = day_of_era / 365;
		let year = year_of_era + (era * 400);
		let ordinal = day_of_era - (365 * year + year / 4 - year / 100);
		// TODO look at as's
		let year = Year::from_i16(year as i16);
		let month = Month::from_ordinal(ordinal as u16, year.is_leap_year());
		let day = ordinal as u16 - month.previous().last_day_ordinal(year.is_leap_year());
		let day = day as u8;

		unsafe { Self::from_ymd_unchecked(year, month, day) }
	}

	#[must_use]
	pub const fn add_days(self, days: isize) -> Self {
		let total_days_since_ce = self.days_after_common_era() + days;
		Self::from_days_after_common_era(total_days_since_ce)
	}
}

impl PartialOrd for Date {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let year_ordering = self.year.cmp(&other.year);
		let month_ordering = self.month.cmp(&other.month);
		let day_ordering = self.day.cmp(&other.day);

		if year_ordering != Ordering::Equal {
			Some(year_ordering)
		} else if month_ordering != Ordering::Equal {
			Some(month_ordering)
		} else if day_ordering != Ordering::Equal {
			Some(day_ordering)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl Ord for Date {
	fn cmp(&self, other: &Self) -> Ordering {
		let year_ordering = self.year.cmp(&other.year);
		let month_ordering = self.month.cmp(&other.month);
		let day_ordering = self.day.cmp(&other.day);

		if year_ordering != Ordering::Equal {
			year_ordering
		} else if month_ordering != Ordering::Equal {
			month_ordering
		} else if day_ordering != Ordering::Equal {
			day_ordering
		} else {
			Ordering::Equal
		}
	}
}

// TODO addition

impl Display for Date {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{:0width$}-{:02}-{:02}",
			self.year,
			self.month as u8,
			self.day,
			width = 4 + (self.year() < 0.into()) as usize
		)
	}
}
