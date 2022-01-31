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
	pub const MIN: Self =
		unsafe { Self::from_calendar_date_unchecked(Year::MIN, Month::January, 1) };

	/// The latest date which can be represented
	pub const MAX: Self =
		unsafe { Self::from_calendar_date_unchecked(Year::MAX, Month::December, 31) };

	// TODO validated from_calendar_date

	/// Creates a date without checking to make sure that it's valid.
	///
	/// # Example
	///
	/// ```
	/// use botic::Date;
	///
	/// let y2k = unsafe {
	///     Date::from_calendar_date_unchecked(Year::from(2000), Month::January, 1)
	/// };
	/// ```
	///
	/// # Safety
	///
	/// This function results in undefined behavior if the given date is not a real date
	pub const unsafe fn from_calendar_date_unchecked(year: Year, month: Month, day: u8) -> Self {
		Self { year, month, day }
	}

	// TODO docs

	pub const fn year(self) -> Year {
		self.year
	}

	pub const fn month(self) -> Month {
		self.month
	}

	pub const fn day(self) -> u8 {
		self.day
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
