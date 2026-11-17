use std::fmt;
use std::io;

use crate::timestamp::Timestamp;
use crate::write;

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
	#[deprecated(note = "use Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcDateTime: Self = Self::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// Compact datetime with milliseconds, in UTC: `2026-03-02 13:22:15.488`
	#[deprecated(note = "use Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisDateTime: Self = Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// Compact datetime with nanoseconds, in UTC: `2026-03-02 13:22:15.488167982`
	#[deprecated(note = "use Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosDateTime: Self = Self::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Compact date, in UTC: `2026-03-02`
	#[deprecated(note = "use Format::Date(FormatTimezone::UTC) instead")]
	pub const UtcDate: Self = Self::Date(FormatTimezone::UTC);
	/// Compact time, in UTC: `13:22:15`
	#[deprecated(note = "use Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcTime: Self = Self::Time(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// Compact time with milliseconds, in UTC: `13:22:15.488`
	#[deprecated(note = "use Format::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisTime: Self = Self::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// Compact time with nanoseconds, in UTC: `13:22:15.488167982`
	#[deprecated(note = "use Format::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosTime: Self = Self::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Date/time format suitable to append to filenames. in UTC: `2026-03-02_13-22-15`
	#[deprecated(note = "use Format::FileName(FormatTimezone::UTC) instead")]
	pub const UtcFileName: Self = Self::FileName(FormatTimezone::UTC);
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in UTC: `Mon, 02 Mar 2026 13:22:15 +0000`
	#[deprecated(note = "use Format::RFC2822(FormatTimezone::UTC) instead")]
	pub const UtcRFC2822: Self = Self::RFC2822(FormatTimezone::UTC);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in UTC: `2026-03-02T13:22:15Z`
	#[deprecated(note = "use Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcRFC3339: Self = Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	#[deprecated(note = "use Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisRFC3339: Self = Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	#[deprecated(note = "use Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosRFC3339: Self = Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in UTC: `Mar  2 13:22:15`
	#[deprecated(note = "use Format::RFC3164(FormatTimezone::UTC) instead")]
	pub const UtcRFC3164: Self = Self::RFC3164(FormatTimezone::UTC);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), in UTC: `2026-03-02T13:22:15Z`
	#[deprecated(note = "use Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds) instead")]
	pub const UtcISO8601: Self = Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	#[deprecated(note = "use Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds) instead")]
	pub const UtcMillisISO8601: Self = Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	#[deprecated(note = "use Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds) instead")]
	pub const UtcNanosISO8601: Self = Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds);
	/// Compact datetime, in local timezone: `2026-03-02 15:22:15 +0200`
	#[deprecated(note = "use Format::DateTime(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalDateTime: Self = Self::DateTime(FormatTimezone::Local, FormatPrecision::Seconds);
	/// Compact datetime with milliseconds, in local timezone: `2026-03-02 15:22:15.488 +0200`
	#[deprecated(note = "use Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisDateTime: Self = Self::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// Compact datetime with nanoseconds, in local timezone: `2026-03-02 15:22:15.488167982 +0200`
	#[deprecated(note = "use Format::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosDateTime: Self = Self::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Compact date, in local timezone: `2026-03-02`
	#[deprecated(note = "use Format::Date(FormatTimezone::Local) instead")]
	pub const LocalDate: Self = Self::Date(FormatTimezone::Local);
	/// Compact time, in local timezone: `15:22:15`
	#[deprecated(note = "use Format::Time(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalTime: Self = Self::Time(FormatTimezone::Local, FormatPrecision::Seconds);
	/// Compact time with milliseconds, in local timezone: `15:22:15.488`
	#[deprecated(note = "use Format::Time(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisTime: Self = Self::Time(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// Compact time with nanoseconds, in local timezone: `15:22:15.488167982`
	#[deprecated(note = "use Format::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosTime: Self = Self::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Date/time format suitable to append to filenames. in local timezone: `2026-03-02_15-22-15`
	#[deprecated(note = "use Format::FileName(FormatTimezone::Local) instead")]
	pub const LocalFileName: Self = Self::FileName(FormatTimezone::Local);
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in local timezone: `Mon, 02 Mar 2026 15:22:15 +0200`
	#[deprecated(note = "use Format::RFC2822(FormatTimezone::Local) instead")]
	pub const LocalRFC2822: Self = Self::RFC2822(FormatTimezone::Local);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in local timezone: `2026-03-02T15:22:15+02:00`
	#[deprecated(note = "use Format::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalRFC3339: Self = Self::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+02:00`
	#[deprecated(note = "use Format::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisRFC3339: Self = Self::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488167982+02:00`
	#[deprecated(note = "use Format::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosRFC3339: Self = Self::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in local timezone: `Mar  2 15:22:15`
	#[deprecated(note = "use Format::RFC3164(FormatTimezone::Local) instead")]
	pub const LocalRFC3164: Self = Self::RFC3164(FormatTimezone::Local);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), in local timezone: `2026-03-02T15:22:15+02:00`
	#[deprecated(note = "use Format::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds) instead")]
	pub const LocalISO8601: Self = Self::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+02:00`
	#[deprecated(note = "use Format::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds) instead")]
	pub const LocalMillisISO8601: Self = Self::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488167982+02:00`
	#[deprecated(note = "use Format::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds) instead")]
	pub const LocalNanosISO8601: Self = Self::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds);
	/// Seconds since UNIX epoch: `1772457735`
	#[deprecated(note = "use Format::Timestamp(FormatPrecision::Seconds) instead")]
	pub const TimestampSeconds: Self = Self::Timestamp(FormatPrecision::Seconds);
	/// Milliseconds since UNIX epoch: `1772457735488`
	#[deprecated(note = "use Format::Timestamp(FormatPrecision::Milliseconds) instead")]
	pub const TimestampMilliseconds: Self = Self::Timestamp(FormatPrecision::Milliseconds);
	/// Nanoseconds since UNIX epoch: `1772457735488167982`
	#[deprecated(note = "use Format::Timestamp(FormatPrecision::Nanoseconds) instead")]
	pub const TimestampNanoseconds: Self = Self::Timestamp(FormatPrecision::Nanoseconds);
}

impl Default for Format {
	/// Returns a default formatting for [`Timestamp`], as [`Self::DateTime`] in [`UTC`][FormatTimezone::UTC] with [millisecond][FormatPrecision::Milliseconds] precision.
	fn default() -> Self {
		Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds)
	}
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
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
		let mut buf = write::Buffer::new();

		match self {
			Self::DateTime(FormatTimezone::UTC, pr) => {
				let parts = ts.as_utc_parts();
				match pr {
					FormatPrecision::Seconds => {
						// "2026-03-02 13:22:15"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.space();
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
					}
					FormatPrecision::Milliseconds => {
						// "2026-03-02 13:22:15.488"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.space();
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
					}
					FormatPrecision::Nanoseconds => {
						// "2026-03-02 13:22:15.488167982"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.space();
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.u32_dec_padded(parts.nanoseconds, b'0', 6);
					}
				}
			}
			Self::DateTime(FormatTimezone::Local, pr) => {
				let parts = ts.as_local_parts();
				match pr {
					FormatPrecision::Seconds => {
						// "2026-03-02 15:22:15 +0200"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.space();
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.space();
						buf.str(parts.gmt_offset_sign());
						buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
						buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
					}
					FormatPrecision::Milliseconds => {
						// "2026-03-02 15:22:15.488 +0200"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.space();
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.space();
						buf.str(parts.gmt_offset_sign());
						buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
						buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
					}
					FormatPrecision::Nanoseconds => {
						// "2026-03-02 15:22:15.488167982 +0200"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.space();
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.u32_dec_padded(parts.nanoseconds, b'0', 6);
						buf.space();
						buf.str(parts.gmt_offset_sign());
						buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
						buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
					}
				}
			}
			Self::FileName(tz) => {
				// 	"2026-03-02_13-22-15"
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};

				buf.u64_dec(parts.year);
				buf.hyphen();
				buf.u8_dec_padded(parts.month, b'0', 2);
				buf.hyphen();
				buf.u8_dec_padded(parts.month_day, b'0', 2);
				buf.underscore();
				buf.u8_dec_padded(parts.hour, b'0', 2);
				buf.hyphen();
				buf.u8_dec_padded(parts.minutes, b'0', 2);
				buf.hyphen();
				buf.u8_dec_padded(parts.seconds, b'0', 2);
			}
			Self::Date(tz) => {
				// "2026-03-02"
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};

				buf.u64_dec(parts.year);
				buf.hyphen();
				buf.u8_dec_padded(parts.month, b'0', 2);
				buf.hyphen();
				buf.u8_dec_padded(parts.month_day, b'0', 2);
			}
			Self::Time(tz, pr) => {
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};

				match pr {
					FormatPrecision::Seconds => {
						// "13:22:15"
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
					}
					FormatPrecision::Milliseconds => {
						// "13:22:15.488"
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
					}
					FormatPrecision::Nanoseconds => {
						// "13:22:15.488167982"
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.u32_dec_padded(parts.nanoseconds, b'0', 6);
					}
				}
			}
			Self::RFC2822(tz) => {
				// "Mon, 02 Mar 2026 13:22:15 +0000" (UTC), "Mon, 02 Mar 2026 15:22:15 +0200" (local)
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};

				buf.str(parts.day_name());
				buf.str(", ");
				buf.u8_dec_padded(parts.month_day, b'0', 2);
				buf.space();
				buf.str(parts.month_name());
				buf.space();
				buf.u64_dec(parts.year);
				buf.space();
				buf.u8_dec_padded(parts.hour, b'0', 2);
				buf.colon();
				buf.u8_dec_padded(parts.minutes, b'0', 2);
				buf.colon();
				buf.u8_dec_padded(parts.seconds, b'0', 2);
				buf.space();
				buf.str(parts.gmt_offset_sign());
				buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
				buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
			}
			Self::RFC3339(FormatTimezone::UTC, pr) | Self::ISO8601(FormatTimezone::UTC, pr) => {
				let parts = ts.as_utc_parts();
				match pr {
					FormatPrecision::Seconds => {
						// "2026-03-02T13:22:15Z"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.u8(b'T');
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.u8(b'Z');
					}
					FormatPrecision::Milliseconds => {
						// "2026-03-02T13:22:15.488Z`
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.u8(b'T');
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.u8(b'Z');
					}
					FormatPrecision::Nanoseconds => {
						// "2026-03-02T13:22:15.488167982Z"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.u8(b'T');
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.u32_dec_padded(parts.nanoseconds, b'0', 6);
						buf.u8(b'Z');
					}
				};
			}
			Self::RFC3339(FormatTimezone::Local, pr) | Self::ISO8601(FormatTimezone::Local, pr) => {
				let parts = ts.as_local_parts();

				match pr {
					FormatPrecision::Seconds => {
						// "2026-03-02T15:22:15+02:00"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.u8(b'T');
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.str(parts.gmt_offset_sign());
						buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
					}
					FormatPrecision::Milliseconds => {
						// "2026-03-02T15:22:15.488+02:00`
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.u8(b'T');
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.u8(b'.');
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.str(parts.gmt_offset_sign());
						buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
					}
					FormatPrecision::Nanoseconds => {
						// "2026-03-02T15:22:15.488167982+02:00"
						buf.u64_dec(parts.year);
						buf.hyphen();
						buf.u8_dec_padded(parts.month, b'0', 2);
						buf.hyphen();
						buf.u8_dec_padded(parts.month_day, b'0', 2);
						buf.u8(b'T');
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.u8(b'.');
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
						buf.u32_dec_padded(parts.nanoseconds, b'0', 6);
						buf.str(parts.gmt_offset_sign());
						buf.u8_dec_padded(parts.gmt_offset_hours, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.gmt_offset_minutes, b'0', 2);
					}
				};
			}
			Self::RFC3164(tz) => {
				// "Mar  2 15:22:15"
				let parts = match tz {
					FormatTimezone::UTC => ts.as_utc_parts(),
					FormatTimezone::Local => ts.as_local_parts(),
				};

				buf.str(parts.month_name());
				buf.space();
				buf.u8_dec_padded(parts.month_day, b' ', 2);
				buf.space();
				buf.u8_dec_padded(parts.hour, b'0', 2);
				buf.colon();
				buf.u8_dec_padded(parts.minutes, b'0', 2);
				buf.colon();
				buf.u8_dec_padded(parts.seconds, b'0', 2);
			}
			Self::HTTP | Self::RFC7231 => {
				// "Mon, 02 Mar 2026 13:22:15 GMT"
				let parts = ts.as_utc_parts();
				buf.str(parts.day_name());
				buf.str(", ");
				buf.u8_dec_padded(parts.month_day, b'0', 2);
				buf.space();
				buf.str(parts.month_name());
				buf.space();
				buf.u64_dec(parts.year);
				buf.space();
				buf.u8_dec_padded(parts.hour, b'0', 2);
				buf.colon();
				buf.u8_dec_padded(parts.minutes, b'0', 2);
				buf.colon();
				buf.u8_dec_padded(parts.seconds, b'0', 2);
				buf.str(" GMT");
			}
			// "1772457735"
			Self::Timestamp(FormatPrecision::Seconds) => buf.u64_dec(ts.as_secs()),
			// "1772457735488"
			Self::Timestamp(FormatPrecision::Milliseconds) => buf.u128_dec(ts.as_millis()),
			// "1772457735488167982"
			Self::Timestamp(FormatPrecision::Nanoseconds) => buf.u128_dec(ts.as_nanos()),
		}

		buf.write(out)
	}

	/// Returns the string representation length for a [`Timestamp`] using this [`Format`]
	pub fn string_len(&self, ts: &Timestamp) -> usize {
		// minor optimization: every date before the timestamp for
		// 9999-12-30 00:00:00 has a four-digit year, regardless of timezone.
		const FOUR_DIGIT_YEAR_CUTOFF_TIMESTAMP_SECS: u64 = 253402128000;

		let digits_u64 = |n: u64| -> usize { n.checked_ilog10().unwrap_or(0) as usize + 1 };
		let digits_u128 = |n: u128| -> usize { n.checked_ilog10().unwrap_or(0) as usize + 1 };
		let digits_utc_year = |ts: &Timestamp| -> usize {
			if ts.as_secs() < FOUR_DIGIT_YEAR_CUTOFF_TIMESTAMP_SECS {
				4
			} else {
				digits_u64(ts.as_utc_parts().year)
			}
		};
		let digits_local_year = |ts: &Timestamp| -> usize {
			if ts.as_secs() < FOUR_DIGIT_YEAR_CUTOFF_TIMESTAMP_SECS {
				4
			} else {
				digits_u64(ts.as_local_parts().year)
			}
		};

		match self {
			// fixed-size formats

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
			// Mar  2 13:22:15
			Self::RFC3164(FormatTimezone::UTC) => 15,
			// Mar  2 15:22:15
			Self::RFC3164(FormatTimezone::Local) => 15,

			// variable length formats

			// 2026-03-02 13:22:15
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds) => 15 + digits_utc_year(ts),
			// 2026-03-02 13:22:15.488
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds) => 19 + digits_utc_year(ts),
			// 2026-03-02 13:22:15.488167982
			Self::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => 25 + digits_utc_year(ts),
			// 2026-03-02 15:22:15 +0200
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Seconds) => 21 + digits_local_year(ts),
			// 2026-03-02 15:22:15.488 +0200
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds) => 25 + digits_local_year(ts),
			// 2026-03-02 15:22:15.488167982 +0200
			Self::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds) => 31 + digits_local_year(ts),
			// 2026-03-02
			Self::Date(FormatTimezone::UTC) => 6 + digits_utc_year(ts),
			// 2026-03-02
			Self::Date(FormatTimezone::Local) => 6 + digits_local_year(ts),
			// 2026-03-02_13-22-15
			Self::FileName(FormatTimezone::UTC) => 15 + digits_utc_year(ts),
			// 2026-03-02_15-22-15
			Self::FileName(FormatTimezone::Local) => 15 + digits_local_year(ts),
			// Mon, 02 Mar 2026 13:22:15 +0000
			Self::RFC2822(FormatTimezone::UTC) => 27 + digits_utc_year(ts),
			// Mon, 02 Mar 2026 15:22:15 +0200
			Self::RFC2822(FormatTimezone::Local) => 27 + digits_local_year(ts),
			// 2026-03-02T13:22:15Z
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds) | Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds) => 16 + digits_utc_year(ts),
			// 2026-03-02T13:22:15.488Z
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds) | Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds) => 20 + digits_utc_year(ts),
			// 2026-03-02T13:22:15.488167982Z
			Self::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds) | Self::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds) => 26 + digits_utc_year(ts),
			// 2026-03-02T15:22:15+02:00
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds) | Self::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds) => 21 + digits_local_year(ts),
			// 2026-03-02T15:22:15.488+02:00
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds) | Self::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds) => 25 + digits_local_year(ts),
			// 2026-03-02T15:22:15.488167982+02:00
			Self::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds) | Self::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds) => 31 + digits_local_year(ts),
			// Mon, 02 Mar 2026 13:22:15 GMT
			Self::HTTP | Self::RFC7231 => 25 + digits_utc_year(ts),

			// 1772457735
			Self::Timestamp(FormatPrecision::Seconds) => digits_u64(ts.as_secs()),
			// 1772457735488
			Self::Timestamp(FormatPrecision::Milliseconds) => digits_u128(ts.as_millis()),
			// 1772457735488167982
			Self::Timestamp(FormatPrecision::Nanoseconds) => digits_u128(ts.as_nanos()),
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
	#[deprecated(note = "match to Format::RFC3339 instead")]
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
		let ts = Timestamp::from_utc_date(2026, 3, 6, 14, 43, 49, 38, 23456).expect("invalid parts");

		assert_eq!(Format::Timestamp(FormatPrecision::Seconds).as_string(&ts), "1772808229");
		assert_eq!(Format::Timestamp(FormatPrecision::Milliseconds).as_string(&ts), "1772808229038");
		assert_eq!(Format::Timestamp(FormatPrecision::Nanoseconds).as_string(&ts), "1772808229038023456");
	}

	#[test]
	fn timestamp_as_utc_string() {
		let ts = Timestamp::from_utc_date(2026, 3, 6, 14, 43, 49, 38, 23456).expect("invalid parts");

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
			let ts = Timestamp::from_utc_date(2026, 3, 6, 14, 43, 49, 38, 23456).expect("invalid parts");

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
			for ts in [
				Timestamp::from_utc_date(2026, 3, 6, 14, 43, 49, 38, 23456).expect("invalid parts for year 2026"),
				Timestamp::from_utc_date(9999, 12, 31, 23, 59, 59, 999, 999999).expect("invalid parts for last day of year 9999"),
				Timestamp::from_utc_date(10000, 1, 1, 0, 0, 0, 0, 0).expect("invalid parts for first day of year 10000"),
				Timestamp::from_utc_date(31562, 9, 15, 6, 13, 55, 777, 8888).expect("invalid parts for year 31562"),
				Timestamp::from_utc_date(54732890, 1, 26, 22, 50, 11, 111, 222).expect("invalid parts for year 54732890"),
			] {
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
					assert_eq!(format.string_len(&ts), format.as_string(&ts).len(), "{} for {}", format, ts);
				}
			}
		})
	}

	#[test]
	fn timestamp_as_integer() {
		let ts = Timestamp::from_utc_date(2026, 1, 29, 7, 43, 19, 134, 943903).expect("invalid parts");

		assert_eq!(Format::Timestamp(FormatPrecision::Seconds).as_integer(&ts), Some(1769672599_u128));
		assert_eq!(Format::Timestamp(FormatPrecision::Milliseconds).as_integer(&ts), Some(1769672599134_u128));
		assert_eq!(Format::Timestamp(FormatPrecision::Nanoseconds).as_integer(&ts), Some(1769672599134943903_u128));
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
		assert!(Format::Timestamp(FormatPrecision::Seconds).is_integer());
		assert!(Format::Timestamp(FormatPrecision::Milliseconds).is_integer());
		assert!(Format::Timestamp(FormatPrecision::Nanoseconds).is_integer());
		assert!(!Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds).is_integer());
		assert!(!Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds).is_integer());
		assert!(!Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds).is_integer());
		assert!(!Format::RFC2822(FormatTimezone::UTC).is_integer());
		assert!(!Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds).is_integer());
		assert!(!Format::HTTP.is_integer());
		assert!(!Format::RFC7231.is_integer());
	}
}
