use core::ops::Sub;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::io;
use std::time;
use std::time::Duration;

use crate::constant::{TIMESTAMP_MAX_MILLISECONDS, TIMESTAMP_MAX_NANOSECONDS, TIMEZONE_UTC, U32_NANOS_IN_SECOND, U64_SECONDS_IN_DAY, U128_MILLIS_IN_SECOND, U128_NANOS_IN_MILLI, U128_NANOS_IN_SECOND};
use crate::error::{CastError, TimestampOpError};
use crate::format::Format;
use crate::parts::TimestampParts;

/// A UTC datetime, which can be used to initialize [`Timestamp`]s.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UTCDateTime {
	/// Year (`1970`-`584554051223`).
	pub year: u64,
	/// Month of the year (`1`-`12`).
	pub month: u8,
	/// Day of the month (`1`-`31`).
	pub day: u8,
	/// Hours since midnight (`0`-`23`).
	pub hour: u8,
	/// Minutes after the hour (`0`-`59`).
	pub minutes: u8,
	/// Seconds after the minute (`0`-`59`).
	pub seconds: u8,
	/// Milliseconds after the second (`0`-`999`).
	pub milliseconds: u16,
	/// Nanoseconds after the millisecond (`0`-`999999`).
	pub nanoseconds: u32,
}

impl UTCDateTime {
	/// Minimum supported [`UTCDateTime`]: `1970-01-01 00:00:00.000000000 UTC`
	pub const MIN: Self = Self {
		year: 1970,
		month: 1,
		day: 1,
		hour: 0,
		minutes: 0,
		seconds: 0,
		milliseconds: 0,
		nanoseconds: 0,
	};

	/// Maximum supported [`UTCDateTime`]: `584554051223-11-09 07:00:15.999999999 UTC`
	pub const MAX: Self = Self {
		year: 584554051223,
		month: 11,
		day: 9,
		hour: 7,
		minutes: 0,
		seconds: 15,
		milliseconds: 999,
		nanoseconds: 999999,
	};

	/// UNIX epoch [`UTCDateTime`] (`1970-01-01 00:00:00 UTC`)
	pub const EPOCH: Self = Self::MIN;

	/// Returns the UNIX epoch [`UTCDateTime`] (`1970-01-01 00:00:00 UTC`).
	#[deprecated(note = "use UTCDateTime::EPOCH instead")]
	#[inline]
	pub fn epoch() -> Self {
		Self::EPOCH
	}
}

/// Encapsulates a timestamp, as number of nanoseconds since UNIX epoch (`1970-01-01 00:00:00 UTC`).
#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
pub struct Timestamp {
	seconds: u64,
	nanoseconds: u32,
}

/// Initializes a [`Timestamp`] from a [`std::time::SystemTime`] reference.
impl TryFrom<&std::time::SystemTime> for Timestamp {
	type Error = CastError;

	fn try_from(time: &time::SystemTime) -> Result<Self, Self::Error> {
		match time.duration_since(time::UNIX_EPOCH) {
			Ok(d) => Self::from_nanos(d.as_nanos()),
			Err(e) => Err(CastError::from(e)),
		}
	}
}

/// Initializes a [`Timestamp`] from a [`std::time::SystemTime`].
impl TryFrom<std::time::SystemTime> for Timestamp {
	type Error = CastError;

	fn try_from(time: time::SystemTime) -> Result<Self, Self::Error> {
		Self::try_from(&time)
	}
}

/// Initializes a [`Timestamp`] from a given [`UTCDateTime`], which gets consumed.
impl TryFrom<UTCDateTime> for Timestamp {
	type Error = CastError;

	fn try_from(udt: UTCDateTime) -> Result<Self, Self::Error> {
		Timestamp::from_utc_datetime(&udt)
	}
}

/// Initializes a [`Timestamp`] from a given [`UTCDateTime`] reference.
impl<'i> TryFrom<&'i UTCDateTime> for Timestamp {
	type Error = CastError;

	fn try_from(udt: &'i UTCDateTime) -> Result<Self, Self::Error> {
		Timestamp::from_utc_datetime(udt)
	}
}

/// Initializes a [`Timestamp`] from a (year, month, day) tuple.
impl TryFrom<(u64, u8, u8)> for Timestamp {
	type Error = CastError;

	fn try_from((year, month, day): (u64, u8, u8)) -> Result<Self, Self::Error> {
		Self::try_from(UTCDateTime {
			year,
			month,
			day,
			..UTCDateTime::EPOCH
		})
	}
}

/// Initializes a [`Timestamp`] from a (year, month, day, hour, minutes, seconds) tuple.
impl TryFrom<(u64, u8, u8, u8, u8, u8)> for Timestamp {
	type Error = CastError;

	fn try_from((year, month, day, hour, minutes, seconds): (u64, u8, u8, u8, u8, u8)) -> Result<Self, Self::Error> {
		Self::try_from(UTCDateTime {
			year,
			month,
			day,
			hour,
			minutes,
			seconds,
			..UTCDateTime::EPOCH
		})
	}
}

/// Initializes a [`Timestamp`] from a (year, month, day, hour, minutes, seconds, milliseconds, nanoseconds) tuple.
impl TryFrom<(u64, u8, u8, u8, u8, u8, u16, u32)> for Timestamp {
	type Error = CastError;

	fn try_from((year, month, day, hour, minutes, seconds, milliseconds, nanoseconds): (u64, u8, u8, u8, u8, u8, u16, u32)) -> Result<Self, Self::Error> {
		Self::try_from(UTCDateTime {
			year,
			month,
			day,
			hour,
			minutes,
			seconds,
			milliseconds,
			nanoseconds,
		})
	}
}

impl<'i> Timestamp {
	/// Minimum supported [`Timestamp`]: `1970-01-01 00:00:00.000000000 UTC`.
	pub const MIN: Timestamp = Self { seconds: 0, nanoseconds: 0 };

	/// Maximum supported [`Timestamp`]: `584554051223-11-09 07:00:15.999999999 UTC`.
	pub const MAX: Timestamp = Self {
		seconds: u64::MAX,
		nanoseconds: U32_NANOS_IN_SECOND - 1,
	};

	/// UNIX epoch [`Timestamp`] (`1970-01-01 00:00:00 UTC`)
	pub const EPOCH: Self = Self::MIN;

	/// Creates a [`Timestamp`] from a given epoch timestamp in seconds + nanoseconds.
	/// Overflowing nanoseconds (>= `1_000_000_000`) are carried over as seconds, if possible.
	pub fn new(secs: u64, nanos: u32) -> Result<Self, CastError> {
		let seconds = match secs.checked_add((nanos / U32_NANOS_IN_SECOND) as u64) {
			Some(s) => s,
			None => return Err(CastError::Overflow("total seconds overflow")),
		};
		let nanoseconds = nanos % U32_NANOS_IN_SECOND;

		Ok(Self { seconds, nanoseconds })
	}

	/// Creates a [`Timestamp`] from a given epoch timestamp in seconds.
	pub fn from_secs(secs: u64) -> Self {
		Self { seconds: secs, nanoseconds: 0 }
	}

	/// Creates a [`Timestamp`] from a given epoch timestamp in milliseconds.
	///
	/// # Errors
	///
	/// Returns [`CastError::Overflow`] if the provided milliseconds don't fit a [`Timestamp`].
	pub fn from_millis(millis: u128) -> Result<Self, CastError> {
		if millis > TIMESTAMP_MAX_MILLISECONDS {
			return Err(CastError::Overflow("milliseconds from epoch is larger than what Timestamp can represent"));
		}

		Ok(Self {
			seconds: (millis / U128_MILLIS_IN_SECOND) as _,
			nanoseconds: ((millis % U128_MILLIS_IN_SECOND) * U128_NANOS_IN_MILLI) as _,
		})
	}

	/// Creates a [`Timestamp`] from a given epoch timestamp in nanoseconds.
	///
	/// # Errors
	///
	/// Returns [`CastError::Overflow`] if the provided nanoseconds don't fit a [`Timestamp`].
	pub fn from_nanos(nanos: u128) -> Result<Self, CastError> {
		if nanos > TIMESTAMP_MAX_NANOSECONDS {
			return Err(CastError::Overflow("nanoseconds from epoch is larger than what Timestamp can represent"));
		}

		Ok(Self {
			seconds: (nanos / U128_NANOS_IN_SECOND) as _,
			nanoseconds: (nanos % U128_NANOS_IN_SECOND) as _,
		})
	}

	/// Creates a [`Timestamp`] from a given [`TimestampParts`].
	///
	/// # Errors
	///
	/// Returns [`CastError`] upon invalid [`TimestampParts`] fields.
	pub fn from_parts(parts: &'i TimestampParts) -> Result<Self, CastError> {
		parts.to_timestamp()
	}

	/// Creates a [`Timestamp`] from a given [`UTCDateTime`].
	fn from_utc_datetime(udt: &UTCDateTime) -> Result<Self, CastError> {
		TimestampParts {
			nanoseconds: udt.nanoseconds,
			milliseconds: udt.milliseconds,
			seconds: udt.seconds,
			minutes: udt.minutes,
			hour: udt.hour,
			month_day: udt.day,
			month: udt.month,
			year: udt.year,
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

	/// Returns an UNIX epoch [`Timestamp`] (`1970-01-01 00:00:00 UTC`).
	#[deprecated(note = "use Timestamp::EPOCH instead")]
	#[inline]
	pub fn epoch() -> Self {
		Self::EPOCH
	}

	/// Creates a [`Timestamp`] from a given [`std::time::SystemTime`].
	#[deprecated(note = "use Timestamp::try_from() instead")]
	pub fn from_system_time(st: std::time::SystemTime) -> Self {
		Self::try_from(st).expect("failed to parse SystemTime")
	}

	/// Creates a [`Timestamp`] for the current time.
	pub fn now() -> Self {
		Self::try_from(time::SystemTime::now()).expect("failed to parse current SystemTime")
	}

	/// Updates a [`Timestamp`] in place, with the contents of another [`Timestamp`].
	pub fn copy_from(&mut self, other: &Self) {
		self.seconds = other.seconds;
		self.nanoseconds = other.nanoseconds;
	}

	/// Returns the number of seconds + nanoseconds since UNIX epoch.
	pub fn epoch_offset(&self) -> (u64, u32) {
		(self.seconds, self.nanoseconds)
	}

	/// Returns the number of days since UNIX epoch.
	pub fn days_since_epoch(&self) -> u64 {
		self.seconds / U64_SECONDS_IN_DAY
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
		TimestampParts::utc(self)
	}

	/// Returns a [`TimestampParts`] for the timezone, in the local timezone.
	pub fn as_local_parts(&self) -> TimestampParts<'i> {
		TimestampParts::local(self)
	}

	/// Returns a string representation for the timestamp, in a given [`Format`].
	pub fn as_string(&self, format: &Format) -> String {
		format.as_string(self)
	}

	/// Returns the string representation length for the given [`Format`].
	pub fn string_len(&self, format: &Format) -> usize {
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

	/// Returns a [`Timestamp`] with a [`time::Duration`] added to the timestamp.
	///
	/// # Errors
	///
	/// Returns [`TimestampOpError::Overflow`] if the sum exceeds the maximum [`Timestamp`] value.
	pub fn add_duration(&self, d: &Duration) -> Result<Timestamp, TimestampOpError> {
		let nanos = self.nanoseconds + d.subsec_nanos();

		let nanoseconds = nanos % U32_NANOS_IN_SECOND;
		let seconds = match self.seconds.checked_add(d.as_secs()).and_then(|s| s.checked_add((nanos / U32_NANOS_IN_SECOND) as u64)) {
			Some(s) => s,
			None => return Err(TimestampOpError::Overflow("total seconds overflow")),
		};

		Ok(Self { seconds, nanoseconds })
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
	///
	/// # Errors
	///
	/// Returns [`TimestampOpError::Underflow`] if `other` is later than `self`.
	pub fn diff_as_duration(&self, other: &Self) -> Result<Duration, TimestampOpError> {
		let self_nanos = self.as_nanos();
		let other_nanos = other.as_nanos();

		if other_nanos > self_nanos {
			return Err(TimestampOpError::Underflow("attempted to subtract a Timestamp from a smaller Timestamp"));
		}

		let delta = self_nanos - other_nanos;
		let secs = (delta / U128_NANOS_IN_SECOND) as u64;
		let nanos = (delta % U128_NANOS_IN_SECOND) as u32;

		Ok(Duration::new(secs, nanos))
	}
}

impl fmt::Display for Timestamp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_string(&Format::default()))
	}
}

impl PartialOrd for Timestamp {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(Ord::cmp(self, other))
	}
}

impl Ord for Timestamp {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cmp(other)
	}
}

impl Sub for Timestamp {
	type Output = Result<Duration, TimestampOpError>;

	fn sub(self, other: Self) -> Self::Output {
		self.diff_as_duration(&other)
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod tests {
	use super::*;

	use crate::constant::{TIMESTAMP_MAX_MILLISECONDS, TIMESTAMP_MAX_NANOSECONDS};

	#[test]
	fn new() {
		assert_eq!(Timestamp::new(0, 0), Ok(Timestamp::EPOCH));
		assert_eq!(Timestamp::new(1234, 2000000789), Ok(Timestamp { seconds: 1236, nanoseconds: 789 }));
		assert_eq!(Timestamp::new(1772858897, 123000000), Ok(Timestamp::from_millis(1772858897123).unwrap()));
		assert_eq!(Timestamp::new(u64::MAX, 123), Ok(Timestamp { seconds: u64::MAX, nanoseconds: 123 }));
		assert_eq!(Timestamp::new(u64::MAX, 3123456789), Err(CastError::Overflow("total seconds overflow")));
	}

	#[test]
	fn cast_from() {
		// cast from seconds
		assert_eq!(Timestamp::from_secs(0), Timestamp::EPOCH);
		assert_eq!(Timestamp::from_secs(u64::MAX), Timestamp { seconds: u64::MAX, nanoseconds: 0 });
		assert_eq!(Timestamp::from_secs(1772858897), Timestamp::try_from((2026, 3, 7, 4, 48, 17)).unwrap());

		// cast from milliseconds
		assert_eq!(Timestamp::from_millis(0), Ok(Timestamp::EPOCH));
		assert_eq!(Timestamp::from_millis(1772858897123), Ok(Timestamp::try_from((2026, 3, 7, 4, 48, 17, 123, 0)).unwrap()));
		assert_eq!(
			Timestamp::from_millis(TIMESTAMP_MAX_MILLISECONDS),
			Ok(Timestamp {
				seconds: 18446744073709551615,
				nanoseconds: 999000000
			})
		);
		assert_eq!(
			Timestamp::from_millis(TIMESTAMP_MAX_MILLISECONDS + 1),
			Err(CastError::Overflow("milliseconds from epoch is larger than what Timestamp can represent"))
		);

		// cast from nanoseconds
		assert_eq!(Timestamp::from_nanos(0), Ok(Timestamp::EPOCH));
		assert_eq!(Timestamp::from_nanos(1772858897123456789), Ok(Timestamp::try_from((2026, 3, 7, 4, 48, 17, 123, 456789)).unwrap()));
		assert_eq!(
			Timestamp::from_nanos(TIMESTAMP_MAX_NANOSECONDS),
			Ok(Timestamp {
				seconds: 18446744073709551615,
				nanoseconds: 999999999,
			})
		);
		assert_eq!(
			Timestamp::from_nanos(TIMESTAMP_MAX_NANOSECONDS + 1),
			Err(CastError::Overflow("nanoseconds from epoch is larger than what Timestamp can represent"))
		);
	}

	#[test]
	fn try_from() {
		assert_eq!(Timestamp::try_from(std::time::UNIX_EPOCH), Ok(Timestamp::EPOCH));
		assert!(Timestamp::try_from(std::time::SystemTime::now()).is_ok());
		assert!(
			Timestamp::try_from(std::time::UNIX_EPOCH - std::time::Duration::from_secs(1)).is_err(),
			"casting from pre-epoch SystemTime must fail"
		);

		assert_eq!(Timestamp::try_from((2026, 3, 7)), Ok(Timestamp { seconds: 1772841600, nanoseconds: 0 }), "cast from UTC date");
		assert_eq!(
			Timestamp::try_from((2026, 3, 7, 4, 48, 17)),
			Ok(Timestamp { seconds: 1772858897, nanoseconds: 0 }),
			"cast from UTC date+time"
		);
		assert_eq!(
			Timestamp::try_from((2026, 3, 7, 4, 48, 17, 446, 37892)),
			Ok(Timestamp {
				seconds: 1772858897,
				nanoseconds: 446037892,
			}),
			"cast from UTC date+time+millis"
		);

		assert_eq!(
			Timestamp::try_from(UTCDateTime {
				year: 2026,
				month: 3,
				day: 7,
				hour: 4,
				minutes: 48,
				seconds: 17,
				milliseconds: 446,
				nanoseconds: 37892,
			}),
			Ok(Timestamp {
				seconds: 1772858897,
				nanoseconds: 446037892,
			}),
			"cast from UTCDateTime"
		);
	}

	#[test]
	fn comparison() {
		assert_eq!(Timestamp::new(1234, 100).unwrap(), Timestamp::new(1234, 100).unwrap(), "equal");
		assert!(Timestamp::new(1234, 100).unwrap() > Timestamp::new(1234, 50).unwrap(), "same secs, more millis");
		assert!(Timestamp::new(1234, 100).unwrap() < Timestamp::new(1234, 200).unwrap(), "same secs, less millis");
		assert!(Timestamp::new(5678, 100).unwrap() > Timestamp::new(1234, 100).unwrap(), "more secs");
		assert!(Timestamp::new(1234, 100).unwrap() < Timestamp::new(5768, 100).unwrap(), "less secs");
	}

	#[test]
	fn operators() {
		// addition
		assert_eq!(Timestamp::from_secs(1234).add_duration(&Duration::ZERO), Ok(Timestamp::from_secs(1234)));
		assert_eq!(
			Timestamp::from_nanos(TIMESTAMP_MAX_NANOSECONDS).unwrap().add_duration(&Duration::from_nanos(1)),
			Err(TimestampOpError::Overflow("total seconds overflow")),
		);
		assert_eq!(
			Timestamp::new(0, 999999).unwrap().add_duration(&Duration::MAX),
			Err(TimestampOpError::Overflow("total seconds overflow"))
		);

		// subtraction
		assert_eq!(Timestamp::from_secs(1234) - Timestamp::from_secs(1234), Ok(Duration::ZERO));
		assert_eq!(
			Timestamp::from_nanos(1234).unwrap() - Timestamp::from_nanos(5768).unwrap(),
			Err(TimestampOpError::Underflow("attempted to subtract a Timestamp from a smaller Timestamp"))
		);
		assert_eq!(Timestamp::from_secs(5678) - Timestamp::from_secs(1234), Ok(Duration::from_secs(4444)));
	}

	#[test]
	fn copy_from() {
		let mut ts_1 = Timestamp::new(1234, 888).unwrap();
		let ts_2 = Timestamp::new(5678, 999).unwrap();
		assert_ne!(ts_1, ts_2);

		ts_1.copy_from(&ts_2);
		assert_eq!(ts_1, ts_2);
		assert_eq!(ts_1, Timestamp::new(5678, 999).unwrap());
	}

	#[test]
	fn days_since_epoch() {
		assert_eq!(Timestamp::EPOCH.days_since_epoch(), 0);
		assert_eq!(Timestamp::try_from((1980, 7, 19, 18, 15, 0)).unwrap().days_since_epoch(), 3852);
		assert_eq!(Timestamp::new(1772457020, 789).unwrap().days_since_epoch(), 20514);
		assert_eq!(Timestamp::from_secs(1772457213).days_since_epoch(), 20514);
		assert_eq!(Timestamp::from_millis(1772457213123).unwrap().days_since_epoch(), 20514);
	}

	#[test]
	fn casting_to() {
		let ts = Timestamp::new(1772457319, 38123456).unwrap();
		assert_eq!(ts.as_secs(), 1772457319, "cast to seconds");
		assert_eq!(ts.as_millis(), 1772457319038, "cast to millis");
		assert_eq!(ts.as_nanos(), 1772457319038123456, "cast to nanos");
	}

	#[test]
	fn to_string() {
		// the Display trait implementation serializes Timestamps into UTC time strings.
		assert_eq!(Timestamp::new(1772457020, 789).unwrap().to_string(), "2026-03-02 13:10:20.000",);
		assert_eq!(Timestamp::from_secs(1772457213).to_string(), "2026-03-02 13:13:33.000");
		assert_eq!(Timestamp::from_millis(1772457213123).unwrap().to_string(), "2026-03-02 13:13:33.123");
		assert_eq!(Timestamp::try_from((2026, 3, 6, 14, 43, 39, 128, 564)).expect("invalid parts").to_string(), "2026-03-06 14:43:39.128");
	}

	#[test]
	fn utc_parts_conversion() {
		assert_eq!(
			Timestamp::from_millis(1772457319335).unwrap().as_utc_parts(),
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
			Timestamp::from_nanos(1772457319335012345).unwrap().as_utc_parts(),
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
		let a = Timestamp::new(1234, 5678).unwrap();
		let b = a.add_duration(&Duration::from_millis(2234));
		assert_eq!(b, Ok(Timestamp::new(1236, 234005678).unwrap()));
	}
}
