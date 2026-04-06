use super::Timestamp;

use crate::c_bindings;
use crate::constant::{
	DAY_NAMES, DAYS_IN_MONTH_COMMON_YEAR, DAYS_IN_MONTH_LEAP_YEAR, MONTH_NAMES, TIMEZONE_UTC, U8_DAYS_IN_WEEK, U8_HOURS_IN_DAY, U8_MINUTES_IN_HOUR, U8_MONTHS_IN_YEAR, U8_SECONDS_IN_MINUTE,
	U16_DAYS_IN_COMMON_YEAR, U16_DAYS_IN_LEAP_YEAR, U16_MILLIS_IN_SECOND, U16_SECONDS_IN_HOUR, U16_SECONDS_IN_MINUTE, U16_UNIX_EPOCH_YEAR, U32_NANOS_IN_MILLI,
};

/// A decomposition of a [`Timestamp`] into date + time parts, for a given timezone.
#[derive(Clone, Debug, PartialEq)]
pub struct TimestampParts<'l> {
	/// Nanoseconds after the millisecond (0 - 999999)
	pub nanoseconds: u32,
	/// Milliseconds after the second (0 - 999)
	pub milliseconds: u16,
	/// Seconds after the minute (0 - 59)
	pub seconds: u8,
	/// Minutes after the hour (0 - 59)
	pub minutes: u8,
	/// Hours since midnight (0 - 23)
	pub hour: u8,
	/// Day of the month (1-31)
	pub month_day: u8,
	/// Month of the year (1-12)
	pub month: u8,
	/// Year.
	pub year: u16,
	/// Day of the week, begining from Sunday (1-7)
	pub week_day: u8,
	/// Day of the year (1-366)
	pub year_day: u16,
	/// Whether the timezone GMT offset is negative or positive.
	pub gmt_offset_negative: bool,
	/// Timezone GMT offset, in hours.
	pub gmt_offset_hours: u8,
	/// Timezone GMT offset, in minutes.
	pub gmt_offset_minutes: u8,
	/// Timezone abbreviation.
	pub timezone: &'l str,
}

impl<'i> TimestampParts<'i> {
	/// Converts a GMT offset in signed seconds to (is_negative, hours, minutes).
	fn _gmt_offset_parts(gmt_offset_secs: i16) -> (bool, u8, u8) {
		let secs: u16;
		let negative: bool;

		if gmt_offset_secs >= 0 {
			negative = false;
			secs = gmt_offset_secs as u16;
		} else {
			negative = true;
			secs = -gmt_offset_secs as u16;
		}

		let hours = (secs / U16_SECONDS_IN_HOUR) as u8;
		let mins = ((secs % U16_SECONDS_IN_HOUR) / U16_SECONDS_IN_MINUTE) as u8;

		(negative, hours, mins)
	}

	/// Creates a UTC [`TimestampParts`] from a given [`Timestamp`].
	pub fn utc(ts: &Timestamp) -> Self {
		let (seconds, nanos) = ts.epoch_offset();
		let ts = seconds as c_bindings::CTime;
		let tm = match c_bindings::c_time_to_utc_tm(ts) {
			Some(tm) => tm,
			None => panic!("failed to parse UTC parts for timestamp={seconds}s"),
		};

		TimestampParts {
			nanoseconds: (nanos % U32_NANOS_IN_MILLI) as _,
			milliseconds: (nanos / U32_NANOS_IN_MILLI) as _,
			seconds: tm.tm_sec as _,
			minutes: tm.tm_min as _,
			hour: tm.tm_hour as _,
			month_day: tm.tm_mday as _,
			month: (1 + tm.tm_mon) as _,
			year: (1900 + tm.tm_year) as _,
			week_day: (1 + tm.tm_wday) as _,
			year_day: (1 + tm.tm_yday) as _,
			gmt_offset_negative: false,
			gmt_offset_hours: 0 as _,
			gmt_offset_minutes: 0 as _,
			timezone: TIMEZONE_UTC,
		}
	}

	/// Creates a local timezone [`TimestampParts`] from a given [`Timestamp`].
	pub fn local(ts: &Timestamp) -> Self {
		let (seconds, nanos) = ts.epoch_offset();
		let ts = seconds as c_bindings::CTime;
		let tm = match c_bindings::c_time_to_local_tm(ts) {
			Some(tm) => tm,
			None => panic!("failed to parse local parts for timestamp={seconds}s"),
		};

		let gmt_offset_secs: i16;
		let timezone: &str;
		#[cfg(not(target_env = "msvc"))]
		{
			gmt_offset_secs = tm.tm_gmtoff as _;
			timezone = c_bindings::c_timezone_from_tm(&tm);
		}
		#[cfg(target_env = "msvc")]
		{
			(timezone, gmt_offset_secs) = c_bindings::c_tz_info();
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
			year: (1900 + tm.tm_year) as _,
			week_day: (1 + tm.tm_wday) as _,
			year_day: (1 + tm.tm_yday) as _,
			gmt_offset_negative: gmt_offset_negative,
			gmt_offset_hours: gmt_offset_hours,
			gmt_offset_minutes: gmt_offset_minutes,
			timezone: timezone,
		}
	}

	/// Returns a sign string for the timezone GTM offset (either `+` or `-`).
	pub fn gmt_offset_sign(&self) -> &'i str {
		if self.gmt_offset_negative { "-" } else { "+" }
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
		if self.week_day == 0 {
			panic!("invalid month for {self:?}");
		}
		MONTH_NAMES[((self.month - 1) % U8_MONTHS_IN_YEAR) as usize]
	}

	/// Resolves whether the year is a leap year ([`true`]) or common ([`false`]).
	pub fn is_leap_year(&self) -> bool {
		(self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
	}

	/// Validates a [`TimestampParts`] for correctness.
	fn validate(&self) -> Result<(), &'i str> {
		if self.nanoseconds >= U32_NANOS_IN_MILLI {
			return Err("invalid nanoseconds field");
		}
		if self.milliseconds >= U16_MILLIS_IN_SECOND {
			return Err("invalid milliseconds field");
		}
		if self.seconds >= U8_SECONDS_IN_MINUTE {
			return Err("invalid seconds field");
		}
		if self.minutes >= U8_MINUTES_IN_HOUR {
			return Err("invalid minutes field");
		}
		if self.hour >= U8_HOURS_IN_DAY {
			return Err("invalid hour field");
		}
		if self.month < 1 || self.month >= U8_MONTHS_IN_YEAR {
			return Err("invalid month field");
		}
		if self.year < U16_UNIX_EPOCH_YEAR {
			return Err("invalid year field");
		}
		if self.month_day < 1 || self.month_day > (if self.is_leap_year() { DAYS_IN_MONTH_LEAP_YEAR } else { DAYS_IN_MONTH_COMMON_YEAR })[(self.month - 1) as usize] {
			return Err("invalid month_day field");
		};
		if self.week_day < 1 || self.week_day > U8_DAYS_IN_WEEK {
			return Err("invalid week_day field");
		}
		if self.year_day < 1 || self.year_day > (if self.is_leap_year() { U16_DAYS_IN_LEAP_YEAR } else { U16_DAYS_IN_COMMON_YEAR }) {
			return Err("invalid year_day field");
		}
		// no checks for self.gmt_offset_negative
		if self.gmt_offset_hours >= U8_HOURS_IN_DAY {
			return Err("invalid gmt_offset_hours field");
		}
		if self.gmt_offset_minutes > U8_MINUTES_IN_HOUR {
			return Err("invalid gmt_offset_minutes field");
		}
		// no checks for self.timezone

		Ok(())
	}

	/// Converts the parts structure back into a [`Timestamp`], interpreting it as UTC.
	// TODO: make this function timezone agnostic.
	pub fn utc_to_timestamp(&self) -> Timestamp {
		if self.timezone != TIMEZONE_UTC {
			panic!("cannot convert a TimestampParts in timezone `{tz}' to UTC back to Timestamp", tz = self.timezone);
		}
		if let Err(e) = self.validate() {
			panic!("{}", e);
		}

		let tm = &mut c_bindings::c_tm {
			tm_sec: self.seconds as _,
			tm_min: self.minutes as _,
			tm_hour: self.hour as _,
			tm_mday: self.month_day as _,
			tm_mon: (self.month - 1) as _,
			tm_year: (self.year - 1900) as _,
			// none of the following fields are used
			tm_wday: 0 as _,
			tm_yday: 0 as _,
			tm_isdst: 0,
			tm_gmtoff: 0,
			tm_zone: c_bindings::NULL_C_CHAR,
		};

		let secs = c_bindings::c_utc_tm_to_time(tm) as u64;
		let nanos = self.nanoseconds + ((self.milliseconds as u32) * U32_NANOS_IN_MILLI);
		super::Timestamp::new(secs, nanos)
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn gmt_offset_parts() {
		assert_eq!(TimestampParts::_gmt_offset_parts(30600), (false, 8, 30));
		assert_eq!(TimestampParts::_gmt_offset_parts(-13500), (true, 3, 45));
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
	fn validation() {
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

		// rely on the order fields are checkeds so we don't build a new TimestampParts every time
		parts.nanoseconds = 1000037;
		assert_eq!(parts.validate(), Err("invalid nanoseconds field"));
		parts.nanoseconds = 10037;

		parts.milliseconds = 2196;
		assert_eq!(parts.validate(), Err("invalid milliseconds field"));
		parts.milliseconds = 761;

		parts.seconds = 77;
		assert_eq!(parts.validate(), Err("invalid seconds field"));
		parts.seconds = 19;

		parts.minutes = 77;
		assert_eq!(parts.validate(), Err("invalid minutes field"));
		parts.minutes = 58;

		parts.hour = 24;
		assert_eq!(parts.validate(), Err("invalid hour field"));
		parts.hour = 22;

		parts.month = 0;
		assert_eq!(parts.validate(), Err("invalid month field"));
		parts.month = 15;
		assert_eq!(parts.validate(), Err("invalid month field"));
		parts.month = 2;

		parts.year = 1969;
		assert_eq!(parts.validate(), Err("invalid year field"));
		parts.year = 2026;

		parts.month_day = 0;
		assert_eq!(parts.validate(), Err("invalid month_day field"));
		parts.month_day = 44;
		assert_eq!(parts.validate(), Err("invalid month_day field"));
		// 2026 is a common year, so...
		parts.month_day = 29;
		assert_eq!(parts.validate(), Err("invalid month_day field"));
		parts.month_day = 6;

		parts.week_day = 0;
		assert_eq!(parts.validate(), Err("invalid week_day field"));
		parts.week_day = 9;
		assert_eq!(parts.validate(), Err("invalid week_day field"));
		parts.week_day = 6;

		parts.year_day = 0;
		assert_eq!(parts.validate(), Err("invalid year_day field"));
		parts.year_day = 390;
		assert_eq!(parts.validate(), Err("invalid year_day field"));
		parts.year_day = 37;

		parts.gmt_offset_hours = 25;
		assert_eq!(parts.validate(), Err("invalid gmt_offset_hours field"));
		parts.gmt_offset_hours = 3;

		parts.gmt_offset_minutes = 79;
		assert_eq!(parts.validate(), Err("invalid gmt_offset_minutes field"));
		parts.gmt_offset_minutes = 45;

		assert_eq!(parts.validate(), Ok(()));
	}

	#[test]
	fn parts_to_utc() {
		assert_eq!(
			TimestampParts {
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
			}
			.utc_to_timestamp(),
			Timestamp::from_nanos(1772947335320123456)
		);
	}

	#[test]
	fn utc_to_and_from_parts() {
		let ts = Timestamp::from_utc_date(2026, 03, 24, 18, 47, 31, 111, 222);
		let parts = ts.as_utc_parts();

		assert_eq!(
			parts,
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
			}
		);

		let from_parts: Timestamp = parts.utc_to_timestamp();
		assert_eq!(ts, from_parts);
	}
}
