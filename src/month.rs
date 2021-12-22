use derive_more::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use self::Month::*;

/// Months of the year
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

	/// Get the number of the month
	///
	/// # Example
	///
	/// ```
	/// use botic::Month;
	///
	/// assert_eq!(1, Month::January.number());
	/// ```
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

	/// Get the next month.
	///
	/// ```rust
	/// use botic::Month;
	///
	/// assert_eq!(Month::January.next(), Month::February);
	/// ```
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
}

impl From<Month> for u8 {
	fn from(month: Month) -> Self {
		month as u8
	}
}
