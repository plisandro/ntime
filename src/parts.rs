use super::Timestamp;

use crate::c_bindings;
use crate::constant::{
	DAY_NAMES, DAYS_IN_MONTH_COMMON_YEAR, DAYS_IN_MONTH_LEAP_YEAR, MONTH_NAMES, MONTH_TO_DAYS_COMMON_YEAR, MONTH_TO_DAYS_LEAP_YEAR, TIMESTAMP_MAX_LOCAL_SECONDS, TIMESTAMP_MAX_LOCAL_YEAR,
	TIMESTAMP_MAX_YEAR, TIMESTAMP_MIN_YEAR, TIMEZONE_UTC, U8_DAYS_IN_WEEK, U8_HOURS_IN_DAY, U8_MINUTES_IN_HOUR, U8_MONTHS_IN_YEAR, U8_SECONDS_IN_MINUTE, U16_DAYS_IN_COMMON_YEAR,
	U16_DAYS_IN_LEAP_YEAR, U16_MILLIS_IN_SECOND, U32_NANOS_IN_MILLI, U32_SECONDS_IN_HOUR, U32_SECONDS_IN_MINUTE, U64_DAYS_IN_4_YEARS, U64_DAYS_IN_400_YEAR_CYCLE, U64_LEAP_YEARS_BEFORE_EPOCH,
	U64_SECONDS_IN_COMMON_YEAR, U64_SECONDS_IN_DAY, U64_SECONDS_IN_HOUR, U64_SECONDS_IN_MINUTE, U64_TM_YEAR_OFFSET, U64_UNIX_EPOCH_SHIFT_IN_DAYS,
};
use crate::error::CastError;

/// A decomposition of a [`Timestamp`] into date + time parts, for a given timezone.
#[derive(Clone, Debug, PartialEq)]
pub struct TimestampParts<'l> {
	/// Nanoseconds after the millisecond (`0`-`999999`).
	pub nanoseconds: u32,
	/// Milliseconds after the second (`0`-`999`).
	pub milliseconds: u16,
	/// Seconds after the minute (`0`-`59`).
	pub seconds: u8,
	/// Minutes after the hour (`0`-`59`).
	pub minutes: u8,
	/// Hours since midnight (`0`-`23`).
	pub hour: u8,
	/// Day of the month (`1`-`31`).
	pub month_day: u8,
	/// Month of the year (`1`-`12`).
	pub month: u8,
	/// Year (`1970`-`584554051223`).
	pub year: u64,
	/// Day of the week, beginning from Sunday (`1`-`7`).
	pub week_day: u8,
	/// Day of the year (`1`-`366`).
	pub year_day: u16,
	/// Whether the timezone GMT offset is negative or positive.
	pub gmt_offset_negative: bool,
	/// Timezone GMT offset, in hours.
	pub gmt_offset_hours: u8,
	/// Timezone GMT offset, in minutes.
	pub gmt_offset_minutes: u8,
	/// Timezone abbreviation. Note this is OS-provided, and not necessarily an IANA identifier.
	pub timezone: &'l str,
}

impl<'i> TimestampParts<'i> {
	/// [`TimestampParts`] for UNIX epoch (`1970-01-01 00:00 UTC`)
	pub const EPOCH: Self = Self {
		nanoseconds: 0,
		milliseconds: 0,
		seconds: 0,
		minutes: 0,
		hour: 0,
		month_day: 1,
		month: 1,
		year: TIMESTAMP_MIN_YEAR,
		week_day: 5,
		year_day: 1,
		gmt_offset_negative: false,
		gmt_offset_hours: 0,
		gmt_offset_minutes: 0,
		timezone: TIMEZONE_UTC,
	};

	/// Returns an UNIX epoch [`TimestampParts`] (`1970-01-01 00:00:00 UTC`).
	#[deprecated(note = "use TimestampParts::EPOCH instead")]
	#[inline]
	pub fn epoch() -> Self {
		Self::EPOCH
	}

	/// Resolves whether a given year is a leap year.
	fn _is_leap_year(year: u64) -> bool {
		// Implements an short-circuit version of the Drepper-Neri-Schneider algorithm:
		// https://github.com/gcc-mirror/gcc/commit/86a0df1a6c7fe4a835620b868e76ea78d42d6620 .
		// Relies on the fact that most compilers will optimize "mod 25" without divisions.
		(year & 3 == 0) && (!year.is_multiple_of(25) || year & 15 == 0)
	}

	/// Creates a UTC [`TimestampParts`] from a given [`Timestamp`].
	pub fn utc(ts: &Timestamp) -> Self {
		let (seconds, nanos) = ts.epoch_offset();

		let days_since_epoch = seconds / U64_SECONDS_IN_DAY;
		let day_total_seconds = seconds % U64_SECONDS_IN_DAY;
		let day_milliseconds = nanos / U32_NANOS_IN_MILLI;
		let day_nanoseconds = nanos % U32_NANOS_IN_MILLI;

		// Implements Ben Joffe's Fast Time of Day algorithm.
		// See https://www.benjoffe.com/fast-time-of-day for details.
		const M_MUL: u64 = ((1 << 32) / U64_SECONDS_IN_MINUTE + 1) << 32;
		const H_MUL: u64 = ((1 << 32) / U64_SECONDS_IN_HOUR + 1) << 32;
		let hprd: u128 = (day_total_seconds as u128).wrapping_mul(H_MUL as u128);
		let mlow = day_total_seconds.wrapping_mul(M_MUL);
		let hlow = hprd as u64;

		let day_hour = hprd.wrapping_shr(64);
		let day_minutes = (hlow as u128).wrapping_mul(60).wrapping_shr(64) as u64;
		let day_seconds = (mlow as u128).wrapping_mul(60).wrapping_shr(64) as u64;

		// Implements Ben Joffe's Fast Day of The Week algorithm.
		// See https://www.benjoffe.com/fast-day-of-week for details.
		const M: u64 = 0xFFFFFFFFFFFFFFFF / 7;
		const Z: u64 = 0x9400 << 48;
		let a = days_since_epoch.wrapping_mul(M).wrapping_add(Z);
		let b = days_since_epoch.wrapping_shr(2) + days_since_epoch.wrapping_shr(5);

		let week_day = a.wrapping_add(b).wrapping_shr(61).wrapping_add(1);

		// Implements Ben Joffe's Julian Map algorithm, based on the work of Neri-Schneider et all. Good stuff!
		// See https://www.benjoffe.com/fast-date for details.
		// Note that year 1 is a leap year in the Julian calendar.
		let days = days_since_epoch + U64_UNIX_EPOCH_SHIFT_IN_DAYS + MONTH_TO_DAYS_LEAP_YEAR[10] as u64 + 1;
		// quarter-days since 0000-02-28 06:00
		let qday = days * 4 + 3;
		// century-Februaries elapsed
		let cent = qday / U64_DAYS_IN_400_YEAR_CYCLE;
		// map to Julian quarter-Day
		let qjul = qday - (cent & !3) + cent * 4;
		// year (later incremented if Jan/Feb)
		let mut year = qjul / U64_DAYS_IN_4_YEARS;
		// day of year (starting March 1st)
		let yday = (qjul % U64_DAYS_IN_4_YEARS) / 4;
		// Neri‑Schneider equivalent of...
		let n = yday * 2141 + 197913;
		// M = (yday * 5 + 2) / 153
		let mut month = n / 65536;
		// D = yday - (M * 153 + 2) / 5
		let d = n % 65536 / 2141;

		let day = d + 1;

		let year_day = if yday > MONTH_TO_DAYS_LEAP_YEAR[10] as u64 {
			// check if Jan or Feb
			month -= 12;
			year += 1;

			yday - MONTH_TO_DAYS_LEAP_YEAR[10] as u64
		} else {
			// adjust for leap years
			yday + if Self::_is_leap_year(year) {
				MONTH_TO_DAYS_LEAP_YEAR[2] + 1
			} else {
				MONTH_TO_DAYS_COMMON_YEAR[2] + 1
			} as u64
		};

		TimestampParts {
			nanoseconds: day_nanoseconds as _,
			milliseconds: day_milliseconds as _,
			seconds: day_seconds as _,
			minutes: day_minutes as _,
			hour: day_hour as _,
			month_day: day as _,
			month: month as _,
			year: year as _,
			week_day: week_day as _,
			year_day: year_day as _,
			gmt_offset_negative: false,
			gmt_offset_hours: 0 as _,
			gmt_offset_minutes: 0 as _,
			timezone: TIMEZONE_UTC,
		}
	}

	/// Creates a local timezone [`TimestampParts`] from a given [`Timestamp`].
	pub fn local(ts: &Timestamp) -> Self {
		let (seconds, nanos) = ts.epoch_offset();
		if seconds > TIMESTAMP_MAX_LOCAL_SECONDS {
			panic!("timestamp {ts:?} cannot be converted to local timezone due to libc limitations");
		}

		let c_secs = seconds as c_bindings::CTime;
		let tm = match c_bindings::c_time_to_local_tm(c_secs) {
			Some(tm) => tm,
			None => panic!("failed to parse local parts for {ts:?}"),
		};

		let gmt_offset_secs: i32;
		let timezone: &str;
		#[cfg(not(target_env = "msvc"))]
		{
			gmt_offset_secs = tm.tm_gmtoff as _;
			timezone = c_bindings::c_timezone_from_tm(&tm);
		}
		#[cfg(target_env = "msvc")]
		{
			let is_dst = tm.tm_isdst > 0;
			gmt_offset_secs = c_bindings::c_tz_gmt_offset(is_dst);
			let (standard_tz, dst_tz) = &*c_bindings::MSVC_TZ_NAMES;
			timezone = (if is_dst { dst_tz } else { standard_tz }).as_str();
		}

		let (gmt_offset_negative, gmt_offset_hours, gmt_offset_minutes) = Self::_gmt_offset_parts(gmt_offset_secs);

		TimestampParts {
			nanoseconds: (nanos % U32_NANOS_IN_MILLI) as _,
			milliseconds: (nanos / U32_NANOS_IN_MILLI) as _,
			seconds: tm.tm_sec as _,
			minutes: tm.tm_min as _,
			hour: tm.tm_hour as _,
			month_day: tm.tm_mday as _,
			month: (1 + tm.tm_mon) as _,
			year: tm.tm_year as u64 + U64_TM_YEAR_OFFSET,
			week_day: (1 + tm.tm_wday) as _,
			year_day: (1 + tm.tm_yday) as _,
			gmt_offset_negative,
			gmt_offset_hours,
			gmt_offset_minutes,
			timezone,
		}
	}

	/// Converts a GMT offset in signed seconds to (is_negative, hours, minutes).
	fn _gmt_offset_parts(gmt_offset_secs: i32) -> (bool, u8, u8) {
		let secs: u32;
		let negative: bool;

		if gmt_offset_secs >= 0 {
			negative = false;
			secs = gmt_offset_secs as u32;
		} else {
			negative = true;
			secs = -gmt_offset_secs as u32;
		}

		let hours = (secs / U32_SECONDS_IN_HOUR) as u8;
		let mins = ((secs % U32_SECONDS_IN_HOUR) / U32_SECONDS_IN_MINUTE) as u8;

		(negative, hours, mins)
	}

	/// Computes day of the year given a [`TimestampParts]` year, month and day of the month.
	fn year_day(&self) -> u16 {
		let month_to_days = (if self.is_leap_year() { MONTH_TO_DAYS_LEAP_YEAR } else { MONTH_TO_DAYS_COMMON_YEAR })[(self.month - 1) as usize];
		month_to_days + (self.month_day as u16)
	}

	/// Verifies whether a given [`TimestampParts`] is in UTC timezone, or +00:00 GMT.
	#[inline]
	pub fn is_utc(&self) -> bool {
		self.gmt_offset_hours == 0 && self.gmt_offset_minutes == 0
	}

	/// Returns a sign string for the timezone GMT offset (either `+` or `-`).
	pub fn gmt_offset_sign(&self) -> &'i str {
		if self.gmt_offset_negative { "-" } else { "+" }
	}

	/// Returns the GMT offset in seconds (f.ex. 7200 for +02:00).
	pub fn gmt_offset_seconds(&self) -> i32 {
		let offset = ((self.gmt_offset_hours as u32 * U32_SECONDS_IN_HOUR) + (self.gmt_offset_minutes as u32 * U32_SECONDS_IN_MINUTE)) as i32;

		if self.gmt_offset_negative { -offset } else { offset }
	}

	/// Returns a short day name: `Tue`
	pub fn day_name(&self) -> &str {
		if self.week_day == 0 {
			panic!("invalid week day for {self:?}");
		}
		DAY_NAMES[((self.week_day - 1) % U8_DAYS_IN_WEEK) as usize]
	}

	/// Returns a short month name: `Mar`
	pub fn month_name(&self) -> &str {
		if self.month == 0 {
			panic!("invalid month for {self:?}");
		}
		MONTH_NAMES[((self.month - 1) % U8_MONTHS_IN_YEAR) as usize]
	}

	/// Resolves whether the year is a leap year ([`true`]) or common ([`false`]).
	pub fn is_leap_year(&self) -> bool {
		Self::_is_leap_year(self.year)
	}

	/// Resolves the total number of leap since UNIX epoch.
	pub fn leap_years_since_epoch(&self) -> u64 {
		if self.year < TIMESTAMP_MIN_YEAR {
			return 0;
		}
		let year = self.year - 1;
		let leap_years = (year / 4) - (year / 100) + (year / 400);

		leap_years - U64_LEAP_YEARS_BEFORE_EPOCH
	}

	/// Validates [`TimestampParts`] individual fields for correctness.
	// TODO: use Overflow/Underflow when necessary
	fn validate_fields(&self) -> Result<(), CastError> {
		let max_year = if self.is_utc() { TIMESTAMP_MAX_YEAR } else { TIMESTAMP_MAX_LOCAL_YEAR };

		if self.nanoseconds >= U32_NANOS_IN_MILLI {
			return Err(CastError::TimestampParts("invalid nanoseconds field".into()));
		}
		if self.milliseconds >= U16_MILLIS_IN_SECOND {
			return Err(CastError::TimestampParts("invalid milliseconds field".into()));
		}
		if self.seconds >= U8_SECONDS_IN_MINUTE {
			return Err(CastError::TimestampParts("invalid seconds field".into()));
		}
		if self.minutes >= U8_MINUTES_IN_HOUR {
			return Err(CastError::TimestampParts("invalid minutes field".into()));
		}
		if self.hour >= U8_HOURS_IN_DAY {
			return Err(CastError::TimestampParts("invalid hour field".into()));
		}
		if self.month < 1 || self.month > U8_MONTHS_IN_YEAR {
			return Err(CastError::TimestampParts("invalid month field".into()));
		}
		if self.year < TIMESTAMP_MIN_YEAR || self.year > max_year {
			return Err(CastError::TimestampParts("invalid year field".into()));
		}
		if self.month_day < 1 || self.month_day > (if self.is_leap_year() { DAYS_IN_MONTH_LEAP_YEAR } else { DAYS_IN_MONTH_COMMON_YEAR })[(self.month - 1) as usize] {
			return Err(CastError::TimestampParts("invalid month_day field".into()));
		};
		if self.week_day < 1 || self.week_day > U8_DAYS_IN_WEEK {
			return Err(CastError::TimestampParts("invalid week_day field".into()));
		}
		if self.year_day < 1 || self.year_day > (if self.is_leap_year() { U16_DAYS_IN_LEAP_YEAR } else { U16_DAYS_IN_COMMON_YEAR }) {
			return Err(CastError::TimestampParts("invalid year_day field".into()));
		}
		// no checks for self.gmt_offset_negative
		if self.gmt_offset_hours >= U8_HOURS_IN_DAY {
			return Err(CastError::TimestampParts("invalid gmt_offset_hours field".into()));
		}
		if self.gmt_offset_minutes >= U8_MINUTES_IN_HOUR {
			return Err(CastError::TimestampParts("invalid gmt_offset_minutes field".into()));
		}
		// no checks for self.timezone

		Ok(())
	}

	/// Validates [`TimestampParts`] for correctness.
	///
	/// # Errors
	///
	/// Returns a [`CastError`] upon invalid [`TimestampParts`] fields.
	pub fn validate(&self) -> Result<(), CastError> {
		self.validate_fields()?;

		// ensure the provided day of the year matches with other fields
		let expected_year_day = self.year_day();
		if self.year_day != expected_year_day {
			return Err(CastError::TimestampParts("year_day doesn't match year + month + month_day".into()));
		}

		Ok(())
	}

	/// Converts the parts structure back into a [`Timestamp`], much like glibc's timegm().
	///
	/// # Errors
	///
	/// Returns a [`CastError`] upon invalid [`TimestampParts`] fields.
	// TODO: use Overflow/Underflow when necessary
	pub fn to_timestamp(&self) -> Result<Timestamp, CastError> {
		self.validate_fields()?;

		// See https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_15 for
		// a description of this algorithm:
		//
		// seconds_since_epoch = tm_sec + tm_min*60 + tm_hour*3600 + tm_yday*86400 +
		//                       (tm_year-70)*31536000 + ((tm_year-69)/4)*86400 -
		//                       ((tm_year-1)/100)*86400 + ((tm_year+299)/400)*86400
		let tm_sec = self.seconds as u64;
		let tm_min = self.minutes as u64;
		let tm_hour = self.hour as u64;
		let tm_year = self.year - TIMESTAMP_MIN_YEAR;
		let tm_yday = (self.year_day() - 1) as u64;
		let leap_years = self.leap_years_since_epoch();

		// compute total seconds since epoch...
		let mut secs = tm_sec + tm_min * U64_SECONDS_IN_MINUTE + tm_hour * U64_SECONDS_IN_HOUR + tm_yday * U64_SECONDS_IN_DAY;
		let year_secs = tm_year.checked_mul(U64_SECONDS_IN_COMMON_YEAR).ok_or(CastError::from("timestamp year overflow"))?;
		let leap_year_secs = leap_years.checked_mul(U64_SECONDS_IN_DAY).ok_or(CastError::from("timestamp leap years overflow"))?;
		secs = secs.checked_add(year_secs).ok_or(CastError::from("timestamp overflow"))?;
		secs = secs.checked_add(leap_year_secs).ok_or(CastError::from("timestamp overflow"))?;

		// ..and add GMT offset.
		let gmt_offset_secs = (self.gmt_offset_minutes as u64) * U64_SECONDS_IN_MINUTE + (self.gmt_offset_hours as u64) * U64_SECONDS_IN_HOUR;
		if self.gmt_offset_negative {
			secs = secs.checked_add(gmt_offset_secs).ok_or(CastError::from("timestamp GMT offset overflow"))?;
		} else {
			secs = secs.checked_sub(gmt_offset_secs).ok_or(CastError::from("timestamp GMT offset underflow"))?;
		};

		let nanos = self.nanoseconds + ((self.milliseconds as u32) * U32_NANOS_IN_MILLI);

		Ok(super::Timestamp::new(secs, nanos))
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod test {
	use super::*;

	use crate::format::{Format, Precision, Timezone};
	use crate::test_helpers;

	#[test]
	fn is_utc() {
		let utc = TimestampParts::EPOCH;

		let mut local = TimestampParts::EPOCH;
		local.hour = 17;
		local.gmt_offset_hours = 4;
		local.timezone = "Asia/Baku";

		assert!(utc.is_utc());
		assert!(!local.is_utc());
	}

	#[test]
	fn gmt_offset_parts() {
		assert_eq!(TimestampParts::_gmt_offset_parts(30600), (false, 8, 30));
		assert_eq!(TimestampParts::_gmt_offset_parts(-13500), (true, 3, 45));
	}

	#[test]
	fn gmt_offset_seconds() {
		for (parts, want) in [
			(TimestampParts::EPOCH, 0),
			(
				TimestampParts {
					gmt_offset_negative: true,
					gmt_offset_hours: 9,
					gmt_offset_minutes: 30,
					timezone: "Pacific/Marquesas",
					..TimestampParts::EPOCH
				},
				-34200,
			),
			(
				TimestampParts {
					gmt_offset_negative: true,
					gmt_offset_hours: 6,
					gmt_offset_minutes: 0,
					timezone: "America/Edmonton",
					..TimestampParts::EPOCH
				},
				-21600,
			),
			(
				TimestampParts {
					gmt_offset_negative: false,
					gmt_offset_hours: 4,
					gmt_offset_minutes: 0,
					timezone: "Asia/Baku",
					..TimestampParts::EPOCH
				},
				14400,
			),
			(
				TimestampParts {
					gmt_offset_negative: false,
					gmt_offset_hours: 3,
					gmt_offset_minutes: 30,
					timezone: "Asia/Tehran",
					..TimestampParts::EPOCH
				},
				12600,
			),
		] {
			assert_eq!(parts.gmt_offset_seconds(), want, "{parts:?}");
		}
	}

	#[test]
	fn is_leap_year() {
		let mut parts = TimestampParts {
			nanoseconds: 0,
			milliseconds: 0,
			seconds: 0,
			minutes: 0,
			hour: 0,
			month_day: 0,
			month: 0,
			year: 0,
			week_day: 0,
			year_day: 0,
			gmt_offset_negative: false,
			gmt_offset_hours: 0,
			gmt_offset_minutes: 0,
			timezone: "none",
		};

		parts.year = 1900;
		assert!(!parts.is_leap_year());

		parts.year = 2000;
		assert!(parts.is_leap_year());

		parts.year = 2001;
		assert!(!parts.is_leap_year());

		parts.year = 2020;
		assert!(parts.is_leap_year());

		parts.year = 2026;
		assert!(!parts.is_leap_year());
	}

	#[test]
	fn leap_years_since_epoch() {
		let mut parts = TimestampParts::EPOCH;

		assert_eq!(parts.leap_years_since_epoch(), 0);
		parts.year = 1971;
		assert_eq!(parts.leap_years_since_epoch(), 0);
		// 1972 is the first leap year since UNIX epoch
		parts.year = 1972;
		assert_eq!(parts.leap_years_since_epoch(), 0);
		parts.year = 1973;
		assert_eq!(parts.leap_years_since_epoch(), 1);

		parts.year = 2026;
		assert_eq!(parts.leap_years_since_epoch(), 14);
		// 2028 is a leap year
		parts.year = 2028;
		assert_eq!(parts.leap_years_since_epoch(), 14);
		parts.year = 2029;
		assert_eq!(parts.leap_years_since_epoch(), 15);
	}

	#[test]
	#[should_panic]
	fn parts_conversion_max_timestamp() {
		_ = Timestamp::MAX.as_local_parts();
	}

	#[test]
	#[should_panic]
	fn parts_conversion_timestamp_overflow() {
		let _ = Timestamp::from_secs(TIMESTAMP_MAX_LOCAL_SECONDS + 1).as_local_parts();
	}

	#[test]
	fn parts_field_validation() {
		let mut parts = TimestampParts::EPOCH;

		// rely on the order fields are checked so we don't build a new TimestampParts every time
		parts.nanoseconds = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.nanoseconds = 999999;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.nanoseconds = 1000000;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid nanoseconds field".into())));
		parts.nanoseconds = 10037;

		parts.milliseconds = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.milliseconds = 999;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.milliseconds = 1000;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid milliseconds field".into())));
		parts.milliseconds = 2196;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid milliseconds field".into())));
		parts.milliseconds = 761;

		parts.seconds = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.seconds = 59;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.seconds = 60;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid seconds field".into())));
		parts.seconds = 77;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid seconds field".into())));
		parts.seconds = 19;

		parts.minutes = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.minutes = 59;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.minutes = 60;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid minutes field".into())));
		parts.minutes = 77;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid minutes field".into())));
		parts.minutes = 58;

		parts.hour = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.hour = 2;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.hour = 24;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid hour field".into())));
		parts.hour = 22;

		parts.month = 0;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid month field".into())));
		parts.month = 1;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.month = 12;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.month = 15;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid month field".into())));
		parts.month = 2;

		parts.year = 1969;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid year field".into())));
		parts.year = u64::MAX;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid year field".into())));
		parts.year = 2026;

		parts.month_day = 0;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid month_day field".into())));
		parts.month_day = 1;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.month_day = 44;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid month_day field".into())));
		// 2026 is a common year, so...
		parts.month_day = 29;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid month_day field".into())));
		parts.month_day = 6;

		parts.week_day = 0;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid week_day field".into())));
		parts.week_day = 1;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.week_day = 7;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.week_day = 8;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid week_day field".into())));
		parts.week_day = 6;

		parts.year_day = 0;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid year_day field".into())));
		parts.year_day = 1;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.year_day = 365;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.year_day = 366;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid year_day field".into())));
		parts.year_day = 390;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid year_day field".into())));
		parts.year_day = 37;

		parts.gmt_offset_hours = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.gmt_offset_hours = 23;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.gmt_offset_hours = 24;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid gmt_offset_hours field".into())));
		parts.gmt_offset_hours = 3;

		parts.gmt_offset_minutes = 0;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.gmt_offset_minutes = 59;
		assert_eq!(parts.validate_fields(), Ok(()));
		parts.gmt_offset_minutes = 60;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid gmt_offset_minutes field".into())));
		parts.gmt_offset_minutes = 79;
		assert_eq!(parts.validate_fields(), Err(CastError::TimestampParts("invalid gmt_offset_minutes field".into())));
		parts.gmt_offset_minutes = 45;

		assert_eq!(parts.validate_fields(), Ok(()));
	}

	#[test]
	fn parts_validation() {
		let mut parts = TimestampParts::EPOCH;

		// rely on the order fields are checked so we don't build a new TimestampParts every time
		parts.nanoseconds = 1000037;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid nanoseconds field".into())));
		parts.nanoseconds = 10037;

		parts.milliseconds = 2196;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid milliseconds field".into())));
		parts.milliseconds = 761;

		parts.seconds = 77;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid seconds field".into())));
		parts.seconds = 19;

		parts.minutes = 77;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid minutes field".into())));
		parts.minutes = 58;

		parts.hour = 24;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid hour field".into())));
		parts.hour = 22;

		parts.month = 15;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid month field".into())));
		parts.month = 2;

		parts.year = 1969;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid year field".into())));
		parts.year = 2026;

		// 2026 is a common year, so...
		parts.month_day = 29;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid month_day field".into())));
		parts.month_day = 6;

		parts.week_day = 9;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid week_day field".into())));
		parts.week_day = 6;

		parts.year_day = 390;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid year_day field".into())));
		parts.year_day = 12;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("year_day doesn't match year + month + month_day".into())));
		parts.year_day = 37;

		parts.gmt_offset_hours = 25;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid gmt_offset_hours field".into())));
		parts.gmt_offset_hours = 3;

		parts.gmt_offset_minutes = 79;
		assert_eq!(parts.validate(), Err(CastError::TimestampParts("invalid gmt_offset_minutes field".into())));
		parts.gmt_offset_minutes = 45;

		assert_eq!(parts.validate(), Ok(()));
	}

	#[test]
	fn epoch_parts() {
		assert_eq!(Timestamp::EPOCH.as_utc_parts(), TimestampParts::EPOCH);
	}

	#[test]
	fn parts_to_timestamp_error() {
		for (parts, want) in [
			(
				TimestampParts {
					nanoseconds: 111,
					milliseconds: 222,
					seconds: 35,
					minutes: 0,
					hour: 0,
					month_day: 1,
					month: 1,
					year: 311,
					week_day: 1,
					year_day: 1,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
				Err(CastError::TimestampParts("invalid year field".into())),
			),
			(
				TimestampParts {
					nanoseconds: 111,
					milliseconds: 222,
					seconds: 35,
					minutes: 0,
					hour: 0,
					month_day: 1,
					month: 1,
					year: 1970,
					week_day: 1,
					year_day: 1,
					gmt_offset_negative: false,
					gmt_offset_hours: 4,
					gmt_offset_minutes: 0,
					timezone: "Asia/Baku",
				},
				Err(CastError::from("timestamp GMT offset underflow")),
			),
			// local year is too large
			(
				TimestampParts {
					nanoseconds: 0,
					milliseconds: 0,
					seconds: 0,
					minutes: 0,
					hour: 0,
					month_day: 1,
					month: 1,
					year: TIMESTAMP_MAX_LOCAL_YEAR + 1,
					week_day: 1,
					year_day: 1,
					gmt_offset_negative: true,
					gmt_offset_hours: 3,
					gmt_offset_minutes: 0,
					timezone: "Europe/Istanbul",
				},
				Err(CastError::TimestampParts("invalid year field".into())),
			),
			// Max supported date is 584554051223-11-09 07:00:15 UTC.
			(
				TimestampParts {
					nanoseconds: 456,
					milliseconds: 123,
					seconds: 16,
					minutes: 0,
					hour: 7,
					month_day: 9,
					month: 11,
					year: TIMESTAMP_MAX_YEAR,
					week_day: 1,
					year_day: 1,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
				Err(CastError::from("timestamp overflow")),
			),
		] {
			assert_eq!(parts.to_timestamp(), want, "{:?}", parts);
		}
	}

	#[test]
	fn parts_to_timestamp() {
		let mut parts = TimestampParts {
			nanoseconds: 123456,
			milliseconds: 320,
			seconds: 15,
			minutes: 22,
			hour: 5,
			month_day: 8,
			month: 3,
			year: 2026,
			week_day: 1,
			year_day: 67,
			gmt_offset_negative: false,
			gmt_offset_hours: 0,
			gmt_offset_minutes: 0,
			timezone: TIMEZONE_UTC,
		};

		// UTC parts
		assert_eq!(parts.to_timestamp().unwrap(), Timestamp::from_nanos(1772947335320123456).unwrap());

		// timezoned parts
		parts.gmt_offset_negative = true;
		parts.gmt_offset_hours = 9;
		parts.gmt_offset_minutes = 30;
		parts.timezone = "Pacific/Marquesas";
		assert_eq!(parts.to_timestamp().unwrap(), Timestamp::from_nanos(1772981535320123456).unwrap());
	}

	#[test]
	fn utc_to_and_from_parts() {
		for (ts, want_parts) in [
			// epoch
			(
				Timestamp::EPOCH,
				TimestampParts {
					nanoseconds: 0,
					milliseconds: 0,
					seconds: 0,
					minutes: 0,
					hour: 0,
					month_day: 1,
					month: 1,
					year: 1970,
					week_day: 5,
					year_day: 1,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// min supported timestamp
			(
				Timestamp::MIN,
				TimestampParts {
					nanoseconds: 0,
					milliseconds: 0,
					seconds: 0,
					minutes: 0,
					hour: 0,
					month_day: 1,
					month: 1,
					year: 1970,
					week_day: 5,
					year_day: 1,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// max supported timestamp
			(
				Timestamp::MAX,
				TimestampParts {
					nanoseconds: 999999,
					milliseconds: 999,
					seconds: 15,
					minutes: 0,
					hour: 7,
					month_day: 9,
					month: 11,
					year: 584554051223,
					week_day: 5,
					year_day: 313,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// max seconds timestamp
			(
				Timestamp::from_secs(u64::MAX),
				TimestampParts {
					nanoseconds: 0,
					milliseconds: 0,
					seconds: 15,
					minutes: 0,
					hour: 7,
					month_day: 9,
					month: 11,
					year: 584554051223,
					week_day: 5,
					year_day: 313,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// leap year
			(
				Timestamp::try_from((2024, 11, 15, 13, 35, 18, 123, 456)).expect("invalid parts"),
				TimestampParts {
					nanoseconds: 456,
					milliseconds: 123,
					seconds: 18,
					minutes: 35,
					hour: 13,
					month_day: 15,
					month: 11,
					year: 2024,
					week_day: 6,
					year_day: 320,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// leap year, January
			(
				Timestamp::try_from((2024, 1, 27, 7, 8, 9, 111, 222)).expect("invalid parts"),
				TimestampParts {
					nanoseconds: 222,
					milliseconds: 111,
					seconds: 9,
					minutes: 8,
					hour: 7,
					month_day: 27,
					month: 1,
					year: 2024,
					week_day: 7,
					year_day: 27,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// leap year, December
			(
				Timestamp::try_from((2024, 12, 15, 10, 11, 12, 333, 444)).expect("invalid parts"),
				TimestampParts {
					nanoseconds: 444,
					milliseconds: 333,
					seconds: 12,
					minutes: 11,
					hour: 10,
					month_day: 15,
					month: 12,
					year: 2024,
					week_day: 1,
					year_day: 350,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// non-leap year
			(
				Timestamp::try_from((2026, 3, 24, 18, 47, 31, 111, 222)).expect("invalid parts"),
				TimestampParts {
					nanoseconds: 222,
					milliseconds: 111,
					seconds: 31,
					minutes: 47,
					hour: 18,
					month_day: 24,
					month: 3,
					year: 2026,
					week_day: 3,
					year_day: 83,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// noon-leap year, February
			(
				Timestamp::try_from((2026, 2, 23, 7, 8, 9, 111, 222)).expect("invalid parts"),
				TimestampParts {
					nanoseconds: 222,
					milliseconds: 111,
					seconds: 9,
					minutes: 8,
					hour: 7,
					month_day: 23,
					month: 2,
					year: 2026,
					week_day: 2,
					year_day: 54,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
			// non-leap year, December
			(
				Timestamp::try_from((2026, 12, 17, 9, 11, 22, 333, 444)).expect("invalid parts"),
				TimestampParts {
					nanoseconds: 444,
					milliseconds: 333,
					seconds: 22,
					minutes: 11,
					hour: 9,
					month_day: 17,
					month: 12,
					year: 2026,
					week_day: 5,
					year_day: 351,
					gmt_offset_negative: false,
					gmt_offset_hours: 0,
					gmt_offset_minutes: 0,
					timezone: "UTC",
				},
			),
		] {
			let parts = ts.as_utc_parts();
			assert_eq!(parts, want_parts);

			let ts_from_parts: Timestamp = parts.to_timestamp().expect("invalid TimestampParts casted from Timestamp");
			assert_eq!(ts, ts_from_parts);
		}
	}

	#[test]
	fn local_offset() {
		test_helpers::mocks::with_timezone("America/Montevideo", || {
			// GMT -03:00
			let ts = Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts");
			assert_eq!(ts.as_string(&Format::DateTime(Timezone::Local, Precision::Milliseconds)), "2026-03-06 11:43:49.038 -0300");
			let parts = ts.as_local_parts();
			assert!(parts.gmt_offset_negative);
			assert_eq!(parts.gmt_offset_hours, 3);
			assert_eq!(parts.gmt_offset_minutes, 0);
		});

		test_helpers::mocks::with_timezone("Pacific/Samoa", || {
			// GMT -11:00
			let ts = Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts");
			assert_eq!(ts.as_string(&Format::DateTime(Timezone::Local, Precision::Milliseconds)), "2026-03-06 03:43:49.038 -1100");

			let parts = ts.as_local_parts();
			assert!(parts.gmt_offset_negative);
			assert_eq!(parts.gmt_offset_hours, 11);
			assert_eq!(parts.gmt_offset_minutes, 0);
		});

		test_helpers::mocks::with_timezone("Pacific/Kanton", || {
			// GMT +13:00
			let ts = Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts");
			assert_eq!(ts.as_string(&Format::DateTime(Timezone::Local, Precision::Milliseconds)), "2026-03-07 03:43:49.038 +1300");

			let parts = ts.as_local_parts();
			assert!(!parts.gmt_offset_negative);
			assert_eq!(parts.gmt_offset_hours, 13);
			assert_eq!(parts.gmt_offset_minutes, 0);
		});
	}

	#[test]
	fn utc_local_gmt_offset_match() {
		// verifies whether (local + GMT offset) parts match their UTC counterpart; this should *always*
		// be true, regardless of platform and local TZ settings.
		for ts in [
			Timestamp::try_from((2026, 1, 1)).expect("invalid timestamp for 2026-01-01 00:00:00"),
			Timestamp::try_from((2026, 7, 1)).expect("invalid timestamp for 2026-07-01 00:00:00"),
			Timestamp::try_from((2026, 7, 25, 11, 32, 12)).expect("invalid timestamp for 2026-07-25 11:32:12"),
		] {
			let local = &ts.as_local_parts();
			let gmt_offset_secs = local.gmt_offset_seconds();
			let gmt_shifted = &Timestamp::from_secs((ts.as_secs() as i64 + gmt_offset_secs as i64) as u64).as_utc_parts();

			assert_eq!(
				(local.year, local.month, local.month_day, local.hour, local.minutes, local.seconds),
				(gmt_shifted.year, gmt_shifted.month, gmt_shifted.month_day, gmt_shifted.hour, gmt_shifted.minutes, gmt_shifted.seconds),
				"local parts for {ts:?} don't match UTC + GMT offset parts: {local:?} vs. {gmt_shifted:?}",
			);
		}
	}
}
