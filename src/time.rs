use core::cmp::Ordering;
use core::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Time {
	hour: u8,
	minute: u8,
	second: u8,
	nanosecond: u32,
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
	pub const unsafe fn from_hms_unchecked(hour: u8, minute: u8, second: u8) -> Self {
		Self {
			hour,
			minute,
			second,
			nanosecond: 0,
		}
	}

	/// Create a `Time` from an hour, minute, second, and millisecond
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, second is
	/// greater than 60, or millisecond is greater than 999 results in undefined behavior
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

	/// Create a `Time` from an hour, minute, second, and microsecond
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, second is
	/// greater than 60, or microsecond is greater than 999,999 results in undefined behavior
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

	/// Create a `Time` from an hour, minute, second, and nanosecond
	///
	/// # Safety
	///
	/// Creating a time where the hour is greater than 23, minute is greater than 59, second is
	/// greater than 60, or nanosecond is greater than 999,999,999 results in undefined behavior
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

	/// Get the clock hour. The returned value will always be in the range `0..24`
	pub const fn hour(self) -> u8 {
		self.hour
	}

	/// Get the minute within the hour. The returned value will always be in the range `0..60`
	pub const fn minute(self) -> u8 {
		self.minute
	}

	// Get the second within the minute. The returned value will always be in the range `0..=60`
	pub const fn second(self) -> u8 {
		self.second
	}

	// Get the millisecond within the second.
	// The returned value will always be in the range `0..1_000`
	pub const fn millisecond(self) -> u16 {
		(self.nanosecond / 1_000_000) as u16
	}

	// Get the microsecond within the second.
	// The returned value will always be in the range `0..1_000_000`
	pub const fn microsecond(self) -> u32 {
		(self.nanosecond / 1_000) as u32
	}

	// Get the nanosecond within the second.
	// The returned value will always be in the range `0..1_000_000`
	pub const fn nanosecond(self) -> u32 {
		self.nanosecond
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

// TODO addition
