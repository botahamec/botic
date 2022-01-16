use std::ops::Add;

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
	pub const fn millisecond(self) -> u8 {
		(self.nanosecond / 1_000_000) as u8
	}

	// Get the microsecond within the second.
	// The returned value will always be in the range `0..1_000_000`
	pub const fn microsecond(self) -> u8 {
		(self.nanosecond / 1_000) as u8
	}

	// Get the nanosecond within the second.
	// The returned value will always be in the range `0..1_000_000`
	pub const fn nanosecond(self) -> u32 {
		self.nanosecond
	}
}

// TODO addition