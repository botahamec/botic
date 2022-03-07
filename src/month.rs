use derive_more::Display;

use thiserror::Error;

use self::Month::*;

use core::str::FromStr;

/// Months of the year
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, PartialOrd, Ord)]
#[repr(u8)]
pub enum Month {
	January = 1,
	February = 2,
	March = 3,
	April = 4,
	May = 5,
	June = 6,
	July = 7,
	August = 8,
	September = 9,
	October = 10,
	November = 11,
	December = 12,
}

impl Month {
	/// Get the month based on the number.
	/// Returns `None` if the input is 0 or greater than 12.
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!(Some(Month::January), Month::from_u8(1));
	/// assert_eq!(None, Month::from_u8(0));
	/// assert_eq!(None, Month::from_u8(13));
	#[must_use]
	pub const fn from_u8(num: u8) -> Option<Self> {
		match num {
			1 => Some(January),
			2 => Some(February),
			3 => Some(March),
			4 => Some(April),
			5 => Some(May),
			6 => Some(June),
			7 => Some(July),
			8 => Some(August),
			9 => Some(September),
			10 => Some(October),
			11 => Some(November),
			12 => Some(December),
			_ => None,
		}
	}

	/// Get the month from the given string,
	/// which is assumed to be the month's abbreviation.
	/// Returns `None` if the string is not a valid abbrevation of a month
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!(Some(Month::January), Month::from_abbreviation("Jan"));
	/// assert_eq!(None, Month::from_abbreviation("Janu"));
	/// ```
	#[must_use]
	pub fn from_abbreviation(abbreviation: &str) -> Option<Self> {
		match abbreviation {
			"Jan" => Some(January),
			"Feb" => Some(February),
			"Mar" => Some(March),
			"Apr" => Some(April),
			"May" => Some(May),
			"Jun" => Some(June),
			"Jul" => Some(July),
			"Aug" => Some(August),
			"Sep" => Some(September),
			"Oct" => Some(October),
			"Nov" => Some(November),
			"Dec" => Some(December),
			_ => None,
		}
	}

	/// Get the month from the given string,
	/// which is assumed to be the month's name.
	/// Returns `None` if the string is not a valid month
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!(Some(Month::January), Month::from_name("January"));
	/// assert_eq!(None, Month::from_name("Janu"));
	/// ```
	#[must_use]
	pub fn from_name(name: &str) -> Option<Self> {
		match name {
			"January" => Some(January),
			"February" => Some(February),
			"March" => Some(March),
			"April" => Some(April),
			"May" => Some(May),
			"June" => Some(June),
			"July" => Some(July),
			"August" => Some(August),
			"September" => Some(September),
			"October" => Some(October),
			"November" => Some(November),
			"December" => Some(December),
			_ => None,
		}
	}

	/// Get the number of the month
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!(1, Month::January.number());
	/// ```
	#[must_use]
	pub const fn number(self) -> u8 {
		self as u8
	}

	/// Get the name of the month
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!("January", Month::January.name());
	/// ```
	#[must_use]
	pub const fn name(self) -> &'static str {
		match self {
			January => "January",
			February => "February",
			March => "March",
			April => "April",
			May => "May",
			June => "June",
			July => "July",
			August => "August",
			September => "September",
			October => "October",
			November => "November",
			December => "December",
		}
	}

	/// Get the abbreviated name of the month. This is always three letters
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!("Jan", Month::January.abbreviation());
	/// ```
	#[must_use]
	pub const fn abbreviation(self) -> &'static str {
		match self {
			January => "Jan",
			February => "Feb",
			March => "Mar",
			April => "Apr",
			May => "May",
			June => "Jun",
			July => "Jul",
			August => "Aug",
			September => "Sep",
			October => "Oct",
			November => "Nov",
			December => "Dec",
		}
	}

	// TODO docs

	#[must_use]
	pub const fn from_ordinal_common(ordinal: u16) -> Self {
		if ordinal < 31 {
			January
		} else if ordinal < 59 {
			February
		} else if ordinal < 90 {
			March
		} else if ordinal < 120 {
			April
		} else if ordinal < 151 {
			May
		} else if ordinal < 181 {
			June
		} else if ordinal < 212 {
			July
		} else if ordinal < 243 {
			August
		} else if ordinal < 273 {
			September
		} else if ordinal < 304 {
			October
		} else if ordinal < 334 {
			November
		} else {
			December
		}
	}

	#[must_use]
	pub const fn from_ordinal_leap(ordinal: u16) -> Self {
		if ordinal < 31 {
			January
		} else if ordinal < 60 {
			February
		} else if ordinal < 91 {
			March
		} else if ordinal < 121 {
			April
		} else if ordinal < 152 {
			May
		} else if ordinal < 182 {
			June
		} else if ordinal < 213 {
			July
		} else if ordinal < 244 {
			August
		} else if ordinal < 274 {
			September
		} else if ordinal < 305 {
			October
		} else if ordinal < 335 {
			November
		} else {
			December
		}
	}

	#[must_use]
	pub const fn from_ordinal(ordinal: u16, leap_year: bool) -> Self {
		if leap_year {
			Self::from_ordinal_leap(ordinal)
		} else {
			Self::from_ordinal_common(ordinal)
		}
	}

	/// Get the next month.
	///
	/// ```rust
	/// use botic::Month;
	///
	/// assert_eq!(Month::January.next(), Month::February);
	/// ```
	#[must_use]
	pub const fn next(self) -> Self {
		match self {
			January => February,
			February => March,
			March => April,
			April => May,
			May => June,
			June => July,
			July => August,
			August => September,
			September => October,
			October => November,
			November => December,
			December => January,
		}
	}

	/// Get the previous month.
	///
	/// ```rust
	/// use botic::Month;
	///
	/// assert_eq!(Month::January.previous(), Month::December);
	/// ```
	#[must_use]
	pub const fn previous(self) -> Self {
		match self {
			January => December,
			February => January,
			March => February,
			April => March,
			May => April,
			June => May,
			July => June,
			August => July,
			September => August,
			October => September,
			November => October,
			December => November,
		}
	}

	// TODO examples

	/// Returns the number of days up to the end of the month in a year.
	/// This doesn't account for leap day
	#[must_use]
	pub const fn last_day_ordinal_common(self) -> u16 {
		match self {
			January => 31,
			February => 59,
			March => 90,
			April => 120,
			May => 151,
			June => 181,
			July => 212,
			August => 243,
			September => 273,
			October => 304,
			November => 334,
			December => 365,
		}
	}

	/// Returns the number of days up to the end of the month in a leap year.
	#[must_use]
	pub const fn last_day_ordinal_leap(self) -> u16 {
		match self {
			January => 31,
			February => 60,
			March => 91,
			April => 121,
			May => 152,
			June => 182,
			July => 213,
			August => 244,
			September => 274,
			October => 305,
			November => 335,
			December => 366,
		}
	}

	/// Returns the number of days up to the end of the month.
	/// Whether or not it's a leap year must be indicated
	#[must_use]
	pub const fn last_day_ordinal(self, leap_year: bool) -> u16 {
		if leap_year {
			self.last_day_ordinal_leap()
		} else {
			self.last_day_ordinal_common()
		}
	}
}

impl From<Month> for u8 {
	fn from(month: Month) -> Self {
		month as u8
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
#[error("Failed to parse the month")]
// TODO Consider trying to figure out what month the user meant to use
pub struct ParseMonthError;

// TODO optimize to look like this: https://github.com/chronotope/chrono/blob/main/src/format/scan.rs#L102
// TODO make case-insensitive
impl FromStr for Month {
	type Err = ParseMonthError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Ok(num) = u8::from_str(s) {
			if let Some(month) = Month::from_u8(num) {
				Ok(month)
			} else {
				Err(ParseMonthError)
			}
		} else if let Some(month) = Month::from_abbreviation(s) {
			Ok(month)
		} else if let Some(month) = Month::from_name(s) {
			Ok(month)
		} else {
			Err(ParseMonthError)
		}
	}
}
