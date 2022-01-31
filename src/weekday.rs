use core::str::FromStr;

use derive_more::Display;

use thiserror::Error;

use self::Weekday::*;

/// Day of the week
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display)]
#[repr(u8)]
pub enum Weekday {
	Monday = 0,
	Tuesday = 1,
	Wednesday = 2,
	Thursday = 3,
	Friday = 4,
	Saturday = 5,
	Sunday = 6,
}

impl Weekday {
	/// Get the weekday from its name. Returns `None` if an invalid name was given.
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(Weekday::Monday, Weekday::from_name("Monday").unwrap());
	/// assert_eq!(None, Weekday::from_name("monday"));
	/// ```
	pub fn from_name(name: &str) -> Option<Self> {
		match name {
			"Monday" => Some(Monday),
			"Tuesday" => Some(Tuesday),
			"Wednesday" => Some(Wednesday),
			"Thursday" => Some(Thursday),
			"Friday" => Some(Friday),
			"Saturday" => Some(Saturday),
			"Sunday" => Some(Sunday),
			_ => None,
		}
	}

	/// Get the next weekday
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(Weekday::Tuesday, Weekday::Monday.next());
	/// ```
	pub const fn next(self) -> Self {
		match self {
			Monday => Tuesday,
			Tuesday => Wednesday,
			Wednesday => Thursday,
			Thursday => Friday,
			Friday => Saturday,
			Saturday => Sunday,
			Sunday => Monday,
		}
	}

	/// Get the previous weekday
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(Weekday::Sunday, Weekday::Monday.previous());
	/// ```
	pub const fn previous(self) -> Self {
		match self {
			Monday => Sunday,
			Tuesday => Monday,
			Wednesday => Tuesday,
			Thursday => Wednesday,
			Friday => Thursday,
			Saturday => Friday,
			Sunday => Saturday,
		}
	}

	/// Get the zero-indexed number of days from Monday.
	/// In other words, the number representing the day of the week,
	/// starting with Monday = 0
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(0, Weekday::Monday.number_days_from_monday());
	/// assert_eq!(6, Weekday::Sunday.number_days_from_monday());
	/// ```
	pub const fn number_days_from_monday(self) -> u8 {
		self as u8
	}

	/// Get the one-indexed number of days from Monday.
	/// In other words, the number representing the day of the week,
	/// starting with Monday = 1
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(1, Weekday::Monday.number_from_monday());
	/// assert_eq!(7, Weekday::Sunday.number_from_monday());
	/// ```
	pub const fn number_from_monday(self) -> u8 {
		self.number_days_from_monday() + 1
	}

	/// Get the zero-indexed number of days from Sunday.
	/// In other words, the number representing the day of the week,
	/// starting with Sunday = 0
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(0, Weekday::Sunday.number_days_from_sunday());
	/// assert_eq!(1, Weekday::Monday.number_days_from_sunday());
	/// ```
	// TODO benchmark this
	pub const fn number_days_from_sunday(self) -> u8 {
		match self {
			Sunday => 0,
			_ => (self as u8) + 1,
		}
	}

	/// Get the one-indexed number of days from Sunday.
	/// In other words, the number representing the day of the week,
	/// starting with Sunday = 1
	///
	/// # Example
	///
	/// ```
	/// use botic::Weekday;
	///
	/// assert_eq!(1, Weekday::Sunday.number_from_sunday());
	/// assert_eq!(2, Weekday::Monday.number_from_sunday());
	/// ```
	pub const fn number_from_sunday(self) -> u8 {
		self.number_days_from_sunday() + 1
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
#[error("Failed to parse the month")]
// TODO Consider trying to figure out what month the user meant to use
pub struct ParseWeekdayError;

// TODO make case-insensitive
// TODO support short names
impl FromStr for Weekday {
	type Err = ParseWeekdayError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match Self::from_name(s) {
			Some(weekday) => Ok(weekday),
			None => Err(ParseWeekdayError),
		}
	}
}
