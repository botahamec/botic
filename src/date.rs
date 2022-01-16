use crate::{Month, Year};

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

// TODO addition
