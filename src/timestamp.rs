use core::ops::Sub;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::io;
use std::time;
use std::time::Duration;

use crate::constant::{TIMEZONE_UTC, U128_MILLIS_IN_SECOND, U128_NANOS_IN_MILLI, U128_NANOS_IN_SECOND};
use crate::format::Format;
use crate::parts::TimestampParts;

/// Encapsulates a timestamp, as number of nanoseconds since UNIX epoch (1970-01-01 00:00 UTC).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
	seconds: u64,
	nanoseconds: u32,
}

impl<'i> Timestamp {
	/// Creates a [`Timestamp`] from a given epoch timestamp in seconds + nanoseconds.
	pub fn new(secs: u64, nanos: u32) -> Self {
		Self { seconds: secs, nanoseconds: nanos }
	}

	/// Creates a [`Timestamp`] from a given epoch timestamp in seconds.
	pub fn from_secs(secs: u64) -> Self {
		Self { seconds: secs, nanoseconds: 0 }
	}

	/// Creates a [`Timestamp`] from a given epoch timestamp in milliseconds.
	pub fn from_millis(msecs: u128) -> Self {
		Self {
			seconds: (msecs / U128_MILLIS_IN_SECOND) as _,
			nanoseconds: ((msecs % U128_MILLIS_IN_SECOND) * U128_NANOS_IN_MILLI) as _,
		}
	}

	/// Creates a [`Timestamp`] from a given epoch timestamp in nanoseconds.
	pub fn from_nanos(nanos: u128) -> Self {
		Self {
			seconds: (nanos / U128_NANOS_IN_SECOND) as _,
			nanoseconds: (nanos % U128_NANOS_IN_SECOND) as _,
		}
	}

	/// Creates a [`Timestamp`] from a given [`TimestampParts`].
	pub fn from_parts(parts: &'i TimestampParts) -> Result<Self, &'i str> {
		parts.to_timestamp()
	}

	/// Creates a [`Timestamp`] from a given UTC date + time.
	pub fn from_utc_date(year: u16, month: u8, day: u8, hour: u8, minutes: u8, secs: u8, millis: u16, nanos: u32) -> Result<Self, &'i str> {
		TimestampParts {
			nanoseconds: nanos,
			milliseconds: millis,
			seconds: secs,
			minutes: minutes,
			hour: hour,
			month_day: day,
			month: month,
			year: year,
			// week_day and year_day are unused when converting back to [`Timestamp`]
			week_day: 1,
			year_day: 1,
			gmt_offset_negative: false,
			gmt_offset_hours: 0,
			gmt_offset_minutes: 0,
			timezone: TIMEZONE_UTC,
		}
		.to_timestamp()
	}

	/// Returns an UNIX epoch [`Timestamp`] (1970-01-01 00:00 UTC)
	pub fn epoch() -> Self {
		Self::new(0, 0)
	}

	/// Creates a [`Timestamp`] from a given [`std::time::SystemTime`].
	pub fn from_system_time(time: std::time::SystemTime) -> Self {
		match time.duration_since(time::UNIX_EPOCH) {
			Ok(d) => Self::from_nanos(d.as_nanos()),
			Err(e) => panic!("failed to parse time duration: {e}"),
		}
	}

	/// Creates a [`Timestamp`] for the current time.
	pub fn now() -> Self {
		Self::from_system_time(time::SystemTime::now())
	}

	/// Updates a [`Timestamp`] in place, with the contents of another [`Timestamp`].
	pub fn copy_from(&mut self, other: &Self) {
		self.seconds = other.seconds;
		self.nanoseconds = other.nanoseconds;
	}

	/// Returs the number of seconds + nanoseconds since UNIX epoch.
	pub fn epoch_offset(&self) -> (u64, u32) {
		(self.seconds, self.nanoseconds)
	}

	/// Returns number of seconds since UNIX epoch.
	pub fn as_secs(&self) -> u64 {
		self.seconds
	}

	/// Returns number of milliseconds since UNIX epoch.
	pub fn as_millis(&self) -> u128 {
		(self.seconds as u128) * U128_MILLIS_IN_SECOND + (self.nanoseconds as u128 / U128_NANOS_IN_MILLI)
	}

	/// Returns number of nanoseconds since UNIX epoch.
	pub fn as_nanos(&self) -> u128 {
		(self.seconds as u128) * U128_NANOS_IN_SECOND + self.nanoseconds as u128
	}

	/// Returns a [`TimestampParts`] for the timezone, as UTC.
	pub fn as_utc_parts(&self) -> TimestampParts<'i> {
		TimestampParts::utc(&self)
	}

	/// Returns a [`TimestampParts`] for the timezone, in the local timezone.
	pub fn as_local_parts(&self) -> TimestampParts<'i> {
		TimestampParts::local(&self)
	}

	/// Returns a string representation for the timestamp, in a given [`Format`].
	pub fn as_string(&self, format: &Format) -> String {
		format.as_string(self)
	}

	/// Returns the string representation length for the given [`Format`].
	pub fn string_len<T: io::Write>(&self, format: &Format) -> usize {
		format.string_len(self)
	}

	/// Returns a integer representation for the timestamp, if the given [`Format`] supports it.
	pub fn as_integer(&self, format: &Format) -> Option<u128> {
		format.as_integer(self)
	}

	/// Serializes a string representation into a [`io::Write`],in the given [`Format`].
	pub fn write<T: io::Write>(&self, out: &mut T, format: &Format) -> io::Result<()> {
		format.write(out, self)
	}

	/// Adds a [`time::Duration`] to the timestamp.
	pub fn add_duration(&mut self, d: &Duration) -> &Self {
		let nanos = d.as_nanos() + self.nanoseconds as u128;

		self.seconds = self.seconds + (nanos / U128_NANOS_IN_SECOND) as u64;
		self.nanoseconds = (nanos % U128_NANOS_IN_SECOND) as u32;

		self
	}

	/// Compares two timestamps for ordering.
	fn cmp(&self, other: &Self) -> Ordering {
		if self.seconds == other.seconds {
			if self.nanoseconds < other.nanoseconds {
				return Ordering::Less;
			}
			if self.nanoseconds > other.nanoseconds {
				return Ordering::Greater;
			}
			return Ordering::Equal;
		}

		if self.seconds < other.seconds {
			return Ordering::Less;
		}
		Ordering::Greater
	}

	/// Returns the difference between two timestamps as [`time::Duration`].
	pub fn diff_as_duration(&self, other: &Self) -> Duration {
		let self_nanos = self.as_nanos();
		let other_nanos = other.as_nanos();

		if other_nanos >= self_nanos {
			Duration::ZERO
		} else {
			Duration::from_nanos((self_nanos - other_nanos) as u64)
		}
	}
}

impl fmt::Display for Timestamp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_string(&Format::LocalDateTime))
	}
}

impl PartialOrd for Timestamp {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Timestamp {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cmp(other)
	}
}

impl Sub for Timestamp {
	type Output = Duration;

	fn sub(self, other: Self) -> Self::Output {
		self.diff_as_duration(&other)
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers;

	#[test]
	fn from_date() {
		assert_eq!(
			Timestamp::from_utc_date(2026, 03, 07, 04, 48, 17, 446, 37892),
			Ok(Timestamp {
				seconds: 1772858897,
				nanoseconds: 446037892,
			}),
			"cast from UTC date"
		);
	}

	#[test]
	fn comparison() {
		assert_eq!(Timestamp::new(1234, 100), Timestamp::new(1234, 100), "equal");
		assert!(Timestamp::new(1234, 100) > Timestamp::new(1234, 50), "same secs, more millis");
		assert!(Timestamp::new(1234, 100) < Timestamp::new(1234, 200), "same secs, less millis");
		assert!(Timestamp::new(5678, 100) > Timestamp::new(1234, 100), "more secs");
		assert!(Timestamp::new(1234, 100) < Timestamp::new(5768, 100), "less secs");
	}

	#[test]
	fn operators() {
		assert_eq!(Timestamp::from_nanos(1234) - Timestamp::from_nanos(1234), Duration::ZERO, "zero result");
		assert_eq!(Timestamp::from_nanos(1234) - Timestamp::from_nanos(5768), Duration::ZERO, "underflow");
		assert_eq!(Timestamp::from_nanos(5678) - Timestamp::from_nanos(1234), Duration::from_nanos(4444), "OK");
	}

	#[test]
	fn copy_from() {
		let mut ts_1 = Timestamp::new(1234, 888);
		let ts_2 = Timestamp::new(5678, 999);
		assert_ne!(ts_1, ts_2);

		ts_1.copy_from(&ts_2);
		assert_eq!(ts_1, ts_2);
		assert_eq!(ts_1, Timestamp::new(5678, 999));
	}

	#[test]
	fn casting() {
		let ts = Timestamp::new(1772457319, 38123456);
		assert_eq!(ts.as_secs(), 1772457319, "cast to seconds");
		assert_eq!(ts.as_millis(), 1772457319038, "cast to millis");
		assert_eq!(ts.as_nanos(), 1772457319038123456, "cast to nanos");
	}

	#[test]
	fn to_string() {
		// the Display trait implementation serializes Timestamps into local time strings.
		test_helpers::mocks::with_timezone("Asia/Kuching", || {
			assert_eq!(Timestamp::new(1772457020, 789).to_string(), "2026-03-02 21:10:20 +0800",);
			assert_eq!(Timestamp::from_secs(1772457213).to_string(), "2026-03-02 21:13:33 +0800",);
			assert_eq!(Timestamp::from_millis(1772457213123).to_string(), "2026-03-02 21:13:33 +0800",);
			assert_eq!(
				Timestamp::from_utc_date(2026, 03, 06, 14, 43, 39, 128, 564).expect("invalid parts").to_string(),
				"2026-03-06 22:43:39 +0800",
			);
		});
	}

	#[test]
	fn utc_parts_conversion() {
		assert_eq!(
			Timestamp::from_millis(1772457319335).as_utc_parts(),
			TimestampParts {
				nanoseconds: 0,
				milliseconds: 335,
				seconds: 19,
				minutes: 15,
				hour: 13,
				month_day: 2,
				month: 3,
				year: 2026,
				week_day: 2,
				year_day: 61,
				gmt_offset_negative: false,
				gmt_offset_hours: 0,
				gmt_offset_minutes: 0,
				timezone: TIMEZONE_UTC,
			},
			"UTC parts from milliseconds timestamp"
		);

		assert_eq!(
			Timestamp::from_nanos(1772457319335012345).as_utc_parts(),
			TimestampParts {
				nanoseconds: 12345,
				milliseconds: 335,
				seconds: 19,
				minutes: 15,
				hour: 13,
				month_day: 2,
				month: 3,
				year: 2026,
				week_day: 2,
				year_day: 61,
				gmt_offset_negative: false,
				gmt_offset_hours: 0,
				gmt_offset_minutes: 0,
				timezone: TIMEZONE_UTC,
			},
			"UTC parts from nanoseconds timestamp"
		);
	}

	#[test]
	fn add_duration() {
		let mut a = Timestamp::new(1234, 5678);
		a.add_duration(&Duration::from_millis(2234));
		assert_eq!(a, Timestamp::new(1236, 234005678));
	}
}
