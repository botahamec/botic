use core::ops::{Add, AddAssign, Sub, SubAssign};

use derive_more::{Display, FromStr};

/// A year value type, stored as an i16
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, FromStr, Display)]
pub struct Year(i16);

impl Year {
	/// The latest year that can be represented
	pub const MAX: Self = Self(i16::MAX);

	/// The earliest year that can be represented
	pub const MIN: Self = Self(i16::MIN);

	/// An equivalent of `Year::from(i16)`, which can be run at compile-time
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// const YEAR: Year = Year::from_i16(2021);
	/// assert_eq!(2021, YEAR.as_i16());
	/// ```
	pub const fn from_i16(i: i16) -> Self {
		Self(i)
	}

	/// An equivalent of `Year::into` which can be run at compile-time
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// const YEAR: Year = Year::from_i16(2021);
	/// const YEAR_INT: i16 = YEAR.as_i16();
	/// assert_eq!(2021, YEAR_INT);
	/// ```
	pub const fn as_i16(self) -> i16 {
		self.0
	}

	/// Checked year addition.
	/// Computes `self + rhs`, returning `None` if overflow occurred.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!(Some(Year::from(2022)), Year::from_i16(2021).checked_add(1));
	/// assert_eq!(None, Year::MAX.checked_add(1));
	/// ```
	pub const fn checked_add(self, rhs: i16) -> Option<Year> {
		match self.0.checked_add(rhs) {
			Some(year) => Some(Self(year)),
			None => None,
		}
	}

	/// Calculates `self + rhs`
	///
	/// Returns a tuple of the addition along with a boolean indicating
	/// whether an arithmetic overflow would occur. If an overflow would have
	/// occurred then the wrapped value is returned.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!((Year::from(2022), false), Year::from(2021).overflowing_add(1));
	/// assert_eq!((Year::MIN, true), Year::MAX.overflowing_add(1));
	/// ```
	pub const fn overflowing_add(self, rhs: i16) -> (Year, bool) {
		let int_result = self.0.overflowing_add(rhs);
		(Year(int_result.0), int_result.1)
	}

	/// Saturating year addition.
	/// Computes `self + rhs`, saturating at the bounds instead of overflowing.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!(Year::from(2022), Year::from(2021).saturating_add(1));
	/// assert_eq!(Year::MAX, Year::MAX.saturating_add(1));
	/// ```
	pub const fn saturating_add(self, rhs: i16) -> Year {
		Year(self.0.saturating_add(rhs))
	}

	/// Wrapping (modular) addition.
	/// Computes `self + rhs`, wrapping around at the boundary of the type.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!(Year::from(2022), Year::from(2021).wrapping_add(1));
	/// assert_eq!(Year::MIN, Year::MAX.wrapping_add(1));
	pub const fn wrapping_add(self, rhs: i16) -> Year {
		Year(self.0.wrapping_add(rhs))
	}

	/// Checked year subtraction.
	/// Computes `self - rhs`, returning `None` if overflow occurred.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!(Some(Year::from(2020)), Year::from_i16(2021).checked_sub(1));
	/// assert_eq!(None, Year::MIN.checked_sub(1));
	/// ```
	pub const fn checked_sub(self, rhs: i16) -> Option<Year> {
		match self.0.checked_sub(rhs) {
			Some(year) => Some(Self(year)),
			None => None,
		}
	}

	/// Calculates `self - rhs`
	///
	/// Returns a tuple of the subtraction along with a boolean indicating
	/// whether an arithmetic overflow would occur. If an overflow would have
	/// occurred then the wrapped value is returned.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!((Year::from(2020), false), Year::from(2021).overflowing_sub(1));
	/// assert_eq!((Year::MAX, true), Year::MIN.overflowing_sub(1));
	/// ```
	pub const fn overflowing_sub(self, rhs: i16) -> (Year, bool) {
		let int_result = self.0.overflowing_sub(rhs);
		(Year(int_result.0), int_result.1)
	}

	/// Saturating year subtraction.
	/// Computes `self - rhs`, saturating at the bounds instead of overflowing.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!(Year::from(2020), Year::from(2021).saturating_sub(1));
	/// assert_eq!(Year::MIN, Year::MIN.saturating_sub(1));
	/// ```
	pub const fn saturating_sub(self, rhs: i16) -> Year {
		Year(self.0.saturating_sub(rhs))
	}

	/// Wrapping (modular) subtraction.
	/// Computes `self - rhs`, wrapping around at the boundary of the type.
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert_eq!(Year::from(2020), Year::from(2021).wrapping_sub(1));
	/// assert_eq!(Year::MAX, Year::MIN.wrapping_sub(1));
	pub const fn wrapping_sub(self, rhs: i16) -> Year {
		Year(self.0.wrapping_sub(rhs))
	}

	/// Checks if the year is a leap year
	///
	/// # Example
	///
	/// ```
	/// use botic::Year;
	///
	/// assert!(!Year::from(2022).is_leap_year());
	/// assert!(Year::from(2020).is_leap_year());
	/// assert!(Year::from(2000).is_leap_year());
	/// assert!(!Year::from(2100).is_leap_year());
	/// ```
	pub const fn is_leap_year(self) -> bool {
		(self.0 % 4 == 0) && ((self.0 % 100 != 0) || (self.0 % 400 == 0))
	}
}

impl From<i16> for Year {
	fn from(i: i16) -> Self {
		Self(i)
	}
}

impl From<Year> for i16 {
	fn from(year: Year) -> Self {
		year.0
	}
}

impl<I: Into<i16>> Add<I> for Year {
	type Output = Self;

	fn add(self, rhs: I) -> Self::Output {
		Self(self.0 + rhs.into())
	}
}

impl<I: Into<i16>> Sub<I> for Year {
	type Output = Self;

	fn sub(self, rhs: I) -> Self::Output {
		Self(self.0 - rhs.into())
	}
}

impl AddAssign<i16> for Year {
	fn add_assign(&mut self, rhs: i16) {
		self.0 = self.0 + rhs
	}
}

impl SubAssign<i16> for Year {
	fn sub_assign(&mut self, rhs: i16) {
		self.0 = self.0 - rhs
	}
}
