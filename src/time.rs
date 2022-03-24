use core::cmp::Ordering;
use core::fmt::Display;
use core::panic;

use thiserror::Error;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Time {
	hour: u8,
	minute: u8,
	second: u8,
	nanosecond: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Error)]
pub struct InvalidTimeError {
	hour: u8,
	minute: u8,
	second: u8,
	nanosecond: u32,
}

impl Display for InvalidTimeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Tried to construct invalid time {}:{}:{}.{}",
			self.hour, self.minute, self.second, self.nanosecond
		)
	}
}

impl InvalidTimeError {
	const unsafe fn new_unchecked(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Self {
		Self {
			hour,
			minute,
			second,
			nanosecond,
		}
	}
}

impl Time {
	/// A `Time` that is exactly midnight
	pub const MIDNIGHT: Self = unsafe { Self::from_hms_unchecked(0, 0, 0) };

	// TODO validated versions of the following:
	// TODO examples

	/// Create a `Time` from an hour, minute, and second
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, or second is
	/// greater than 60 results in undefined behavior
	#[must_use]
	pub const unsafe fn from_hms_unchecked(hour: u8, minute: u8, second: u8) -> Self {
		Self {
			hour,
			minute,
			second,
			nanosecond: 0,
		}
	}

	pub const fn from_hms(hour: u8, minute: u8, second: u8) -> Result<Self, InvalidTimeError> {
		Self::from_hms_nano(hour, minute, second, 0)
	}

	/// Create a `Time` from an hour, minute, second, and millisecond
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, second is
	/// greater than 60, or millisecond is greater than 999 results in undefined behavior
	#[must_use]
	pub const unsafe fn from_hms_milli_unchecked(
		hour: u8,
		minute: u8,
		second: u8,
		millisecond: u16,
	) -> Self {
		Self {
			hour,
			minute,
			second,
			nanosecond: millisecond as u32 * 1_000_000,
		}
	}

	pub const fn from_hms_milli(
		hour: u8,
		minute: u8,
		second: u8,
		millisecond: u16,
	) -> Result<Self, InvalidTimeError> {
		Self::from_hms_nano(hour, minute, second, millisecond as u32 * 1000)
	}

	/// Create a `Time` from an hour, minute, second, and microsecond
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, second is
	/// greater than 60, or microsecond is greater than 999,999 results in undefined behavior
	#[must_use]
	pub const unsafe fn from_hms_micro_unchecked(
		hour: u8,
		minute: u8,
		second: u8,
		microsecond: u32,
	) -> Self {
		Self {
			hour,
			minute,
			second,
			nanosecond: microsecond * 1_000,
		}
	}

	pub const fn from_hms_micro(
		hour: u8,
		minute: u8,
		second: u8,
		microsecond: u32,
	) -> Result<Self, InvalidTimeError> {
		Self::from_hms_nano(hour, minute, second, microsecond * 1_000_000)
	}

	/// Create a `Time` from an hour, minute, second, and nanosecond
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, second is
	/// greater than 60, or nanosecond is greater than 999,999,999 results in undefined behavior
	#[must_use]
	pub const unsafe fn from_hms_nano_unchecked(
		hour: u8,
		minute: u8,
		second: u8,
		nanosecond: u32,
	) -> Self {
		Self {
			hour,
			minute,
			second,
			nanosecond,
		}
	}

	pub const fn from_hms_nano(
		hour: u8,
		minute: u8,
		second: u8,
		nanosecond: u32,
	) -> Result<Self, InvalidTimeError> {
		if hour >= 24 {
			return unsafe {
				Err(InvalidTimeError::new_unchecked(
					hour, minute, second, nanosecond,
				))
			};
		}

		if minute >= 60 {
			return unsafe {
				Err(InvalidTimeError::new_unchecked(
					hour, minute, second, nanosecond,
				))
			};
		}

		if second > 60 {
			return unsafe {
				Err(InvalidTimeError::new_unchecked(
					hour, minute, second, nanosecond,
				))
			};
		}

		if nanosecond >= 1_000_000_000 {
			return unsafe {
				Err(InvalidTimeError::new_unchecked(
					hour, minute, second, nanosecond,
				))
			};
		}

		if second == 60 && (minute != 59 || hour != 23) {
			return unsafe {
				Err(InvalidTimeError::new_unchecked(
					hour, minute, second, nanosecond,
				))
			};
		}

		unsafe { Ok(Self::from_hms_unchecked(hour, minute, second)) }
	}

	/// Get the clock hour. The returned value will always be in the range `0..24`
	#[must_use]
	pub const fn hour(self) -> u8 {
		self.hour
	}

	/// Get the minute within the hour. The returned value will always be in the range `0..60`
	#[must_use]
	pub const fn minute(self) -> u8 {
		self.minute
	}

	// Get the second within the minute. The returned value will always be in the range `0..=60`
	#[must_use]
	pub const fn second(self) -> u8 {
		self.second
	}

	// Get the millisecond within the second.
	// The returned value will always be in the range `0..1_000`
	#[must_use]
	pub const fn millisecond(self) -> u16 {
		(self.nanosecond / 1_000_000) as u16
	}

	// Get the microsecond within the second.
	// The returned value will always be in the range `0..1_000_000`
	#[must_use]
	pub const fn microsecond(self) -> u32 {
		(self.nanosecond / 1_000) as u32
	}

	// Get the nanosecond within the second.
	// The returned value will always be in the range `0..1_000_000`
	#[must_use]
	pub const fn nanosecond(self) -> u32 {
		self.nanosecond
	}

	/// Adds the specified number of hours to the time.
	/// This returns a tuple of the addition result and a boolean indicating
	/// if overflow happened.
	#[must_use]
	pub const fn add_hours_overflowing(self, hours: isize) -> (Self, bool) {
		let total_hours = self.hour as isize + hours;
		let overflow = 0 > total_hours || total_hours >= 24;
		let total_hours = total_hours % 24 + (24 * total_hours.is_negative() as isize);

		let time = Self {
			hour: total_hours as u8,
			minute: self.minute,
			second: self.second,
			nanosecond: self.nanosecond,
		};

		(time, overflow)
	}

	/// Adds the specified number of minutes to the time.
	/// This returns a tuple of the addition result and a boolean indicating
	/// if overflow happened.
	#[must_use]
	pub const fn add_minutes_overflowing(self, minutes: isize) -> (Self, bool) {
		let total_minutes = (self.minute as isize + minutes) % 60;
		let total_minutes = total_minutes + (60 * total_minutes.is_negative() as isize);
		let added_hours = (self.hour as isize + minutes) / 60;
		let total_hours = self.hour as isize + added_hours;
		let overflow = 0 > total_hours || total_hours >= 24;
		let total_hours = total_hours % 24 + (24 * total_hours.is_negative() as isize);

		let time = Self {
			hour: total_hours as u8,
			minute: total_minutes as u8,
			second: self.second,
			nanosecond: self.nanosecond,
		};

		(time, overflow)
	}

	/// Adds the specified number of seconds to the time.
	/// This returns a tuple of the addition result and a boolean indicating
	/// if overflow happened.
	/// Leap seconds are not included in this calculation.
	#[must_use]
	pub const fn add_seconds_overflowing(self, seconds: isize) -> (Self, bool) {
		let total_seconds = (self.second as isize + seconds) % 60;
		let total_seconds = total_seconds + (60 * total_seconds.is_negative() as isize);
		let added_minutes = (self.second as isize + seconds) / 60;
		let total_minutes = (self.minute as isize + added_minutes) % 60;
		let total_minutes = total_minutes + (60 * total_minutes.is_negative() as isize);
		let added_hours = (self.hour as isize + added_minutes) / 60;
		let total_hours = self.hour as isize + added_hours;
		let overflow = 0 > total_hours || total_hours >= 24;
		let total_hours = total_hours % 24 + (24 * total_hours.is_negative() as isize);

		let time = Self {
			hour: total_hours as u8,
			minute: total_minutes as u8,
			second: total_seconds as u8,
			nanosecond: self.nanosecond,
		};

		(time, overflow)
	}

	/// Adds the specified number of nanoseconds to the time.
	/// This returns a tuple of the addition result and a boolean indicating
	/// if overflow happened.
	/// Leap seconds are not included in this calculation.
	#[must_use]
	pub const fn add_nanoseconds_overflowing(self, nanoseconds: isize) -> (Self, bool) {
		let total_nanos = (self.nanosecond as isize + nanoseconds) % 1_000_000_000;
		let total_nanos = total_nanos + (1_000_000_000 * total_nanos.is_negative() as isize);
		let added_seconds = (self.nanosecond as isize + nanoseconds) / 1_000_000_000;
		let total_seconds = (self.second as isize + added_seconds) % 60;
		let total_seconds = total_seconds + (60 * total_seconds.is_negative() as isize);
		let added_minutes = (self.second as isize + added_seconds) / 60;
		let total_minutes = (self.minute as isize + added_minutes) % 60;
		let total_minutes = total_minutes + (60 * total_minutes.is_negative() as isize);
		let added_hours = (self.minute as isize + added_minutes) / 60;
		let total_hours = self.hour as isize + added_hours;
		let overflow = 0 > total_hours || total_hours >= 24;
		let total_hours = total_hours % 24 + (24 * total_hours.is_negative() as isize);

		let time = Self {
			hour: total_hours as u8,
			minute: total_minutes as u8,
			second: total_seconds as u8,
			nanosecond: total_nanos as u32,
		};

		(time, overflow)
	}

	/// Adds the specified number of hours to the time.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_hours_checked(self, hours: isize) -> Option<Self> {
		let (time, overflow) = self.add_hours_overflowing(hours);

		if overflow {
			None
		} else {
			Some(time)
		}
	}

	/// Adds the specified number of minutes to the time.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_minutes_checked(self, minutes: isize) -> Option<Self> {
		let (time, overflow) = self.add_minutes_overflowing(minutes);

		if overflow {
			None
		} else {
			Some(time)
		}
	}

	/// Adds the specified number of seconds to the time.
	/// Leap seconds are not included in this calculation.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_seconds_checked(self, seconds: isize) -> Option<Self> {
		let (time, overflow) = self.add_seconds_overflowing(seconds);

		if overflow {
			None
		} else {
			Some(time)
		}
	}

	/// Adds the specified number of nanoseconds to the time.
	/// Leap seconds are not included in this calculation.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_nanoseconds_checked(self, nanoseconds: isize) -> Option<Self> {
		let (time, overflow) = self.add_nanoseconds_overflowing(nanoseconds);

		if overflow {
			None
		} else {
			Some(time)
		}
	}

	/// Adds the specified number of nanoseconds to the time.
	/// Leap seconds are not included in this calculation.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_hours_wrapping(self, hours: isize) -> Self {
		self.add_hours_overflowing(hours).0
	}

	/// Adds the specified number of nanoseconds to the time.
	/// Leap seconds are not included in this calculation.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_minutes_wrapping(self, minutes: isize) -> Self {
		self.add_minutes_overflowing(minutes).0
	}

	/// Adds the specified number of nanoseconds to the time.
	/// Leap seconds are not included in this calculation.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_seconds_wrapping(self, seconds: isize) -> Self {
		self.add_seconds_overflowing(seconds).0
	}

	/// Adds the specified number of nanoseconds to the time.
	/// Leap seconds are not included in this calculation.
	/// Returns `None` if overflow occurs.
	#[must_use]
	pub const fn add_nanoseconds_wrapping(self, nanoseconds: isize) -> Self {
		self.add_nanoseconds_overflowing(nanoseconds).0
	}

	/// Adds the specified number of hours to the time.
	///
	/// # Panics
	///
	/// Panics if the resulting time is 24 hours or more
	#[must_use]
	pub fn add_hours(self, hours: isize) -> Self {
		self.add_hours_checked(hours)
			.unwrap_or_else(|| panic!("Overflow when adding {hours} hours to {self}"))
	}

	/// Adds the specified number of minutes to the time
	///
	/// # Panics
	///
	/// Panics if the resulting time is 24 hours or more
	#[must_use]
	pub fn add_minutes(self, minutes: isize) -> Self {
		self.add_minutes_checked(minutes)
			.unwrap_or_else(|| panic!("Overflow when adding {minutes} minutes to {self}"))
	}

	/// Adds the specified number of seconds to the time.
	/// Leap seconds are not included in this calculation
	///
	/// # Panics
	///
	/// Panics if the resulting time is 24 hours or more
	#[must_use]
	pub fn add_seconds(self, seconds: isize) -> Self {
		self.add_seconds_checked(seconds)
			.unwrap_or_else(|| panic!("Overflow when adding {seconds} seconds to {self}"))
	}

	/// Adds the specified number of nanoseconds to the time.
	/// Leap seconds are not included in this calculation
	///
	/// # Panics
	///
	/// Panics if the resulting time is 24 hours or more
	#[must_use]
	pub fn add_nanoseconds(self, nanoseconds: isize) -> Self {
		self.add_nanoseconds_checked(nanoseconds)
			.unwrap_or_else(|| panic!("Overflow when adding {nanoseconds} nanoseconds to {self}"))
	}

	/// Gets the number of seconds since midnight
	#[must_use]
	pub const fn seconds_from_midnight(self) -> u32 {
		self.hour as u32 * 3_600_000_000
			+ self.minute as u32 * 60_000_000
			+ self.second as u32 * 1_000_000
	}

	/// Gets the number of nanoseconds since midnight
	#[must_use]
	pub fn nanoseconds_from_midnight(self) -> u64 {
		u64::from(self.hour) * 3_600_000_000_000
			+ u64::from(self.minute) * 60_000_000_000
			+ u64::from(self.second) * 1_000_000_000
			+ u64::from(self.nanosecond)
	}
}

impl PartialOrd for Time {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let hour_ordering = self.hour.cmp(&other.hour);
		let minute_ordering = self.minute.cmp(&other.minute);
		let second_ordering = self.second.cmp(&other.second);
		let nano_ordering = self.nanosecond.cmp(&other.nanosecond);

		if hour_ordering != Ordering::Equal {
			Some(hour_ordering)
		} else if minute_ordering != Ordering::Equal {
			Some(minute_ordering)
		} else if second_ordering != Ordering::Equal {
			Some(second_ordering)
		} else if nano_ordering != Ordering::Equal {
			Some(nano_ordering)
		} else {
			Some(Ordering::Equal)
		}
	}
}

impl Ord for Time {
	fn cmp(&self, other: &Self) -> Ordering {
		let hour_ordering = self.hour.cmp(&other.hour);
		let minute_ordering = self.minute.cmp(&other.minute);
		let second_ordering = self.second.cmp(&other.second);
		let nano_ordering = self.nanosecond.cmp(&other.nanosecond);

		if hour_ordering != Ordering::Equal {
			hour_ordering
		} else if minute_ordering != Ordering::Equal {
			minute_ordering
		} else if second_ordering != Ordering::Equal {
			second_ordering
		} else if nano_ordering != Ordering::Equal {
			nano_ordering
		} else {
			Ordering::Equal
		}
	}
}

impl Display for Time {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let seconds = f64::from(self.second) + (f64::from(self.nanosecond) / 1_000_000_000.0);
		if self.nanosecond() == 0 {
			write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
		} else if self.second < 10 {
			write!(f, "{:02}:{:02}:0{}", self.hour, self.minute, seconds)
		} else {
			write!(f, "{:02}:{:02}:{}", self.hour, self.minute, seconds)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display_without_nanos() {
		let time = unsafe { Time::from_hms_nano_unchecked(0, 0, 1, 0) };
		let time_str = format!("{time}");
		assert_eq!(time_str, "00:00:01");
	}

	#[test]
	fn display_with_nanos_lt_10() {
		let time = unsafe { Time::from_hms_nano_unchecked(0, 0, 1, 1_000_000) };
		let time_str = format!("{time}");
		assert_eq!(time_str, "00:00:01.001");
	}

	#[test]
	fn display_with_nanos_gt_10() {
		let time = unsafe { Time::from_hms_nano_unchecked(0, 0, 10, 1_000_000) };
		let time_str = format!("{time}");
		assert_eq!(time_str, "00:00:10.001");
	}
}
