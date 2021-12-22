use derive_more::{Display, From};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use self::Month::*;

/// Months of the year
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Display, From)]
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
}

impl From<Month> for u8 {
	fn from(month: Month) -> Self {
		month as u8
	}
}
