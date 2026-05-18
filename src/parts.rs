use super::Timestamp;

use crate::c_bindings;
use crate::constant::{
	DAY_NAMES, DAYS_IN_MONTH_COMMON_YEAR, DAYS_IN_MONTH_LEAP_YEAR, MONTH_NAMES, MONTH_TO_DAYS_COMMON_YEAR, MONTH_TO_DAYS_LEAP_YEAR, TIMEZONE_UTC, U8_DAYS_IN_WEEK, U8_HOURS_IN_DAY, U8_MINUTES_IN_HOUR,
	U8_MONTHS_IN_YEAR, U8_SECONDS_IN_MINUTE, U16_DAYS_IN_COMMON_YEAR, U16_DAYS_IN_LEAP_YEAR, U16_MILLIS_IN_SECOND, U16_SECONDS_IN_HOUR, U16_SECONDS_IN_MINUTE, U16_UNIX_EPOCH_YEAR, U32_NANOS_IN_MILLI,
	U64_LEAP_YEARS_BEFORE_EPOCH, U64_SECONDS_IN_COMMON_YEAR, U64_SECONDS_IN_DAY, U64_SECONDS_IN_HOUR, U64_SECONDS_IN_MINUTE,
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
	/// Returns an UNIX epoch [`TimestampParts`] (1970-01-01 00:00 UTC)
	pub fn epoch() -> Self {
		TimestampParts {
			nanoseconds: 0,
			milliseconds: 0,
			seconds: 0,
			minutes: 0,
			hour: 0,
			month_day: 1,
			month: 1,
			year: U16_UNIX_EPOCH_YEAR,
			week_day: 4,
			year_day: 1,
			gmt_offset_negative: false,
			gmt_offset_hours: 0,
			gmt_offset_minutes: 0,
			timezone: TIMEZONE_UTC,
		}
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
			gmt_offset_secs = c_bindings::c_tz_gmt_offset();
			timezone = (*c_bindings::MSVC_TZ_NAME).as_str();
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

	/// Computes day of the year given a [`TimestampParts]` year, month and day of the month.
	fn year_day(&self) -> u16 {
		let month_to_days = (if self.is_leap_year() { MONTH_TO_DAYS_LEAP_YEAR } else { MONTH_TO_DAYS_COMMON_YEAR })[(self.month - 1) as usize];
		month_to_days + (self.month_day as u16)
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

	/// Resolves the total number of leap since UNIX epoch.
	pub fn leap_years_since_epoch(&self) -> u64 {
		if self.year < U16_UNIX_EPOCH_YEAR {
			return 0;
		}
		let year = self.year as u64 - 1;
		let leap_years = (year / 4) - (year / 100) + (year / 400);

		leap_years - U64_LEAP_YEARS_BEFORE_EPOCH
	}

	/// Validates [`TimestampParts`] individual fields for correctness.
	fn validate_fields(&self) -> Result<(), &'i str> {
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

	/// Validates [`TimestampParts`] for correctness.
	pub fn validate(&self) -> Result<(), &'i str> {
		if let Err(e) = self.validate_fields() {
			return Err(e);
		}

		// ensure the provided day of the year matches with other fields
		let expected_year_day = self.year_day();
		if self.year_day != expected_year_day {
			return Err("year_day field doesn't match year + month + month_day");
		}

		Ok(())
	}

	/// Converts the parts structure back into a [`Timestamp`], much like glibc's timegm().
	// TODO: return errors
	pub fn to_timestamp(&self) -> Result<Timestamp, &'i str> {
		if let Err(e) = self.validate_fields() {
			return Err(e);
		}

		// See https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_15 for
		// a description of this algorithm:
		//
		// seconds_since_epoch = tm_sec + tm_min*60 + tm_hour*3600 + tm_yday*86400 +
		//                       (tm_year-70)*31536000 + ((tm_year-69)/4)*86400 -
		//                       ((tm_year-1)/100)*86400 + ((tm_year+299)/400)*86400
		let tm_sec = self.seconds as u64;
		let tm_min = self.minutes as u64;
		let tm_hour = self.hour as u64;
		let tm_year = (self.year - U16_UNIX_EPOCH_YEAR) as u64;
		let tm_yday = (self.year_day() - 1) as u64;
		let leap_years = self.leap_years_since_epoch() as u64;

		// compute total seconds since epoch...
		let mut secs = tm_sec + tm_min * U64_SECONDS_IN_MINUTE + tm_hour * U64_SECONDS_IN_HOUR + tm_yday * U64_SECONDS_IN_DAY;
		secs += tm_year * U64_SECONDS_IN_COMMON_YEAR + leap_years * U64_SECONDS_IN_DAY;

		// ..and add GMT offset.
		let gmt_offset_secs = (self.gmt_offset_minutes as u64) * U64_SECONDS_IN_MINUTE + (self.gmt_offset_hours as u64) * U64_SECONDS_IN_HOUR;
		if self.gmt_offset_negative {
			secs += gmt_offset_secs
		} else {
			secs -= gmt_offset_secs
		};

		let nanos = self.nanoseconds + ((self.milliseconds as u32) * U32_NANOS_IN_MILLI);

		Ok(super::Timestamp::new(secs, nanos))
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
	fn leap_years_since_epoch() {
		let mut parts = TimestampParts::epoch();

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
	fn validation() {
		let mut parts = TimestampParts::epoch();

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
		parts.year_day = 12;
		assert_eq!(parts.validate(), Err("year_day field doesn't match year + month + month_day"));
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
		assert_eq!(parts.to_timestamp(), Ok(Timestamp::from_nanos(1772947335320123456)));

		// timezoned parts
		parts.gmt_offset_negative = true;
		parts.gmt_offset_hours = 9;
		parts.gmt_offset_minutes = 30;
		parts.timezone = "Pacific/Marquesas";
		assert_eq!(parts.to_timestamp(), Ok(Timestamp::from_nanos(1772981535320123456)));
	}

	#[test]
	fn utc_to_and_from_parts() {
		let ts = Timestamp::from_utc_date(2026, 03, 24, 18, 47, 31, 111, 222).expect("invalid parts");
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

		let from_parts: Timestamp = parts.to_timestamp().expect("invalid TimestampParts casted from Timestamp");
		assert_eq!(ts, from_parts);
	}
}
