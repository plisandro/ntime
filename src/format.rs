use std::io;

use crate::Timestamp;

/// Supported timezones for [`Format`] settings.
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[non_exhaustive]
pub enum FormatTimezone {
	/// Local timezone.
	Local,
	/// UTC/GMT.
	UTC,
}

/// Supported precisions for [`Format`] settings.
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[non_exhaustive]
pub enum FormatPrecision {
	/// Seconds: `15`.
	Seconds,
	/// Milliseconds: `15.488`.
	Milliseconds,
	/// Nanoseconds: `15.488167982`.
	Nanoseconds,
}

/// Supported formats for [`Timestamp`] string serialization.
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[non_exhaustive]
pub enum Format {
	/// Compact datetime.
	///
	///   * [UTC][`FormatTimezone::UTC`], in [seconds][`FormatPrecision::Seconds`]: `2026-03-02 13:22:15`
	///   * [UTC][`FormatTimezone::UTC`], with [milliseconds][`FormatPrecision::Milliseconds`]: `2026-03-02 13:22:15.488`
	///   * [UTC][`FormatTimezone::UTC`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `2026-03-02 13:22:15.488167982`
	///   * [Local][`FormatTimezone::Local`], in [seconds][`FormatPrecision::Seconds`]: `2026-03-02 15:22:15 +0200`
	///   * [Local][`FormatTimezone::Local`], with [milliseconds][`FormatPrecision::Milliseconds`]: `2026-03-02 15:22:15.488 +0200`
	///   * [Local][`FormatTimezone::Local`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `2026-03-02 15:22:15.488167982 +0200`
	DateTime(FormatTimezone, FormatPrecision),

	/// Compact date.
	///
	///   * [UTC][`FormatTimezone::UTC`]: `2026-03-02`
	///   * [Local][`FormatTimezone::Local`]: `2026-03-02`
	Date(FormatTimezone),

	/// Compact time.
	///
	///   * [UTC][`FormatTimezone::UTC`], in [seconds][`FormatPrecision::Seconds`]: `13:22:15`
	///   * [UTC][`FormatTimezone::UTC`], with [milliseconds][`FormatPrecision::Milliseconds`]: `13:22:15.488`
	///   * [UTC][`FormatTimezone::UTC`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `13:22:15.488167982`
	///   * [Local][`FormatTimezone::Local`], in [seconds][`FormatPrecision::Seconds`]: `15:22:15`
	///   * [Local][`FormatTimezone::Local`], with [milliseconds][`FormatPrecision::Milliseconds`]: `15:22:15.488`
	///   * [Local][`FormatTimezone::Local`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `15:22:15.488167982`
	Time(FormatTimezone, FormatPrecision),

	/// Date/time format suitable to append to filenames.
	///
	///   * [UTC][`FormatTimezone::UTC`]: `2026-03-02_13-22-15`
	///   * [Local][`FormatTimezone::Local`]: `2026-03-02_15-22-15`
	FileName(FormatTimezone),

	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format).
	///
	///   * [UTC][`FormatTimezone::UTC`]: `Mon, 02 Mar 2026 13:22:15 +0000`
	///   * [Local][`FormatTimezone::Local`]: `Mon, 02 Mar 2026 15:22:15 +0200`
	RFC2822(FormatTimezone),

	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html).
	///
	///   * [UTC][`FormatTimezone::UTC`]: `Mar  2 13:22:15`
	///   * [Local][`FormatTimezone::Local`]: `Mar  2 15:22:15`
	RFC3164(FormatTimezone),

	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF).
	///
	///   * [UTC][`FormatTimezone::UTC`], in [seconds][`FormatPrecision::Seconds`]: `2026-03-02T13:22:15Z`
	///   * [UTC][`FormatTimezone::UTC`], with [milliseconds][`FormatPrecision::Milliseconds`]: `2026-03-02T13:22:15.488Z`
	///   * [UTC][`FormatTimezone::UTC`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `2026-03-02T13:22:15.488167982Z`
	///   * [Local][`FormatTimezone::Local`], in [seconds][`FormatPrecision::Seconds`]: `2026-03-02T15:22:15+02:00`
	///   * [Local][`FormatTimezone::Local`], with [milliseconds][`FormatPrecision::Milliseconds`]: `2026-03-02T15:22:15.488+02:00`
	///   * [Local][`FormatTimezone::Local`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `2026-03-02T15:22:15.488167982+02:00`
	RFC3339(FormatTimezone, FormatPrecision),

	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html).
	///
	///   * [UTC][`FormatTimezone::UTC`], in [seconds][`FormatPrecision::Seconds`]: `2026-03-02T13:22:15Z`
	///   * [UTC][`FormatTimezone::UTC`], with [milliseconds][`FormatPrecision::Milliseconds`]: `2026-03-02T13:22:15.488Z`
	///   * [UTC][`FormatTimezone::UTC`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `2026-03-02T13:22:15.488167982Z`
	///   * [Local][`FormatTimezone::Local`], in [seconds][`FormatPrecision::Seconds`]: `2026-03-02T15:22:15+02:00`
	///   * [Local][`FormatTimezone::Local`], with [milliseconds][`FormatPrecision::Milliseconds`]: `2026-03-02T15:22:15.488+02:00`
	///   * [Local][`FormatTimezone::Local`], with [nanoseconds][`FormatPrecision::Nanoseconds`]: `2026-03-02T15:22:15.488167982+02:00`
	ISO8601(FormatTimezone, FormatPrecision),

	/// An alias for [`Format::RFC7231`].
	HTTP,
	/// [RFC 7231](https://www.rfc-editor.org/rfc/rfc7231.html) (HTTP/1.1): `Mon, 02 Mar 2026 13:22:15 GMT`
	RFC7231,

	/// Timestamp, as time since Unix epoch.
	///
	///   * In [seconds][`FormatPrecision::Seconds`]: `1772457735`
	///   * In [milliseconds][`FormatPrecision::Milliseconds`]: `1772457735488`
	///   * In [nanoseconds][`FormatPrecision::Nanoseconds`]: `1772457735488167982`
	Timestamp(FormatPrecision),
}

// deprecated API formatting definitions
#[allow(non_upper_case_globals)]
impl Format {
	/// Compact datetime format, in UTC: `2026-03-02 13:22:15`
	#[deprecated(since = "0.9.0", note = "use Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcDateTime: Self = Self::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// Compact datetime with milliseconds, in UTC: `2026-03-02 13:22:15.488`
	#[deprecated(since = "0.9.0", note = "use Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisDateTime: Self = Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// Compact datetime with nanoseconds, in UTC: `2026-03-02 13:22:15.488167982`
	#[deprecated(since = "0.9.0", note = "use Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosDateTime: Self = Self::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Compact date, in UTC: `2026-03-02`
	#[deprecated(since = "0.9.0", note = "use Format::Date(FormatTimezone::UTC) instead")]
	pub const UtcDate: Self = Self::Date(FormatTimezone::UTC);
	/// Compact time, in UTC: `13:22:15`
	#[deprecated(since = "0.9.0", note = "use Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcTime: Self = Self::Time(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// Compact time with milliseconds, in UTC: `13:22:15.488`
	#[deprecated(since = "0.9.0", note = "use Format::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisTime: Self = Self::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// Compact time with nanoseconds, in UTC: `13:22:15.488167982`
	#[deprecated(since = "0.9.0", note = "use Format::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosTime: Self = Self::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Date/time format suitable to append to filenames. in UTC: `2026-03-02_13-22-15`
	#[deprecated(since = "0.9.0", note = "use Format::FileName(FormatTimezone::UTC) instead")]
	pub const UtcFileName: Self = Self::FileName(FormatTimezone::UTC);
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in UTC: `Mon, 02 Mar 2026 13:22:15 +0000`
	#[deprecated(since = "0.9.0", note = "use Format::RFC2822(FormatTimezone::UTC) instead")]
	pub const UtcRFC2822: Self = Self::RFC2822(FormatTimezone::UTC);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in UTC: `2026-03-02T13:22:15Z`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcRFC3339: Self = Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisRFC3339: Self = Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosRFC3339: Self = Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in UTC: `Mar  2 13:22:15`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3164(FormatTimezone::UTC) instead")]
	pub const UtcRFC3164: Self = Self::RFC3164(FormatTimezone::UTC);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), in UTC: `2026-03-02T13:22:15Z`
	#[deprecated(since = "0.9.0", note = "use Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcISO8601: Self = Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	#[deprecated(since = "0.9.0", note = "use Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisISO8601: Self = Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	#[deprecated(since = "0.9.0", note = "use Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosISO8601: Self = Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Compact datetime, in local timezone: `2026-03-02 15:22:15 +0200`
	#[deprecated(since = "0.9.0", note = "use Format::DateTime(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalDateTime: Self = Self::DateTime(FormatTimezone::Local, FormatPrecision::Seconds);
	/// Compact datetime with milliseconds, in local timezone: `2026-03-02 15:22:15.488 +0200`
	#[deprecated(since = "0.9.0", note = "use Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisDateTime: Self = Self::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// Compact datetime with nanoseconds, in local timezone: `2026-03-02 15:22:15.488167982 +0200`
	#[deprecated(since = "0.9.0", note = "use Format::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosDateTime: Self = Self::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Compact date, in local timezone: `2026-03-02`
	#[deprecated(since = "0.9.0", note = "use Format::Date(FormatTimezone::Local) instead")]
	pub const LocalDate: Self = Self::Date(FormatTimezone::Local);
	/// Compact time, in local timezone: `15:22:15`
	#[deprecated(since = "0.9.0", note = "use Format::Time(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalTime: Self = Self::Time(FormatTimezone::Local, FormatPrecision::Seconds);
	/// Compact time with milliseconds, in local timezone: `15:22:15.488`
	#[deprecated(since = "0.9.0", note = "use Format::Time(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisTime: Self = Self::Time(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// Compact time with nanoseconds, in local timezone: `15:22:15.488167982`
	#[deprecated(since = "0.9.0", note = "use Format::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosTime: Self = Self::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Date/time format suitable to append to filenames. in local timezone: `2026-03-02_15-22-15`
	#[deprecated(since = "0.9.0", note = "use Format::FileName(FormatTimezone::Local) instead")]
	pub const LocalFileName: Self = Self::FileName(FormatTimezone::Local);
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in local timezone: `Mon, 02 Mar 2026 15:22:15 +0200`
	#[deprecated(since = "0.9.0", note = "use Format::RFC2822(FormatTimezone::Local) instead")]
	pub const LocalRFC2822: Self = Self::RFC2822(FormatTimezone::Local);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in local timezone: `2026-03-02T15:22:15+02:00`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalRFC3339: Self = Self::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+02:00`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisRFC3339: Self = Self::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488167982+02:00`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosRFC3339: Self = Self::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in local timezone: `Mar  2 15:22:15`
	#[deprecated(since = "0.9.0", note = "use Format::RFC3164(FormatTimezone::Local) instead")]
	pub const LocalRFC3164: Self = Self::RFC3164(FormatTimezone::Local);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), in local timezone: `2026-03-02T15:22:15+02:00`
	#[deprecated(since = "0.9.0", note = "use Format::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalISO8601: Self = Self::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+02:00`
	#[deprecated(since = "0.9.0", note = "use Format::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisISO8601: Self = Self::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488167982+02:00`
	#[deprecated(since = "0.9.0", note = "use Format::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosISO8601: Self = Self::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Seconds since UNIX epoch: `1772457735`
	#[deprecated(since = "0.9.0", note = "use Format::Timestamp(FormatPrecision::Seconds) instead")]
	pub const TimestampSeconds: Self = Self::Timestamp(FormatPrecision::Seconds);
	/// Milliseconds since UNIX epoch: `1772457735488`
	#[deprecated(since = "0.9.0", note = "use Format::Timestamp(FormatPrecision::Milliseconds) instead")]
	pub const TimestampMilliseconds: Self = Self::Timestamp(FormatPrecision::Milliseconds);
	/// Nanoseconds since UNIX epoch: `1772457735488167982`
	#[deprecated(since = "0.9.0", note = "use Format::Timestamp(FormatPrecision::Nanoseconds) instead")]
	pub const TimestampNanoseconds: Self = Self::Timestamp(FormatPrecision::Nanoseconds);
}

impl Default for Format {
	/// Returns a default formatting for [`Timestamp`], as [`Self::DateTime`] in [`UTC`][FormatTimezone::UTC] with [millisecond][FormatPrecision::Milliseconds] precision.
	fn default() -> Self {
		Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds)
	}
}

impl Format {
	/// Returns a string name for the [`Format`]
	pub fn name(&self) -> &'static str {
		match self {
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds) => "compact datetime (UTC)",
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds) => "compact datetime with milliseconds (UTC)",
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => "compact datetime with nanoseconds (UTC)",
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Seconds) => "compact datetime (local)",
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds) => "compact datetime with milliseconds (local)",
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds) => "compact datetime with nanoseconds (local)",
			Self::Date(FormatTimezone::UTC) => "compact date (UTC)",
			Self::Date(FormatTimezone::Local) => "compact date (local)",
			Self::Time(FormatTimezone::UTC, FormatPrecision::Seconds) => "compact time (UTC)",
			Self::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds) => "compact time with milliseconds (UTC)",
			Self::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => "compact time with nanoseconds (UTC)",
			Self::Time(FormatTimezone::Local, FormatPrecision::Seconds) => "compact time (local)",
			Self::Time(FormatTimezone::Local, FormatPrecision::Milliseconds) => "compact time with milliseconds (local)",
			Self::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds) => "compact time with nanoseconds (local)",
			Self::FileName(FormatTimezone::UTC) => "date+time filename (UTC)",
			Self::FileName(FormatTimezone::Local) => "date+time filename (local)",
			Self::RFC2822(FormatTimezone::UTC) => "RFC 2822 (UTC)",
			Self::RFC2822(FormatTimezone::Local) => "RFC 2822 (local)",
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds) => "RFC 3339 (UTC)",
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds) => "RFC 3339 with milliseconds (UTC)",
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => "RFC 3339 with nanoseconds (UTC)",
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds) => "RFC 3339 (local)",
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds) => "RFC 3339 with milliseconds (local)",
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds) => "RFC 3339 with nanoseconds (local)",
			Self::RFC3164(FormatTimezone::UTC) => "syslog RFC 3164 (UTC)",
			Self::RFC3164(FormatTimezone::Local) => "syslog RFC 3164 (local)",
			Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds) => "ISO 8601 (UTC)",
			Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds) => "ISO 8601 with milliseconds (UTC)",
			Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => "ISO 8601 with nanoseconds (UTC)",
			Self::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds) => "ISO 8601 (local)",
			Self::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds) => "ISO 8601 with milliseconds (local)",
			Self::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds) => "ISO 8601 with nanoseconds (local)",

			Self::HTTP | Self::RFC7231 => "RFC 7231 (GMT)",

			Self::Timestamp(FormatPrecision::Seconds) => "seconds since epoch",
			Self::Timestamp(FormatPrecision::Milliseconds) => "milliseconds since epoch",
			Self::Timestamp(FormatPrecision::Nanoseconds) => "nanoseconds since epoch",
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
			Self::DateTime(FormatTimezone::UTC, pr) => {
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
			Self::DateTime(FormatTimezone::Local, pr) => {
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
			Self::FileName(tz) => {
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
			Self::Date(tz) => {
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
			Self::RFC3339(FormatTimezone::UTC, pr) | Self::ISO8601(FormatTimezone::UTC, pr) => {
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
			Self::RFC3339(FormatTimezone::Local, pr) | Self::ISO8601(FormatTimezone::Local, pr) => {
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
			Self::HTTP | Self::RFC7231 => {
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
			Self::Timestamp(FormatPrecision::Seconds) => write!(out, "{}", ts.as_secs()),
			Self::Timestamp(FormatPrecision::Milliseconds) => write!(out, "{}", ts.as_millis()),
			Self::Timestamp(FormatPrecision::Nanoseconds) => write!(out, "{}", ts.as_nanos()),
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

			// 2026-03-02 13:22:15
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds) => 19,
			// 2026-03-02 13:22:15.488
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds) => 23,
			// 2026-03-02 13:22:15.488167982
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => 29,
			// 2026-03-02 15:22:15 +0200
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Seconds) => 25,
			// 2026-03-02 15:22:15.488 +0200
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds) => 29,
			// 2026-03-02 15:22:15.488167982 +0200
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds) => 35,
			// 2026-03-02
			Self::Date(FormatTimezone::UTC) => 10,
			// 2026-03-02
			Self::Date(FormatTimezone::Local) => 10,
			// 13:22:15
			Self::Time(FormatTimezone::UTC, FormatPrecision::Seconds) => 8,
			// 13:22:15.488
			Self::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds) => 12,
			// 13:22:15.488167982
			Self::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => 18,
			// 15:22:15
			Self::Time(FormatTimezone::Local, FormatPrecision::Seconds) => 8,
			// 15:22:15.488
			Self::Time(FormatTimezone::Local, FormatPrecision::Milliseconds) => 12,
			// 15:22:15.488167982
			Self::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds) => 18,
			// 2026-03-02_13-22-15
			Self::FileName(FormatTimezone::UTC) => 19,
			// 2026-03-02_15-22-15
			Self::FileName(FormatTimezone::Local) => 19,
			// Mon, 02 Mar 2026 13:22:15 +0000
			Self::RFC2822(FormatTimezone::UTC) => 31,
			// Mon, 02 Mar 2026 15:22:15 +0200
			Self::RFC2822(FormatTimezone::Local) => 31,
			// Mar  2 13:22:15
			Self::RFC3164(FormatTimezone::UTC) => 15,
			// Mar  2 15:22:15
			Self::RFC3164(FormatTimezone::Local) => 15,
			// 2026-03-02T13:22:15Z
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds) | Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds) => 20,
			// 2026-03-02T13:22:15.488Z
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds) | Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds) => 24,
			// 2026-03-02T13:22:15.488167982Z
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds) | Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => 30,
			// 2026-03-02T15:22:15+02:00
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds) | Self::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds) => 25,
			// 2026-03-02T15:22:15.488+02:00
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds) | Self::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds) => 29,
			// 2026-03-02T15:22:15.488167982+02:00
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds) | Self::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds) => 35,
			// Mon, 02 Mar 2026 13:22:15 GMT
			Self::HTTP | Self::RFC7231 => 29,

			// variable length formats

			// 1772457735
			Self::Timestamp(FormatPrecision::Seconds) => count_digits(ts.as_secs() as u128),
			// 1772457735488
			Self::Timestamp(FormatPrecision::Milliseconds) => count_digits(ts.as_millis()),
			// 1772457735488167982
			Self::Timestamp(FormatPrecision::Nanoseconds) => count_digits(ts.as_nanos()),
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

	/// Serializes a [`Timestamp`] into an integer, if the format supports it.
	pub fn as_integer(&self, ts: &Timestamp) -> Option<u128> {
		match self {
			Self::Timestamp(pr) => match pr {
				FormatPrecision::Seconds => Some(ts.as_secs() as u128),
				FormatPrecision::Milliseconds => Some(ts.as_millis()),
				FormatPrecision::Nanoseconds => Some(ts.as_nanos()),
			},
			_ => None,
		}
	}

	/// Evaluates whether this [`Format`] is a numeric integer.
	pub fn is_integer(&self) -> bool {
		matches!(self, Self::Timestamp(_))
	}

	/// Evaluates whether this [`Format`] is [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html).
	#[deprecated(since = "0.9.0", note = "match to Format::RFC3339 instead")]
	pub fn is_rfc_3339(&self) -> bool {
		matches!(self, Self::RFC3339(_, _))
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
				Format::RFC2822(FormatTimezone::Local),
				Format::RFC2822(FormatTimezone::UTC),
				Format::RFC3164(FormatTimezone::Local),
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
	fn format_is_integer() {
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
