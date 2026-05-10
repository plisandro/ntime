use std::io;

use crate::Timestamp;

/// Defines a format for [`Timestamp`] string serialization.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Format {
	/// Compact datetime, in UTC: `2026-03-02 13:22:15`
	UtcDateTime,
	/// Compact datetime with milliseconds, in UTC: `2026-03-02 13:22:15.488`
	UtcMillisDateTime,
	/// Compact datetime with nanoseconds, in UTC: `2026-03-02 13:22:15.488728341`
	UtcNanosDateTime,
	/// Compact date, in UTC: `2025-03-02`
	UtcDate,
	/// Compact time, in UTC: `13:22:15`
	UtcTime,
	/// Compact time with milliseconds, in UTC: `13:22:15.488`
	UtcMillisTime,
	/// Compact time with nanoseconds, in UTC: `13:22:15.488167982`
	UtcNanosTime,
	/// Date/time format suitable to append to filenames. in UTC: `2026-03-02_13-22-15`
	UtcFileName,
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in UTC: `Mon, 02 Mar 2026 13:22:15 +0000`
	UtcRFC2822,
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in UTC: `2026-03-02T13:22:15Z`
	UtcRFC3339,
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	UtcMillisRFC3339,
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	UtcNanosRFC3339,
	/// An alias for [`Format::UtcRFC7231`].
	UtcHTTP,
	/// [RFC 7231](https://www.rfc-editor.org/rfc/rfc3339.html) (HTTP/1.1), in UTC: `Mon, 02 Mar 2026 13:22:15 UTC`
	UtcRFC7231,
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3339.html), in UTC: `Mar  2 13:22:15`.
	UtcRFC3164,

	/// Compact datetime, in local timezone: `2026-03-02 15:22:15 +0200`
	LocalDateTime,
	/// Compact datetime with milliseconds, in local timezone: `2026-03-02 15:22:15.488 +0200`
	LocalMillisDateTime,
	/// Compact datetime with nanoseconds, in local timezone: `2026-03-02 13:22:15.488728341 +0200`
	LocalNanosDateTime,
	/// Compact date, in local timezone: `2025-03-02`
	LocalDate,
	/// Compact time, in local timezone: `15:22:15`
	LocalTime,
	/// Compact time with milliseconds, in local timezone: `15:22:15.488`
	LocalMillisTime,
	/// Compact time with nanoseconds, in local timezone: `15:22:15.488167982`
	LocalNanosTime,
	/// Date/time format suitable to append to filenames. in local timezone: `2026-03-02_15-22-15`
	LocalFileName,
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in local timezone: `Mon, 02 Mar 2026 15:22:15 +0200`
	LocalRFC2822,
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in local timezone: `2026-03-02T15:22:15+0200`
	LocalRFC3339,
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+0200`
	LocalMillisRFC3339,
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488728341+0200`
	LocalNanosRFC3339,
	/// An alias for [`Format::LocalRFC7231`].
	LocalHTTP,
	/// [RFC 7231](https://www.rfc-editor.org/rfc/rfc3339.html) (HTTP/1.1), in local timezone: `Mon, 02 Mar 2026 15:22:15 CET`
	LocalRFC7231,
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3339.html), in local timezone: `Mar  2 15:22:15`.
	LocalRFC3164,

	/// Seconds since UNIX epoch: `1772795501`
	TimestampSeconds,
	/// Milliseconds since UNIX epoch: `1772795501890`
	TimestampMilliseconds, //
	/// Nanoseconds since UNIX epoch: `1772795501890546`
	TimestampNanoseconds,
}

impl Format {
	/// Evaluates if the given [`Format`] is in UTC timezone.
	pub fn is_utc(&self) -> bool {
		match &self {
			Self::UtcDateTime => true,
			Self::UtcMillisDateTime => true,
			Self::UtcNanosDateTime => true,
			Self::UtcTime => true,
			Self::UtcMillisTime => true,
			Self::UtcNanosTime => true,
			Self::UtcFileName => true,
			Self::UtcRFC2822 => true,
			Self::UtcRFC3339 => true,
			Self::UtcMillisRFC3339 => true,
			Self::UtcNanosRFC3339 => true,
			Self::UtcHTTP => true,
			Self::UtcRFC7231 => true,
			Self::UtcRFC3164 => true,
			Self::TimestampSeconds => true,
			Self::TimestampMilliseconds => true,
			_ => false,
		}
	}

	/// Serializes a [`Timestamp`] as string, into a given [`std::io::Write`].
	pub fn write<T: io::Write>(&self, out: &mut T, ts: &Timestamp) -> io::Result<()> {
		let get_parts = || {
			if self.is_utc() { ts.as_utc_parts() } else { ts.as_local_parts() }
		};

		match self {
			Format::UtcDateTime => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
				)
			}
			Format::LocalDateTime => {
				let parts = ts.as_local_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02} {offset_sign}{offset_hours:02}{offset_minutes:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::UtcMillisDateTime => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}.{msecs:03}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
				)
			}
			Format::LocalMillisDateTime => {
				let parts = ts.as_local_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}.{msecs:03} {offset_sign}{offset_hours:02}{offset_minutes:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::UtcNanosDateTime => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					nsecs = parts.nanoseconds,
				)
			}
			Format::LocalNanosDateTime => {
				let parts = ts.as_local_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06} {offset_sign}{offset_hours:02}{offset_minutes:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					nsecs = parts.nanoseconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::UtcFileName | Format::LocalFileName => {
				let parts = get_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}_{hour:02}-{mins:02}-{secs:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
				)
			}
			Format::UtcDate | Format::LocalDate => {
				let parts = get_parts();
				write!(out, "{year}-{month:02}-{day:02}", year = parts.year, month = parts.month, day = parts.month_day)
			}
			Format::UtcTime | Format::LocalTime => {
				let parts = get_parts();
				write!(out, "{hour:02}:{mins:02}:{secs:02}", hour = parts.hour, mins = parts.minutes, secs = parts.seconds)
			}
			Format::UtcMillisTime | Format::LocalMillisTime => {
				let parts = get_parts();
				write!(
					out,
					"{hour:02}:{mins:02}:{secs:02}.{msecs:03}",
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
				)
			}
			Format::UtcNanosTime | Format::LocalNanosTime => {
				let parts = get_parts();
				write!(
					out,
					"{hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06}",
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					nsecs = parts.nanoseconds,
				)
			}
			Format::UtcRFC2822 | Format::LocalRFC2822 => {
				let parts = get_parts();
				write!(
					out,
					"{day_name}, {day:02} {month_name} {year} {hour:02}:{mins:02}:{secs:02} {offset_sign}{offset_hours:02}{offset_minutes:02}",
					day_name = parts.day_name(),
					day = parts.month_day,
					month_name = parts.month_name(),
					year = parts.year,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::UtcRFC3339 => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}Z",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
				)
			}
			Format::UtcMillisRFC3339 => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}Z",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
				)
			}
			Format::UtcNanosRFC3339 => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06}Z",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					nsecs = parts.nanoseconds,
				)
			}
			Format::LocalRFC3339 => {
				let parts = ts.as_local_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}{offset_sign}{offset_hours:02}{offset_minutes:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::LocalMillisRFC3339 => {
				let parts = ts.as_local_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}{offset_sign}{offset_hours:02}{offset_minutes:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::LocalNanosRFC3339 => {
				let parts = ts.as_local_parts();
				write!(
					out,
					"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06}{offset_sign}{offset_hours:02}{offset_minutes:02}",
					year = parts.year,
					month = parts.month,
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					msecs = parts.milliseconds,
					nsecs = parts.nanoseconds,
					offset_sign = parts.gmt_offset_sign(),
					offset_hours = parts.gmt_offset_hours,
					offset_minutes = parts.gmt_offset_minutes,
				)
			}
			Format::UtcHTTP | Format::UtcRFC7231 | Format::LocalHTTP | Format::LocalRFC7231 => {
				let parts = get_parts();
				write!(
					out,
					"{day_name}, {day:02} {month_name} {year} {hour:02}:{mins:02}:{secs:02} {timezone}",
					day_name = parts.day_name(),
					day = parts.month_day,
					month_name = parts.month_name(),
					year = parts.year,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
					timezone = parts.timezone,
				)
			}
			Format::UtcRFC3164 | Format::LocalRFC3164 => {
				let parts = get_parts();
				write!(
					out,
					"{month_name} {day:>2} {hour:02}:{mins:02}:{secs:02}",
					month_name = parts.month_name(),
					day = parts.month_day,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
				)
			}
			Format::TimestampSeconds => write!(out, "{}", ts.as_secs()),
			Format::TimestampMilliseconds => write!(out, "{}", ts.as_millis()),
			Format::TimestampNanoseconds => write!(out, "{}", ts.as_nanos()),
		}
	}

	/// Serializes a [`Timestamp`] into a [`String`].
	pub fn as_string(&self, ts: &Timestamp) -> String {
		let mut out = Vec::new();
		if let Err(e) = self.write(&mut out, ts) {
			panic!("failed to serialize Timestamp: {}", e);
		}

		match String::from_utf8(out) {
			Ok(s) => s,
			Err(e) => panic!("failed to convert Timestamp to String: {}", e),
		}
	}

	/// Serializes a [`Timestamp`] into am integer, if the format supports it.
	pub fn as_integer(&self, ts: &Timestamp) -> Option<u128> {
		match self {
			Format::TimestampSeconds => Some(ts.as_secs() as u128),
			Format::TimestampMilliseconds => Some(ts.as_millis()),
			Format::TimestampNanoseconds => Some(ts.as_nanos()),
			_ => None,
		}
	}

	/// Evaluates whether this [`Format`] is an numeric integer.
	pub fn is_integer(&self) -> bool {
		match self.as_integer(&Timestamp::epoch()) {
			Some(_) => true,
			None => false,
		}
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod test_format {
	use super::*;
	use crate::test_helpers;

	#[test]
	fn timestamp_as_number_string() {
		let ts = Timestamp::from_utc_date(2026, 03, 06, 14, 43, 49, 038, 23456).expect("invalid parts");

		assert_eq!(Format::TimestampSeconds.as_string(&ts), "1772808229");
		assert_eq!(Format::TimestampMilliseconds.as_string(&ts), "1772808229038");
		assert_eq!(Format::TimestampNanoseconds.as_string(&ts), "1772808229038023456");
	}

	#[test]
	fn timestamp_as_utc_string() {
		let ts = Timestamp::from_utc_date(2026, 03, 06, 14, 43, 49, 038, 23456).expect("invalid parts");

		assert_eq!(Format::UtcDateTime.as_string(&ts), "2026-03-06 14:43:49");
		assert_eq!(Format::UtcMillisDateTime.as_string(&ts), "2026-03-06 14:43:49.038");
		assert_eq!(Format::UtcNanosDateTime.as_string(&ts), "2026-03-06 14:43:49.038023456");
		assert_eq!(Format::UtcFileName.as_string(&ts), "2026-03-06_14-43-49");
		assert_eq!(Format::UtcDate.as_string(&ts), "2026-03-06");
		assert_eq!(Format::UtcTime.as_string(&ts), "14:43:49");
		assert_eq!(Format::UtcMillisTime.as_string(&ts), "14:43:49.038");
		assert_eq!(Format::UtcNanosTime.as_string(&ts), "14:43:49.038023456");
		assert_eq!(Format::UtcRFC2822.as_string(&ts), "Fri, 06 Mar 2026 14:43:49 +0000");
		assert_eq!(Format::UtcRFC3339.as_string(&ts), "2026-03-06T14:43:49Z");
		assert_eq!(Format::UtcMillisRFC3339.as_string(&ts), "2026-03-06T14:43:49.038Z");
		assert_eq!(Format::UtcNanosRFC3339.as_string(&ts), "2026-03-06T14:43:49.038023456Z");
		assert_eq!(Format::UtcHTTP.as_string(&ts), "Fri, 06 Mar 2026 14:43:49 UTC");
		assert_eq!(Format::UtcRFC7231.as_string(&ts), "Fri, 06 Mar 2026 14:43:49 UTC");
		assert_eq!(Format::UtcRFC3164.as_string(&ts), "Mar  6 14:43:49");
	}

	#[test]
	fn timestamp_as_local_string() {
		test_helpers::mocks::with_timezone("America/Montevideo", || {
			let ts = Timestamp::from_utc_date(2026, 03, 06, 14, 43, 49, 038, 23456).expect("invalid parts");

			assert_eq!(Format::LocalDateTime.as_string(&ts), "2026-03-06 11:43:49 -0300");
			assert_eq!(Format::LocalMillisDateTime.as_string(&ts), "2026-03-06 11:43:49.038 -0300");
			assert_eq!(Format::LocalNanosDateTime.as_string(&ts), "2026-03-06 11:43:49.038023456 -0300");
			assert_eq!(Format::LocalFileName.as_string(&ts), "2026-03-06_11-43-49");
			assert_eq!(Format::LocalDate.as_string(&ts), "2026-03-06");
			assert_eq!(Format::LocalTime.as_string(&ts), "11:43:49");
			assert_eq!(Format::LocalMillisTime.as_string(&ts), "11:43:49.038");
			assert_eq!(Format::LocalNanosTime.as_string(&ts), "11:43:49.038023456");
			assert_eq!(Format::LocalRFC2822.as_string(&ts), "Fri, 06 Mar 2026 11:43:49 -0300");
			assert_eq!(Format::LocalRFC3339.as_string(&ts), "2026-03-06T11:43:49-0300");
			assert_eq!(Format::LocalMillisRFC3339.as_string(&ts), "2026-03-06T11:43:49.038-0300");
			assert_eq!(Format::LocalNanosRFC3339.as_string(&ts), "2026-03-06T11:43:49.038023456-0300");
			assert_eq!(Format::LocalHTTP.as_string(&ts), "Fri, 06 Mar 2026 11:43:49 -03");
			assert_eq!(Format::LocalRFC7231.as_string(&ts), "Fri, 06 Mar 2026 11:43:49 -03");
			assert_eq!(Format::LocalRFC3164.as_string(&ts), "Mar  6 11:43:49");
		});
	}

	#[test]
	fn timestamp_as_integer() {
		let ts = Timestamp::from_utc_date(2026, 01, 29, 07, 43, 19, 134, 943903).expect("invalid parts");

		assert_eq!(Format::TimestampSeconds.as_integer(&ts), Some(1769672599 as u128));
		assert_eq!(Format::TimestampMilliseconds.as_integer(&ts), Some(1769672599134 as u128));
		assert_eq!(Format::TimestampNanoseconds.as_integer(&ts), Some(1769672599134943903 as u128));
		assert_eq!(Format::UtcDateTime.as_integer(&ts), None);
		assert_eq!(Format::UtcNanosDateTime.as_integer(&ts), None);
		assert_eq!(Format::UtcTime.as_integer(&ts), None);
		assert_eq!(Format::UtcRFC2822.as_integer(&ts), None);
		assert_eq!(Format::UtcRFC7231.as_integer(&ts), None);
	}

	#[test]
	fn fomat_is_integer() {
		assert_eq!(Format::TimestampSeconds.is_integer(), true);
		assert_eq!(Format::TimestampMilliseconds.is_integer(), true);
		assert_eq!(Format::TimestampNanoseconds.is_integer(), true);
		assert_eq!(Format::UtcDateTime.is_integer(), false);
		assert_eq!(Format::UtcNanosDateTime.is_integer(), false);
		assert_eq!(Format::UtcTime.is_integer(), false);
		assert_eq!(Format::UtcRFC2822.is_integer(), false);
		assert_eq!(Format::UtcRFC7231.is_integer(), false);
	}
}
