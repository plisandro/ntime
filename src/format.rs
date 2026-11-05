use std::io;

use crate::Timestamp;

/// Supported timezones for [`Format`] settings.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormatTimezone {
	/// Local timezone.
	Local,
	/// UTC/GMT.
	UTC,
}

/// Supported precisions for [`Format`] settings.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormatPrecision {
	/// Seconds: `15`.
	Seconds,
	/// Milliseconds: `15.488`.
	Milliseconds,
	/// Nanoseconds: `15.488167982`.
	Nanoseconds,
}

/// Supported formats for [`Timestamp`] string serialization.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Format {
	/// Compact datetime.
	///
	///   * UTC: `2026-03-02 13:22:15`
	///   * UTC, with milliseconds: `2026-03-02 13:22:15.488`
	///   * UTC, with nanoseconds: `2026-03-02 13:22:15.488728341`
	///   * Local: `2026-03-02 15:22:15 +0200`
	///   * Local, with milliseconds: `2026-03-02 15:22:15.488 +0200`
	///   * Local, with nanoseconds: `2026-03-02 13:22:15.488728341 +0200`
	DateTime(FormatTimezone, FormatPrecision),

	/// Compact date, in UTC/local: `2025-03-02`
	Date(FormatTimezone),

	/// Compact time.
	///
	///   * UTC/local: `13:22:15`
	///   * UTC/local, with milliseconds: `13:22:15.488`
	///   * UTC/local, with nanoseconds: `13:22:15.488167982`
	Time(FormatTimezone, FormatPrecision),

	/// Date/time format suitable to append to filenames. in UTC/local: `2026-03-02_13-22-15`
	FileName(FormatTimezone),

	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format).
	///
	///   * UTC: `Mon, 02 Mar 2026 13:22:15 +0000`
	///   * Local: `Mon, 02 Mar 2026 15:22:15 +0200`
	RFC2822(FormatTimezone),

	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in UTC/local: `Mar  2 13:22:15`
	RFC3164(FormatTimezone),

	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF).
	///
	///   * UTC: `2026-03-02T13:22:15Z`
	///   * UTC, with milliseconds: `2026-03-02T13:22:15.488Z`
	///   * UTC, with nanoseconds: `2026-03-02T13:22:15.488167982Z`
	///   * Local: `2026-03-02T15:22:15+02:00`
	///   * Local, with milliseconds: `2026-03-02T15:22:15.488+02:00`
	///   * Local, with nanoseconds: `2026-03-02T15:22:15.488728341+02:00`
	RFC3339(FormatTimezone, FormatPrecision),

	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html).
	///
	///   * UTC: `2026-03-02T13:22:15Z`
	///   * UTC, with milliseconds: `2026-03-02T13:22:15.488Z`
	///   * UTC, with nanoseconds: `2026-03-02T13:22:15.488167982Z`
	///   * Local: `2026-03-02T15:22:15+02:00`
	///   * Local, with milliseconds: `2026-03-02T15:22:15.488+02:00`
	///   * Local, with nanoseconds:`2026-03-02T15:22:15.488728341+02:00`
	ISO8601(FormatTimezone, FormatPrecision),

	/// An alias for [`Format::RFC7231`].
	HTTP,
	/// [RFC 7231](https://www.rfc-editor.org/rfc/rfc7231.html) (HTTP/1.1), in UTC: `Mon, 02 Mar 2026 13:22:15 GMT`
	RFC7231,

	/// Timestamp, as time since Unix epoch.
	///
	///   * Seconds: `1772795501`
	///   * Milliseconds: `1772795501890`
	///   * Nanoseconds: `1772795501890546113`
	Timestamp(FormatPrecision),
}

impl Format {
	/// Returns a default formatting (['DateTime`] in [`UTC`][FormatTimezone::UTC], with [millisecond][FormatPrecision::Milliseconds] precision.
	pub fn default() -> Self {
		Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds)
	}

	/// Returns a string name for the [`Format`]
	pub fn name(&self) -> &'static str {
		match self {
			Self::DateTime(tz, pr) => match tz {
				FormatTimezone::UTC => match pr {
					FormatPrecision::Seconds => "compact datetime (UTC)",
					FormatPrecision::Milliseconds => "compact datetime with milliseconds (UTC)",
					FormatPrecision::Nanoseconds => "compact datetime with nanoseconds (UTC)",
				},
				FormatTimezone::Local => match pr {
					FormatPrecision::Seconds => "compact datetime (local)",
					FormatPrecision::Milliseconds => "compact datetime with milliseconds (local)",
					FormatPrecision::Nanoseconds => "compact datetime with nanoseconds (local)",
				},
			},
			Self::Date(tz) => match tz {
				FormatTimezone::UTC => "compact date (UTC)",
				FormatTimezone::Local => "compact date (local)",
			},
			Self::Time(tz, pr) => match tz {
				FormatTimezone::UTC => match pr {
					FormatPrecision::Seconds => "compact time (UTC)",
					FormatPrecision::Milliseconds => "compact time with milliseconds (UTC)",
					FormatPrecision::Nanoseconds => "compact time with nanoseconds (UTC)",
				},
				FormatTimezone::Local => match pr {
					FormatPrecision::Seconds => "compact time (local)",
					FormatPrecision::Milliseconds => "compact time with milliseconds (local)",
					FormatPrecision::Nanoseconds => "compact time with nanoseconds (local)",
				},
			},
			Self::FileName(tz) => match tz {
				FormatTimezone::UTC => "date+time filename (UTC)",
				FormatTimezone::Local => "date+time filename (local)",
			},
			Self::RFC2822(tz) => match tz {
				FormatTimezone::UTC => "RFC 2822 (UTC)",
				FormatTimezone::Local => "RFC 2822 (local)",
			},
			Self::RFC3339(tz, pr) => match tz {
				FormatTimezone::UTC => match pr {
					FormatPrecision::Seconds => "RFC 3339 (UTC)",
					FormatPrecision::Milliseconds => "RFC 3339 with milliseconds (UTC)",
					FormatPrecision::Nanoseconds => "RFC 3339 with nanoseconds (UTC)",
				},
				FormatTimezone::Local => match pr {
					FormatPrecision::Seconds => "RFC 3339 (local)",
					FormatPrecision::Milliseconds => "RFC 3339 with milliseconds (local)",
					FormatPrecision::Nanoseconds => "RFC 3339 with nanoseconds (local)",
				},
			},
			Self::RFC3164(tz) => match tz {
				FormatTimezone::UTC => "syslog RFC 3164 (UTC)",
				FormatTimezone::Local => "syslog RFC 3164 (local)",
			},
			Self::ISO8601(tz, pr) => match tz {
				FormatTimezone::UTC => match pr {
					FormatPrecision::Seconds => "ISO 8601 (UTC)",
					FormatPrecision::Milliseconds => "ISO 8601 with milliseconds (UTC)",
					FormatPrecision::Nanoseconds => "ISO 8601 with nanoseconds (UTC)",
				},
				FormatTimezone::Local => match pr {
					FormatPrecision::Seconds => "ISO 8601 (local)",
					FormatPrecision::Milliseconds => "ISO 8601 with milliseconds (local)",
					FormatPrecision::Nanoseconds => "ISO 8601 with nanoseconds (local)",
				},
			},

			Self::HTTP | Self::RFC7231 => "RFC 7231 (GMT)",

			Self::Timestamp(pr) => match pr {
				FormatPrecision::Seconds => "seconds since epoch",
				FormatPrecision::Milliseconds => "milliseconds since epoch",
				FormatPrecision::Nanoseconds => "nanoseconds since epoch",
			},
		}
	}

	/// Evaluates if the given [`Format`] is in UTC timezone.
	pub fn is_utc(&self) -> bool {
		match self {
			Self::DateTime(tz, _) => tz == &FormatTimezone::UTC,
			Self::Date(tz) => tz == &FormatTimezone::UTC,
			Self::Time(tz, _) => tz == &FormatTimezone::UTC,
			Self::FileName(tz) => tz == &FormatTimezone::UTC,
			Self::RFC2822(tz) => tz == &FormatTimezone::UTC,
			Self::RFC3339(tz, _) => tz == &FormatTimezone::UTC,
			Self::RFC3164(tz) => tz == &FormatTimezone::UTC,
			Self::ISO8601(tz, _) => tz == &FormatTimezone::UTC,
			Self::HTTP | Self::RFC7231 => true,
			Self::Timestamp(_) => true,
		}
	}

	/// Serializes a [`Timestamp`] as string, into a given [`std::io::Write`].
	pub fn write<T: io::Write>(&self, out: &mut T, ts: &Timestamp) -> io::Result<()> {
		match self {
			Self::DateTime(tz, pr) => match tz {
				FormatTimezone::UTC => {
					let parts = ts.as_utc_parts();
					match pr {
						FormatPrecision::Seconds => write!(
							out,
							"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}",
							year = parts.year,
							month = parts.month,
							day = parts.month_day,
							hour = parts.hour,
							mins = parts.minutes,
							secs = parts.seconds,
						),
						FormatPrecision::Milliseconds => write!(
							out,
							"{year}-{month:02}-{day:02} {hour:02}:{mins:02}:{secs:02}.{msecs:03}",
							year = parts.year,
							month = parts.month,
							day = parts.month_day,
							hour = parts.hour,
							mins = parts.minutes,
							secs = parts.seconds,
							msecs = parts.milliseconds,
						),
						FormatPrecision::Nanoseconds => write!(
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
						),
					}
				}
				FormatTimezone::Local => {
					let parts = ts.as_local_parts();
					match pr {
						FormatPrecision::Seconds => write!(
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
						),
						FormatPrecision::Milliseconds => write!(
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
						),
						FormatPrecision::Nanoseconds => write!(
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
						),
					}
				}
			},
			Format::FileName(tz) => {
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};
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
			Format::Date(tz) => {
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};
				write!(out, "{year}-{month:02}-{day:02}", year = parts.year, month = parts.month, day = parts.month_day)
			}
			Self::Time(tz, pr) => {
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};
				match pr {
					FormatPrecision::Seconds => write!(out, "{hour:02}:{mins:02}:{secs:02}", hour = parts.hour, mins = parts.minutes, secs = parts.seconds),
					FormatPrecision::Milliseconds => write!(
						out,
						"{hour:02}:{mins:02}:{secs:02}.{msecs:03}",
						hour = parts.hour,
						mins = parts.minutes,
						secs = parts.seconds,
						msecs = parts.milliseconds,
					),
					FormatPrecision::Nanoseconds => write!(
						out,
						"{hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06}",
						hour = parts.hour,
						mins = parts.minutes,
						secs = parts.seconds,
						msecs = parts.milliseconds,
						nsecs = parts.nanoseconds,
					),
				}
			}
			Self::RFC2822(tz) => {
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};
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
			Self::RFC3339(tz, pr) | Self::ISO8601(tz, pr) => match tz {
				FormatTimezone::UTC => {
					let parts = ts.as_utc_parts();
					match pr {
						FormatPrecision::Seconds => write!(
							out,
							"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}Z",
							year = parts.year,
							month = parts.month,
							day = parts.month_day,
							hour = parts.hour,
							mins = parts.minutes,
							secs = parts.seconds,
						),
						FormatPrecision::Milliseconds => write!(
							out,
							"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}Z",
							year = parts.year,
							month = parts.month,
							day = parts.month_day,
							hour = parts.hour,
							mins = parts.minutes,
							secs = parts.seconds,
							msecs = parts.milliseconds,
						),
						FormatPrecision::Nanoseconds => write!(
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
						),
					}
				}
				FormatTimezone::Local => {
					let parts = ts.as_local_parts();
					match pr {
						FormatPrecision::Seconds => write!(
							out,
							"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}{offset_sign}{offset_hours:02}:{offset_minutes:02}",
							year = parts.year,
							month = parts.month,
							day = parts.month_day,
							hour = parts.hour,
							mins = parts.minutes,
							secs = parts.seconds,
							offset_sign = parts.gmt_offset_sign(),
							offset_hours = parts.gmt_offset_hours,
							offset_minutes = parts.gmt_offset_minutes,
						),
						FormatPrecision::Milliseconds => write!(
							out,
							"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}{offset_sign}{offset_hours:02}:{offset_minutes:02}",
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
						),
						FormatPrecision::Nanoseconds => write!(
							out,
							"{year}-{month:02}-{day:02}T{hour:02}:{mins:02}:{secs:02}.{msecs:03}{nsecs:06}{offset_sign}{offset_hours:02}:{offset_minutes:02}",
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
						),
					}
				}
			},
			Self::RFC3164(tz) => {
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};
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
			Format::HTTP | Format::RFC7231 => {
				let parts = ts.as_utc_parts();
				write!(
					out,
					"{day_name}, {day:02} {month_name} {year} {hour:02}:{mins:02}:{secs:02} GMT",
					day_name = parts.day_name(),
					day = parts.month_day,
					month_name = parts.month_name(),
					year = parts.year,
					hour = parts.hour,
					mins = parts.minutes,
					secs = parts.seconds,
				)
			}
			Self::Timestamp(ps) => match ps {
				FormatPrecision::Seconds => write!(out, "{}", ts.as_secs()),
				FormatPrecision::Milliseconds => write!(out, "{}", ts.as_millis()),
				FormatPrecision::Nanoseconds => write!(out, "{}", ts.as_nanos()),
			},
		}
	}

	/// Returns the string representation length for a [`Timestamp`] using this [`Format`]
	pub fn string_len(&self, ts: &Timestamp) -> usize {
		let count_digits = |n: u128| -> usize {
			let mut n = n;
			let mut res: usize = 1;

			n /= 10;
			while n != 0 {
				res += 1;
				n /= 10;
			}

			res
		};

		match self {
			// fixed-size formats
			Self::DateTime(tz, pr) => match tz {
				FormatTimezone::UTC => match pr {
					// 2026-03-02 13:22:15
					FormatPrecision::Seconds => 19,
					// 2026-03-02 15:22:15.488
					FormatPrecision::Milliseconds => 23,
					// 2026-03-02 13:22:15.488728341
					FormatPrecision::Nanoseconds => 29,
				},
				FormatTimezone::Local => match pr {
					// 2026-03-02 15:22:15 +0200
					FormatPrecision::Seconds => 25,
					// 2026-03-02 15:22:15.488 +0200
					FormatPrecision::Milliseconds => 29,
					// 2026-03-02 13:22:15.488728341 +0200
					FormatPrecision::Nanoseconds => 35,
				},
			},
			// 2026-03-02_15-22-15
			Self::FileName(_) => 19,
			// 2025-03-02
			Self::Date(_) => 10,
			Self::Time(_, pr) => match pr {
				// 15:22:15
				FormatPrecision::Seconds => 8,
				// 15:22:15.488
				FormatPrecision::Milliseconds => 12,
				// 15:22:15.488167982
				FormatPrecision::Nanoseconds => 18,
			},
			// Mon, 02 Mar 2026 15:22:15 +0200
			Self::RFC2822(_) => 31,
			Self::RFC3339(tz, pr) | Self::ISO8601(tz, pr) => match tz {
				FormatTimezone::UTC => match pr {
					// 2026-03-02T13:22:15Z
					FormatPrecision::Seconds => 20,
					// 2026-03-02T13:22:15.488Z
					FormatPrecision::Milliseconds => 24,
					// 2026-03-02T13:22:15.488167982Z
					FormatPrecision::Nanoseconds => 30,
				},
				FormatTimezone::Local => match pr {
					// 2026-03-02T15:22:15+02:00
					FormatPrecision::Seconds => 25,
					// 2026-03-02T15:22:15.488+02:00
					FormatPrecision::Milliseconds => 29,
					// 2026-03-02T15:22:15.488728341+02:00
					FormatPrecision::Nanoseconds => 35,
				},
			},
			// Mon, 02 Mar 2026 15:22:15 CET
			Self::RFC3164(_) => 15,
			// Mon, 02 Mar 2026 15:22:15 GMT
			Self::HTTP | Self::RFC7231 => 29,

			// variable length formats

			// an arbitrary sequence of digits
			Self::Timestamp(pr) => match pr {
				FormatPrecision::Seconds => count_digits(ts.as_secs() as u128),
				FormatPrecision::Milliseconds => count_digits(ts.as_millis()),
				FormatPrecision::Nanoseconds => count_digits(ts.as_nanos()),
			},
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
			Format::Timestamp(pr) => match pr {
				FormatPrecision::Seconds => Some(ts.as_secs() as u128),
				FormatPrecision::Milliseconds => Some(ts.as_millis()),
				FormatPrecision::Nanoseconds => Some(ts.as_nanos()),
			},
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

		assert_eq!(Format::Timestamp(FormatPrecision::Seconds).as_string(&ts), "1772808229");
		assert_eq!(Format::Timestamp(FormatPrecision::Milliseconds).as_string(&ts), "1772808229038");
		assert_eq!(Format::Timestamp(FormatPrecision::Nanoseconds).as_string(&ts), "1772808229038023456");
	}

	#[test]
	fn timestamp_as_utc_string() {
		let ts = Timestamp::from_utc_date(2026, 03, 06, 14, 43, 49, 038, 23456).expect("invalid parts");

		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds).as_string(&ts), "2026-03-06 14:43:49");
		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds).as_string(&ts), "2026-03-06 14:43:49.038");
		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds).as_string(&ts), "2026-03-06 14:43:49.038023456");
		assert_eq!(Format::FileName(FormatTimezone::UTC).as_string(&ts), "2026-03-06_14-43-49");
		assert_eq!(Format::Date(FormatTimezone::UTC).as_string(&ts), "2026-03-06");
		assert_eq!(Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds).as_string(&ts), "14:43:49");
		assert_eq!(Format::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds).as_string(&ts), "14:43:49.038");
		assert_eq!(Format::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds).as_string(&ts), "14:43:49.038023456");
		assert_eq!(Format::RFC2822(FormatTimezone::UTC).as_string(&ts), "Fri, 06 Mar 2026 14:43:49 +0000");
		assert_eq!(Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds).as_string(&ts), "2026-03-06T14:43:49Z");
		assert_eq!(Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds).as_string(&ts), "2026-03-06T14:43:49.038Z");
		assert_eq!(Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds).as_string(&ts), "2026-03-06T14:43:49.038023456Z");
		assert_eq!(Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds).as_string(&ts), "2026-03-06T14:43:49Z");
		assert_eq!(Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds).as_string(&ts), "2026-03-06T14:43:49.038Z");
		assert_eq!(Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds).as_string(&ts), "2026-03-06T14:43:49.038023456Z");
		assert_eq!(Format::RFC3164(FormatTimezone::UTC).as_string(&ts), "Mar  6 14:43:49");
		assert_eq!(Format::HTTP.as_string(&ts), "Fri, 06 Mar 2026 14:43:49 GMT");
		assert_eq!(Format::RFC7231.as_string(&ts), "Fri, 06 Mar 2026 14:43:49 GMT");
	}

	#[test]
	fn timestamp_as_local_string() {
		test_helpers::mocks::with_timezone("America/Montevideo", || {
			let ts = Timestamp::from_utc_date(2026, 03, 06, 14, 43, 49, 038, 23456).expect("invalid parts");

			assert_eq!(Format::DateTime(FormatTimezone::Local, FormatPrecision::Seconds).as_string(&ts), "2026-03-06 11:43:49 -0300");
			assert_eq!(Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds).as_string(&ts), "2026-03-06 11:43:49.038 -0300");
			assert_eq!(
				Format::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds).as_string(&ts),
				"2026-03-06 11:43:49.038023456 -0300"
			);
			assert_eq!(Format::FileName(FormatTimezone::Local).as_string(&ts), "2026-03-06_11-43-49");
			assert_eq!(Format::Date(FormatTimezone::Local).as_string(&ts), "2026-03-06");
			assert_eq!(Format::Time(FormatTimezone::Local, FormatPrecision::Seconds).as_string(&ts), "11:43:49");
			assert_eq!(Format::Time(FormatTimezone::Local, FormatPrecision::Milliseconds).as_string(&ts), "11:43:49.038");
			assert_eq!(Format::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds).as_string(&ts), "11:43:49.038023456");
			assert_eq!(Format::RFC2822(FormatTimezone::Local).as_string(&ts), "Fri, 06 Mar 2026 11:43:49 -0300");
			assert_eq!(Format::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds).as_string(&ts), "2026-03-06T11:43:49-03:00");
			assert_eq!(Format::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds).as_string(&ts), "2026-03-06T11:43:49.038-03:00");
			assert_eq!(
				Format::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds).as_string(&ts),
				"2026-03-06T11:43:49.038023456-03:00"
			);
			assert_eq!(Format::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds).as_string(&ts), "2026-03-06T11:43:49-03:00");
			assert_eq!(Format::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds).as_string(&ts), "2026-03-06T11:43:49.038-03:00");
			assert_eq!(
				Format::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds).as_string(&ts),
				"2026-03-06T11:43:49.038023456-03:00"
			);
			assert_eq!(Format::RFC3164(FormatTimezone::Local).as_string(&ts), "Mar  6 11:43:49");
		});
	}

	#[test]
	fn timestamp_string_len() {
		test_helpers::mocks::with_timezone("America/Montevideo", || {
			let ts = Timestamp::from_utc_date(2026, 03, 06, 14, 43, 49, 038, 23456).expect("invalid parts");

			for format in [
				Format::Date(FormatTimezone::Local),
				Format::Date(FormatTimezone::UTC),
				Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds),
				Format::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds),
				Format::DateTime(FormatTimezone::Local, FormatPrecision::Seconds),
				Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds),
				Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
				Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds),
				Format::FileName(FormatTimezone::Local),
				Format::FileName(FormatTimezone::UTC),
				Format::HTTP,
				Format::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds),
				Format::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds),
				Format::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds),
				Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds),
				Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
				Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds),
				Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds),
				Format::RFC2822(FormatTimezone::Local),
				Format::RFC2822(FormatTimezone::UTC),
				Format::RFC3164(FormatTimezone::UTC),
				Format::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds),
				Format::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds),
				Format::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds),
				Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds),
				Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
				Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds),
				Format::RFC7231,
				Format::Time(FormatTimezone::Local, FormatPrecision::Milliseconds),
				Format::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds),
				Format::Time(FormatTimezone::Local, FormatPrecision::Seconds),
				Format::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds),
				Format::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
				Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds),
				Format::Timestamp(FormatPrecision::Milliseconds),
				Format::Timestamp(FormatPrecision::Nanoseconds),
				Format::Timestamp(FormatPrecision::Seconds),
			] {
				assert_eq!(format.string_len(&ts), format.as_string(&ts).len());
			}
		})
	}

	#[test]
	fn timestamp_as_integer() {
		let ts = Timestamp::from_utc_date(2026, 01, 29, 07, 43, 19, 134, 943903).expect("invalid parts");

		assert_eq!(Format::Timestamp(FormatPrecision::Seconds).as_integer(&ts), Some(1769672599 as u128));
		assert_eq!(Format::Timestamp(FormatPrecision::Milliseconds).as_integer(&ts), Some(1769672599134 as u128));
		assert_eq!(Format::Timestamp(FormatPrecision::Nanoseconds).as_integer(&ts), Some(1769672599134943903 as u128));
		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds).as_integer(&ts), None);
		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds).as_integer(&ts), None);
		assert_eq!(Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds).as_integer(&ts), None);
		assert_eq!(Format::RFC2822(FormatTimezone::UTC).as_integer(&ts), None);
		assert_eq!(Format::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds).as_integer(&ts), None);
		assert_eq!(Format::HTTP.as_integer(&ts), None);
		assert_eq!(Format::RFC7231.as_integer(&ts), None);
	}

	#[test]
	fn fomat_is_integer() {
		assert_eq!(Format::Timestamp(FormatPrecision::Seconds).is_integer(), true);
		assert_eq!(Format::Timestamp(FormatPrecision::Milliseconds).is_integer(), true);
		assert_eq!(Format::Timestamp(FormatPrecision::Nanoseconds).is_integer(), true);
		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds).is_integer(), false);
		assert_eq!(Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds).is_integer(), false);
		assert_eq!(Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds).is_integer(), false);
		assert_eq!(Format::RFC2822(FormatTimezone::UTC).is_integer(), false);
		assert_eq!(Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds).is_integer(), false);
		assert_eq!(Format::HTTP.is_integer(), false);
		assert_eq!(Format::RFC7231.is_integer(), false);
	}
}
