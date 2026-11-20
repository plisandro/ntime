use std::fmt;
use std::io;

use crate::CastError;
use crate::TimestampParts;
use crate::timestamp::Timestamp;
use crate::write;

/// Supported timezones for [`Format`] settings.
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[non_exhaustive]
pub enum Timezone {
	/// Local timezone.
	Local,
	/// UTC/GMT.
	Utc,
}

/// Supported precisions for [`Format`] settings.
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[non_exhaustive]
pub enum Precision {
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
	///   * [UTC][`Timezone::Utc`], in [seconds][`Precision::Seconds`]: `2026-03-02 13:22:15`
	///   * [UTC][`Timezone::Utc`], with [milliseconds][`Precision::Milliseconds`]: `2026-03-02 13:22:15.488`
	///   * [UTC][`Timezone::Utc`], with [nanoseconds][`Precision::Nanoseconds`]: `2026-03-02 13:22:15.488167982`
	///   * [Local][`Timezone::Local`], in [seconds][`Precision::Seconds`]: `2026-03-02 15:22:15 +0200`
	///   * [Local][`Timezone::Local`], with [milliseconds][`Precision::Milliseconds`]: `2026-03-02 15:22:15.488 +0200`
	///   * [Local][`Timezone::Local`], with [nanoseconds][`Precision::Nanoseconds`]: `2026-03-02 15:22:15.488167982 +0200`
	DateTime(Timezone, Precision),

	/// Compact date.
	///
	///   * [UTC][`Timezone::Utc`]: `2026-03-02`
	///   * [Local][`Timezone::Local`]: `2026-03-02`
	Date(Timezone),

	/// Compact time.
	///
	///   * [UTC][`Timezone::Utc`], in [seconds][`Precision::Seconds`]: `13:22:15`
	///   * [UTC][`Timezone::Utc`], with [milliseconds][`Precision::Milliseconds`]: `13:22:15.488`
	///   * [UTC][`Timezone::Utc`], with [nanoseconds][`Precision::Nanoseconds`]: `13:22:15.488167982`
	///   * [Local][`Timezone::Local`], in [seconds][`Precision::Seconds`]: `15:22:15`
	///   * [Local][`Timezone::Local`], with [milliseconds][`Precision::Milliseconds`]: `15:22:15.488`
	///   * [Local][`Timezone::Local`], with [nanoseconds][`Precision::Nanoseconds`]: `15:22:15.488167982`
	Time(Timezone, Precision),

	/// Date/time format suitable to append to filenames.
	///
	///   * [UTC][`Timezone::Utc`]: `2026-03-02_13-22-15`
	///   * [Local][`Timezone::Local`]: `2026-03-02_15-22-15`
	FileName(Timezone),

	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format).
	///
	///   * [UTC][`Timezone::Utc`]: `Mon, 02 Mar 2026 13:22:15 +0000`
	///   * [Local][`Timezone::Local`]: `Mon, 02 Mar 2026 15:22:15 +0200`
	Rfc2822(Timezone),

	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html).
	///
	///   * [UTC][`Timezone::Utc`]: `Mar  2 13:22:15`
	///   * [Local][`Timezone::Local`]: `Mar  2 15:22:15`
	Rfc3164(Timezone),

	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF).
	///
	///   * [UTC][`Timezone::Utc`], in [seconds][`Precision::Seconds`]: `2026-03-02T13:22:15Z`
	///   * [UTC][`Timezone::Utc`], with [milliseconds][`Precision::Milliseconds`]: `2026-03-02T13:22:15.488Z`
	///   * [UTC][`Timezone::Utc`], with [nanoseconds][`Precision::Nanoseconds`]: `2026-03-02T13:22:15.488167982Z`
	///   * [Local][`Timezone::Local`], in [seconds][`Precision::Seconds`]: `2026-03-02T15:22:15+02:00`
	///   * [Local][`Timezone::Local`], with [milliseconds][`Precision::Milliseconds`]: `2026-03-02T15:22:15.488+02:00`
	///   * [Local][`Timezone::Local`], with [nanoseconds][`Precision::Nanoseconds`]: `2026-03-02T15:22:15.488167982+02:00`
	Rfc3339(Timezone, Precision),

	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html).
	///
	///   * [UTC][`Timezone::Utc`], in [seconds][`Precision::Seconds`]: `2026-03-02T13:22:15Z`
	///   * [UTC][`Timezone::Utc`], with [milliseconds][`Precision::Milliseconds`]: `2026-03-02T13:22:15.488Z`
	///   * [UTC][`Timezone::Utc`], with [nanoseconds][`Precision::Nanoseconds`]: `2026-03-02T13:22:15.488167982Z`
	///   * [Local][`Timezone::Local`], in [seconds][`Precision::Seconds`]: `2026-03-02T15:22:15+02:00`
	///   * [Local][`Timezone::Local`], with [milliseconds][`Precision::Milliseconds`]: `2026-03-02T15:22:15.488+02:00`
	///   * [Local][`Timezone::Local`], with [nanoseconds][`Precision::Nanoseconds`]: `2026-03-02T15:22:15.488167982+02:00`
	Iso8601(Timezone, Precision),

	/// [RFC 7231](https://www.rfc-editor.org/rfc/rfc7231.html) (HTTP/1.1): `Mon, 02 Mar 2026 13:22:15 GMT`
	Rfc7231,

	/// Timestamp, as time since Unix epoch.
	///
	///   * In [seconds][`Precision::Seconds`]: `1772457735`
	///   * In [milliseconds][`Precision::Milliseconds`]: `1772457735488`
	///   * In [nanoseconds][`Precision::Nanoseconds`]: `1772457735488167982`
	Timestamp(Precision),
}

// deprecated API formatting definitions
#[allow(non_upper_case_globals)]
impl Format {
	/// Compact datetime format, in UTC: `2026-03-02 13:22:15`
	#[deprecated(since = "1.0.0", note = "use Format::DateTime(Timezone::Utc, Precision::Seconds) instead")]
	pub const UtcDateTime: Self = Self::DateTime(Timezone::Utc, Precision::Seconds);
	/// Compact datetime with milliseconds, in UTC: `2026-03-02 13:22:15.488`
	#[deprecated(since = "1.0.0", note = "use Format::DateTime(Timezone::Utc, Precision::Milliseconds) instead")]
	pub const UtcMillisDateTime: Self = Self::DateTime(Timezone::Utc, Precision::Milliseconds);
	/// Compact datetime with nanoseconds, in UTC: `2026-03-02 13:22:15.488167982`
	#[deprecated(since = "1.0.0", note = "use Format::DateTime(Timezone::Utc, Precision::Nanoseconds) instead")]
	pub const UtcNanosDateTime: Self = Self::DateTime(Timezone::Utc, Precision::Nanoseconds);
	/// Compact date, in UTC: `2026-03-02`
	#[deprecated(since = "1.0.0", note = "use Format::Date(Timezone::Utc) instead")]
	pub const UtcDate: Self = Self::Date(Timezone::Utc);
	/// Compact time, in UTC: `13:22:15`
	#[deprecated(since = "1.0.0", note = "use Format::Time(Timezone::Utc, Precision::Seconds) instead")]
	pub const UtcTime: Self = Self::Time(Timezone::Utc, Precision::Seconds);
	/// Compact time with milliseconds, in UTC: `13:22:15.488`
	#[deprecated(since = "1.0.0", note = "use Format::Time(Timezone::Utc, Precision::Milliseconds) instead")]
	pub const UtcMillisTime: Self = Self::Time(Timezone::Utc, Precision::Milliseconds);
	/// Compact time with nanoseconds, in UTC: `13:22:15.488167982`
	#[deprecated(since = "1.0.0", note = "use Format::Time(Timezone::Utc, Precision::Nanoseconds) instead")]
	pub const UtcNanosTime: Self = Self::Time(Timezone::Utc, Precision::Nanoseconds);
	/// Date/time format suitable to append to filenames. in UTC: `2026-03-02_13-22-15`
	#[deprecated(since = "1.0.0", note = "use Format::FileName(Timezone::Utc) instead")]
	pub const UtcFileName: Self = Self::FileName(Timezone::Utc);
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in UTC: `Mon, 02 Mar 2026 13:22:15 +0000`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc2822(Timezone::Utc) instead")]
	pub const UtcRFC2822: Self = Self::Rfc2822(Timezone::Utc);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in UTC: `2026-03-02T13:22:15Z`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3339(Timezone::Utc, Precision::Seconds) instead")]
	pub const UtcRFC3339: Self = Self::Rfc3339(Timezone::Utc, Precision::Seconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3339(Timezone::Utc, Precision::Milliseconds) instead")]
	pub const UtcMillisRFC3339: Self = Self::Rfc3339(Timezone::Utc, Precision::Milliseconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3339(Timezone::Utc, Precision::Nanoseconds) instead")]
	pub const UtcNanosRFC3339: Self = Self::Rfc3339(Timezone::Utc, Precision::Nanoseconds);
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in UTC: `Mar  2 13:22:15`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3164(Timezone::Utc) instead")]
	pub const UtcRFC3164: Self = Self::Rfc3164(Timezone::Utc);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), in UTC: `2026-03-02T13:22:15Z`
	#[deprecated(since = "1.0.0", note = "use Format::Iso8601(Timezone::Utc, Precision::Seconds) instead")]
	pub const UtcISO8601: Self = Self::Iso8601(Timezone::Utc, Precision::Seconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with millisecond precision, in UTC: `2026-03-02T13:22:15.488Z`
	#[deprecated(since = "1.0.0", note = "use Format::Iso8601(Timezone::Utc, Precision::Milliseconds) instead")]
	pub const UtcMillisISO8601: Self = Self::Iso8601(Timezone::Utc, Precision::Milliseconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with nanosecond precision, in UTC: `2026-03-02T13:22:15.488167982Z`
	#[deprecated(since = "1.0.0", note = "use Format::Iso8601(Timezone::Utc, Precision::Nanoseconds) instead")]
	pub const UtcNanosISO8601: Self = Self::Iso8601(Timezone::Utc, Precision::Nanoseconds);
	/// Compact datetime, in local timezone: `2026-03-02 15:22:15 +0200`
	#[deprecated(since = "1.0.0", note = "use Format::DateTime(Timezone::Local, Precision::Seconds) instead")]
	pub const LocalDateTime: Self = Self::DateTime(Timezone::Local, Precision::Seconds);
	/// Compact datetime with milliseconds, in local timezone: `2026-03-02 15:22:15.488 +0200`
	#[deprecated(since = "1.0.0", note = "use Format::DateTime(Timezone::Local, Precision::Milliseconds) instead")]
	pub const LocalMillisDateTime: Self = Self::DateTime(Timezone::Local, Precision::Milliseconds);
	/// Compact datetime with nanoseconds, in local timezone: `2026-03-02 15:22:15.488167982 +0200`
	#[deprecated(since = "1.0.0", note = "use Format::DateTime(Timezone::Local, Precision::Nanoseconds) instead")]
	pub const LocalNanosDateTime: Self = Self::DateTime(Timezone::Local, Precision::Nanoseconds);
	/// Compact date, in local timezone: `2026-03-02`
	#[deprecated(since = "1.0.0", note = "use Format::Date(Timezone::Local) instead")]
	pub const LocalDate: Self = Self::Date(Timezone::Local);
	/// Compact time, in local timezone: `15:22:15`
	#[deprecated(since = "1.0.0", note = "use Format::Time(Timezone::Local, Precision::Seconds) instead")]
	pub const LocalTime: Self = Self::Time(Timezone::Local, Precision::Seconds);
	/// Compact time with milliseconds, in local timezone: `15:22:15.488`
	#[deprecated(since = "1.0.0", note = "use Format::Time(Timezone::Local, Precision::Milliseconds) instead")]
	pub const LocalMillisTime: Self = Self::Time(Timezone::Local, Precision::Milliseconds);
	/// Compact time with nanoseconds, in local timezone: `15:22:15.488167982`
	#[deprecated(since = "1.0.0", note = "use Format::Time(Timezone::Local, Precision::Nanoseconds) instead")]
	pub const LocalNanosTime: Self = Self::Time(Timezone::Local, Precision::Nanoseconds);
	/// Date/time format suitable to append to filenames. in local timezone: `2026-03-02_15-22-15`
	#[deprecated(since = "1.0.0", note = "use Format::FileName(Timezone::Local) instead")]
	pub const LocalFileName: Self = Self::FileName(Timezone::Local);
	/// [RFC 2822](https://www.rfc-editor.org/rfc/rfc2822.html) (Internet Message Format), in local timezone: `Mon, 02 Mar 2026 15:22:15 +0200`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc2822(Timezone::Local) instead")]
	pub const LocalRFC2822: Self = Self::Rfc2822(Timezone::Local);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF), in local timezone: `2026-03-02T15:22:15+02:00`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3339(Timezone::Local, Precision::Seconds) instead")]
	pub const LocalRFC3339: Self = Self::Rfc3339(Timezone::Local, Precision::Seconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+02:00`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3339(Timezone::Local, Precision::Milliseconds) instead")]
	pub const LocalMillisRFC3339: Self = Self::Rfc3339(Timezone::Local, Precision::Milliseconds);
	/// [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html) (IETF) with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488167982+02:00`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3339(Timezone::Local, Precision::Nanoseconds) instead")]
	pub const LocalNanosRFC3339: Self = Self::Rfc3339(Timezone::Local, Precision::Nanoseconds);
	/// Abridged datetime format for syslog [RFC 3164](https://www.rfc-editor.org/rfc/rfc3164.html), in local timezone: `Mar  2 15:22:15`
	#[deprecated(since = "1.0.0", note = "use Format::Rfc3164(Timezone::Local) instead")]
	pub const LocalRFC3164: Self = Self::Rfc3164(Timezone::Local);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), in local timezone: `2026-03-02T15:22:15+02:00`
	#[deprecated(since = "1.0.0", note = "use Format::Iso8601(Timezone::Local, Precision::Seconds) instead")]
	pub const LocalISO8601: Self = Self::Iso8601(Timezone::Local, Precision::Seconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with millisecond precision, in local timezone: `2026-03-02T15:22:15.488+02:00`
	#[deprecated(since = "1.0.0", note = "use Format::Iso8601(Timezone::Local, Precision::Milliseconds) instead")]
	pub const LocalMillisISO8601: Self = Self::Iso8601(Timezone::Local, Precision::Milliseconds);
	/// [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html), with nanosecond precision, in local timezone: `2026-03-02T15:22:15.488167982+02:00`
	#[deprecated(since = "1.0.0", note = "use Format::Iso8601(Timezone::Local, Precision::Nanoseconds) instead")]
	pub const LocalNanosISO8601: Self = Self::Iso8601(Timezone::Local, Precision::Nanoseconds);
	/// Seconds since UNIX epoch: `1772457735`
	#[deprecated(since = "1.0.0", note = "use Format::Timestamp(Precision::Seconds) instead")]
	pub const TimestampSeconds: Self = Self::Timestamp(Precision::Seconds);
	/// Milliseconds since UNIX epoch: `1772457735488`
	#[deprecated(since = "1.0.0", note = "use Format::Timestamp(Precision::Milliseconds) instead")]
	pub const TimestampMilliseconds: Self = Self::Timestamp(Precision::Milliseconds);
	/// Nanoseconds since UNIX epoch: `1772457735488167982`
	#[deprecated(since = "1.0.0", note = "use Format::Timestamp(Precision::Nanoseconds) instead")]
	pub const TimestampNanoseconds: Self = Self::Timestamp(Precision::Nanoseconds);
}

impl Default for Format {
	/// Returns a default formatting for [`Timestamp`], as [`Self::DateTime`] in [`UTC`][Timezone::Utc] with [millisecond][Precision::Milliseconds] precision.
	fn default() -> Self {
		Self::DateTime(Timezone::Utc, Precision::Milliseconds)
	}
}

impl fmt::Display for Format {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

impl Format {
	/// An alias for [`Format::Rfc7231`].
	pub const HTTP: Self = Self::Rfc7231;

	/// Returns a string name for the [`Format`]
	pub fn name(&self) -> &'static str {
		match self {
			Self::DateTime(Timezone::Utc, Precision::Seconds) => "compact datetime (UTC)",
			Self::DateTime(Timezone::Utc, Precision::Milliseconds) => "compact datetime with milliseconds (UTC)",
			Self::DateTime(Timezone::Utc, Precision::Nanoseconds) => "compact datetime with nanoseconds (UTC)",
			Self::DateTime(Timezone::Local, Precision::Seconds) => "compact datetime (local)",
			Self::DateTime(Timezone::Local, Precision::Milliseconds) => "compact datetime with milliseconds (local)",
			Self::DateTime(Timezone::Local, Precision::Nanoseconds) => "compact datetime with nanoseconds (local)",
			Self::Date(Timezone::Utc) => "compact date (UTC)",
			Self::Date(Timezone::Local) => "compact date (local)",
			Self::Time(Timezone::Utc, Precision::Seconds) => "compact time (UTC)",
			Self::Time(Timezone::Utc, Precision::Milliseconds) => "compact time with milliseconds (UTC)",
			Self::Time(Timezone::Utc, Precision::Nanoseconds) => "compact time with nanoseconds (UTC)",
			Self::Time(Timezone::Local, Precision::Seconds) => "compact time (local)",
			Self::Time(Timezone::Local, Precision::Milliseconds) => "compact time with milliseconds (local)",
			Self::Time(Timezone::Local, Precision::Nanoseconds) => "compact time with nanoseconds (local)",
			Self::FileName(Timezone::Utc) => "date+time filename (UTC)",
			Self::FileName(Timezone::Local) => "date+time filename (local)",
			Self::Rfc2822(Timezone::Utc) => "RFC 2822 (UTC)",
			Self::Rfc2822(Timezone::Local) => "RFC 2822 (local)",
			Self::Rfc3339(Timezone::Utc, Precision::Seconds) => "RFC 3339 (UTC)",
			Self::Rfc3339(Timezone::Utc, Precision::Milliseconds) => "RFC 3339 with milliseconds (UTC)",
			Self::Rfc3339(Timezone::Utc, Precision::Nanoseconds) => "RFC 3339 with nanoseconds (UTC)",
			Self::Rfc3339(Timezone::Local, Precision::Seconds) => "RFC 3339 (local)",
			Self::Rfc3339(Timezone::Local, Precision::Milliseconds) => "RFC 3339 with milliseconds (local)",
			Self::Rfc3339(Timezone::Local, Precision::Nanoseconds) => "RFC 3339 with nanoseconds (local)",
			Self::Rfc3164(Timezone::Utc) => "syslog RFC 3164 (UTC)",
			Self::Rfc3164(Timezone::Local) => "syslog RFC 3164 (local)",
			Self::Iso8601(Timezone::Utc, Precision::Seconds) => "ISO 8601 (UTC)",
			Self::Iso8601(Timezone::Utc, Precision::Milliseconds) => "ISO 8601 with milliseconds (UTC)",
			Self::Iso8601(Timezone::Utc, Precision::Nanoseconds) => "ISO 8601 with nanoseconds (UTC)",
			Self::Iso8601(Timezone::Local, Precision::Seconds) => "ISO 8601 (local)",
			Self::Iso8601(Timezone::Local, Precision::Milliseconds) => "ISO 8601 with milliseconds (local)",
			Self::Iso8601(Timezone::Local, Precision::Nanoseconds) => "ISO 8601 with nanoseconds (local)",
			Self::Rfc7231 => "RFC 7231 (GMT)",
			Self::Timestamp(Precision::Seconds) => "seconds since epoch",
			Self::Timestamp(Precision::Milliseconds) => "milliseconds since epoch",
			Self::Timestamp(Precision::Nanoseconds) => "nanoseconds since epoch",
		}
	}

	/// Evaluates if the given [`Format`] is in UTC timezone.
	pub fn is_utc(&self) -> bool {
		match self {
			Self::DateTime(tz, _) => tz == &Timezone::Utc,
			Self::Date(tz) => tz == &Timezone::Utc,
			Self::Time(tz, _) => tz == &Timezone::Utc,
			Self::FileName(tz) => tz == &Timezone::Utc,
			Self::Rfc2822(tz) => tz == &Timezone::Utc,
			Self::Rfc3339(tz, _) => tz == &Timezone::Utc,
			Self::Rfc3164(tz) => tz == &Timezone::Utc,
			Self::Iso8601(tz, _) => tz == &Timezone::Utc,
			Self::Rfc7231 => true,
			Self::Timestamp(_) => true,
		}
	}

	/// Serializes a [`Timestamp`] as string, into a given [`std::io::Write`].
	pub fn write<T: io::Write>(&self, out: &mut T, ts: &Timestamp) -> io::Result<()> {
		let mut buf = write::Buffer::new();

		match self {
			Self::DateTime(Timezone::Utc, pr) => {
				let parts = TimestampParts::utc(ts);

				match pr {
					Precision::Seconds => {
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
					Precision::Milliseconds => {
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
					Precision::Nanoseconds => {
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
			Self::DateTime(Timezone::Local, pr) => {
				let parts = TimestampParts::try_local(ts).map_err(io::Error::other)?;

				match pr {
					Precision::Seconds => {
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
					Precision::Milliseconds => {
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
					Precision::Nanoseconds => {
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
					Timezone::Utc => TimestampParts::utc(ts),
					Timezone::Local => TimestampParts::try_local(ts).map_err(io::Error::other)?,
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
					Timezone::Utc => TimestampParts::utc(ts),
					Timezone::Local => TimestampParts::try_local(ts).map_err(io::Error::other)?,
				};

				buf.u64_dec(parts.year);
				buf.hyphen();
				buf.u8_dec_padded(parts.month, b'0', 2);
				buf.hyphen();
				buf.u8_dec_padded(parts.month_day, b'0', 2);
			}
			Self::Time(tz, pr) => {
				let parts = match tz {
					Timezone::Utc => TimestampParts::utc(ts),
					Timezone::Local => TimestampParts::try_local(ts).map_err(io::Error::other)?,
				};

				match pr {
					Precision::Seconds => {
						// "13:22:15"
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
					}
					Precision::Milliseconds => {
						// "13:22:15.488"
						buf.u8_dec_padded(parts.hour, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.minutes, b'0', 2);
						buf.colon();
						buf.u8_dec_padded(parts.seconds, b'0', 2);
						buf.dot();
						buf.u16_dec_padded(parts.milliseconds, b'0', 3);
					}
					Precision::Nanoseconds => {
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
			Self::Rfc2822(tz) => {
				// "Mon, 02 Mar 2026 13:22:15 +0000" (UTC), "Mon, 02 Mar 2026 15:22:15 +0200" (local)
				let parts = match tz {
					Timezone::Utc => TimestampParts::utc(ts),
					Timezone::Local => TimestampParts::try_local(ts).map_err(io::Error::other)?,
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
			Self::Rfc3339(Timezone::Utc, pr) | Self::Iso8601(Timezone::Utc, pr) => {
				let parts = TimestampParts::utc(ts);

				match pr {
					Precision::Seconds => {
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
					Precision::Milliseconds => {
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
					Precision::Nanoseconds => {
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
			Self::Rfc3339(Timezone::Local, pr) | Self::Iso8601(Timezone::Local, pr) => {
				let parts = TimestampParts::try_local(ts).map_err(io::Error::other)?;

				match pr {
					Precision::Seconds => {
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
					Precision::Milliseconds => {
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
					Precision::Nanoseconds => {
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
			Self::Rfc3164(tz) => {
				// "Mar  2 15:22:15"
				let parts = match tz {
					Timezone::Utc => TimestampParts::utc(ts),
					Timezone::Local => TimestampParts::try_local(ts).map_err(io::Error::other)?,
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
			Self::Rfc7231 => {
				// "Mon, 02 Mar 2026 13:22:15 GMT"
				let parts = TimestampParts::utc(ts);

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
			Self::Timestamp(Precision::Seconds) => buf.u64_dec(ts.as_secs()),
			// "1772457735488"
			Self::Timestamp(Precision::Milliseconds) => buf.u128_dec(ts.as_millis()),
			// "1772457735488167982"
			Self::Timestamp(Precision::Nanoseconds) => buf.u128_dec(ts.as_nanos()),
		}

		buf.write(out)
	}

	/// Returns the string representation length for a [`Timestamp`] using this [`Format`]
	///
	/// # Errors
	///
	/// Returns a [`CastError`] upon conversion issues.
	pub fn try_string_len_for(&self, ts: &Timestamp) -> Result<usize, CastError> {
		// minor optimization: every date before the timestamp for
		// 9999-12-30 00:00:00 has a four-digit year, regardless of timezone.
		const FOUR_DIGIT_YEAR_CUTOFF_TIMESTAMP_SECS: u64 = 253402128000;

		let digits_u64 = |n: u64| -> usize { n.checked_ilog10().unwrap_or(0) as usize + 1 };
		let digits_u128 = |n: u128| -> usize { n.checked_ilog10().unwrap_or(0) as usize + 1 };
		let digits_utc_year = |ts: &Timestamp| -> usize {
			if ts.as_secs() < FOUR_DIGIT_YEAR_CUTOFF_TIMESTAMP_SECS {
				4
			} else {
				digits_u64(ts.to_utc_parts().year)
			}
		};
		let digits_local_year = |ts: &Timestamp| -> Result<usize, CastError> {
			if ts.as_secs() < FOUR_DIGIT_YEAR_CUTOFF_TIMESTAMP_SECS {
				Ok(4)
			} else {
				Ok(digits_u64(ts.try_to_local_parts()?.year))
			}
		};

		Ok(match self {
			// fixed-size formats

			// 13:22:15
			Self::Time(Timezone::Utc, Precision::Seconds) => 8,
			// 13:22:15.488
			Self::Time(Timezone::Utc, Precision::Milliseconds) => 12,
			// 13:22:15.488167982
			Self::Time(Timezone::Utc, Precision::Nanoseconds) => 18,
			// 15:22:15
			Self::Time(Timezone::Local, Precision::Seconds) => 8,
			// 15:22:15.488
			Self::Time(Timezone::Local, Precision::Milliseconds) => 12,
			// 15:22:15.488167982
			Self::Time(Timezone::Local, Precision::Nanoseconds) => 18,
			// Mar  2 13:22:15
			Self::Rfc3164(Timezone::Utc) => 15,
			// Mar  2 15:22:15
			Self::Rfc3164(Timezone::Local) => 15,

			// variable length formats

			// 2026-03-02 13:22:15
			Self::DateTime(Timezone::Utc, Precision::Seconds) => 15 + digits_utc_year(ts),
			// 2026-03-02 13:22:15.488
			Self::DateTime(Timezone::Utc, Precision::Milliseconds) => 19 + digits_utc_year(ts),
			// 2026-03-02 13:22:15.488167982
			Self::DateTime(Timezone::Utc, Precision::Nanoseconds) => 25 + digits_utc_year(ts),
			// 2026-03-02 15:22:15 +0200
			Self::DateTime(Timezone::Local, Precision::Seconds) => 21 + digits_local_year(ts)?,
			// 2026-03-02 15:22:15.488 +0200
			Self::DateTime(Timezone::Local, Precision::Milliseconds) => 25 + digits_local_year(ts)?,
			// 2026-03-02 15:22:15.488167982 +0200
			Self::DateTime(Timezone::Local, Precision::Nanoseconds) => 31 + digits_local_year(ts)?,
			// 2026-03-02
			Self::Date(Timezone::Utc) => 6 + digits_utc_year(ts),
			// 2026-03-02
			Self::Date(Timezone::Local) => 6 + digits_local_year(ts)?,
			// 2026-03-02_13-22-15
			Self::FileName(Timezone::Utc) => 15 + digits_utc_year(ts),
			// 2026-03-02_15-22-15
			Self::FileName(Timezone::Local) => 15 + digits_local_year(ts)?,
			// Mon, 02 Mar 2026 13:22:15 +0000
			Self::Rfc2822(Timezone::Utc) => 27 + digits_utc_year(ts),
			// Mon, 02 Mar 2026 15:22:15 +0200
			Self::Rfc2822(Timezone::Local) => 27 + digits_local_year(ts)?,
			// 2026-03-02T13:22:15Z
			Self::Rfc3339(Timezone::Utc, Precision::Seconds) | Self::Iso8601(Timezone::Utc, Precision::Seconds) => 16 + digits_utc_year(ts),
			// 2026-03-02T13:22:15.488Z
			Self::Rfc3339(Timezone::Utc, Precision::Milliseconds) | Self::Iso8601(Timezone::Utc, Precision::Milliseconds) => 20 + digits_utc_year(ts),
			// 2026-03-02T13:22:15.488167982Z
			Self::Rfc3339(Timezone::Utc, Precision::Nanoseconds) | Self::Iso8601(Timezone::Utc, Precision::Nanoseconds) => 26 + digits_utc_year(ts),
			// 2026-03-02T15:22:15+02:00
			Self::Rfc3339(Timezone::Local, Precision::Seconds) | Self::Iso8601(Timezone::Local, Precision::Seconds) => 21 + digits_local_year(ts)?,
			// 2026-03-02T15:22:15.488+02:00
			Self::Rfc3339(Timezone::Local, Precision::Milliseconds) | Self::Iso8601(Timezone::Local, Precision::Milliseconds) => 25 + digits_local_year(ts)?,
			// 2026-03-02T15:22:15.488167982+02:00
			Self::Rfc3339(Timezone::Local, Precision::Nanoseconds) | Self::Iso8601(Timezone::Local, Precision::Nanoseconds) => 31 + digits_local_year(ts)?,
			// Mon, 02 Mar 2026 13:22:15 GMT
			Self::Rfc7231 => 25 + digits_utc_year(ts),

			// 1772457735
			Self::Timestamp(Precision::Seconds) => digits_u64(ts.as_secs()),
			// 1772457735488
			Self::Timestamp(Precision::Milliseconds) => digits_u128(ts.as_millis()),
			// 1772457735488167982
			Self::Timestamp(Precision::Nanoseconds) => digits_u128(ts.as_nanos()),
		})
	}

	/// Returns the string representation length for a [`Timestamp`] using this [`Format`]
	///
	/// # Panics
	///
	/// Panics upon conversion issues.
	#[inline]
	pub fn string_len_for(&self, ts: &Timestamp) -> usize {
		match self.try_string_len_for(ts) {
			Ok(l) => l,
			Err(e) => panic!("{e}"),
		}
	}

	/// Serializes a [`Timestamp`] into a [`String`].
	///
	/// # Errors
	///
	/// Returns a [`CastError`] upon conversion issues.
	pub fn try_to_string_for(&self, ts: &Timestamp) -> Result<String, CastError> {
		let mut out = Vec::new();
		if let Err(err) = self.write(&mut out, ts) {
			// unwrap the underlying CastError - if any
			return Err(err
				.into_inner()
				.and_then(|b| b.downcast::<CastError>().ok())
				.map(|b| *b)
				.unwrap_or(CastError::Internal("failed to serialize Timestamp")));
		};

		String::from_utf8(out).map_err(|_| CastError::Internal("Timestamp serialization generated invalid UTF-8"))
	}

	/// Serializes a [`Timestamp`] into a [`String`].
	///
	/// # Panics
	///
	/// Panics upon conversion issues.
	#[inline]
	pub fn to_string_for(&self, ts: &Timestamp) -> String {
		match self.try_to_string_for(ts) {
			Ok(s) => s,
			Err(e) => panic!("{e}"),
		}
	}

	/// Serializes a [`Timestamp`] into an integer, if the format supports it.
	pub fn to_integer_for(&self, ts: &Timestamp) -> Option<u128> {
		match self {
			Self::Timestamp(pr) => match pr {
				Precision::Seconds => Some(ts.as_secs() as u128),
				Precision::Milliseconds => Some(ts.as_millis()),
				Precision::Nanoseconds => Some(ts.as_nanos()),
			},
			_ => None,
		}
	}

	/// Evaluates whether this [`Format`] is a numeric integer.
	pub fn is_integer(&self) -> bool {
		matches!(self, Self::Timestamp(_))
	}

	/// Evaluates whether this [`Format`] is [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339.html).
	#[deprecated(since = "1.0.0", note = "match to Format::Rfc3339 instead")]
	pub fn is_rfc_3339(&self) -> bool {
		matches!(self, Self::Rfc3339(_, _))
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod test_format {
	use super::*;
	use crate::test_helpers;

	#[test]
	fn timestamp_as_number_string() {
		let ts = Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts");

		assert_eq!(Format::Timestamp(Precision::Seconds).to_string_for(&ts), "1772808229");
		assert_eq!(Format::Timestamp(Precision::Milliseconds).to_string_for(&ts), "1772808229038");
		assert_eq!(Format::Timestamp(Precision::Nanoseconds).to_string_for(&ts), "1772808229038023456");
	}

	#[test]
	fn timestamp_as_utc_string() {
		let ts = Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts");

		assert_eq!(Format::DateTime(Timezone::Utc, Precision::Seconds).to_string_for(&ts), "2026-03-06 14:43:49");
		assert_eq!(Format::DateTime(Timezone::Utc, Precision::Milliseconds).to_string_for(&ts), "2026-03-06 14:43:49.038");
		assert_eq!(Format::DateTime(Timezone::Utc, Precision::Nanoseconds).to_string_for(&ts), "2026-03-06 14:43:49.038023456");
		assert_eq!(Format::FileName(Timezone::Utc).to_string_for(&ts), "2026-03-06_14-43-49");
		assert_eq!(Format::Date(Timezone::Utc).to_string_for(&ts), "2026-03-06");
		assert_eq!(Format::Time(Timezone::Utc, Precision::Seconds).to_string_for(&ts), "14:43:49");
		assert_eq!(Format::Time(Timezone::Utc, Precision::Milliseconds).to_string_for(&ts), "14:43:49.038");
		assert_eq!(Format::Time(Timezone::Utc, Precision::Nanoseconds).to_string_for(&ts), "14:43:49.038023456");
		assert_eq!(Format::Rfc2822(Timezone::Utc).to_string_for(&ts), "Fri, 06 Mar 2026 14:43:49 +0000");
		assert_eq!(Format::Rfc3339(Timezone::Utc, Precision::Seconds).to_string_for(&ts), "2026-03-06T14:43:49Z");
		assert_eq!(Format::Rfc3339(Timezone::Utc, Precision::Milliseconds).to_string_for(&ts), "2026-03-06T14:43:49.038Z");
		assert_eq!(Format::Rfc3339(Timezone::Utc, Precision::Nanoseconds).to_string_for(&ts), "2026-03-06T14:43:49.038023456Z");
		assert_eq!(Format::Iso8601(Timezone::Utc, Precision::Seconds).to_string_for(&ts), "2026-03-06T14:43:49Z");
		assert_eq!(Format::Iso8601(Timezone::Utc, Precision::Milliseconds).to_string_for(&ts), "2026-03-06T14:43:49.038Z");
		assert_eq!(Format::Iso8601(Timezone::Utc, Precision::Nanoseconds).to_string_for(&ts), "2026-03-06T14:43:49.038023456Z");
		assert_eq!(Format::Rfc3164(Timezone::Utc).to_string_for(&ts), "Mar  6 14:43:49");
		assert_eq!(Format::HTTP.to_string_for(&ts), "Fri, 06 Mar 2026 14:43:49 GMT");
		assert_eq!(Format::Rfc7231.to_string_for(&ts), "Fri, 06 Mar 2026 14:43:49 GMT");
	}

	#[test]
	fn timestamp_as_local_string() {
		test_helpers::mocks::with_timezone("America/Montevideo", || {
			let ts = Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts");

			assert_eq!(Format::DateTime(Timezone::Local, Precision::Seconds).to_string_for(&ts), "2026-03-06 11:43:49 -0300");
			assert_eq!(Format::DateTime(Timezone::Local, Precision::Milliseconds).to_string_for(&ts), "2026-03-06 11:43:49.038 -0300");
			assert_eq!(Format::DateTime(Timezone::Local, Precision::Nanoseconds).to_string_for(&ts), "2026-03-06 11:43:49.038023456 -0300");
			assert_eq!(Format::FileName(Timezone::Local).to_string_for(&ts), "2026-03-06_11-43-49");
			assert_eq!(Format::Date(Timezone::Local).to_string_for(&ts), "2026-03-06");
			assert_eq!(Format::Time(Timezone::Local, Precision::Seconds).to_string_for(&ts), "11:43:49");
			assert_eq!(Format::Time(Timezone::Local, Precision::Milliseconds).to_string_for(&ts), "11:43:49.038");
			assert_eq!(Format::Time(Timezone::Local, Precision::Nanoseconds).to_string_for(&ts), "11:43:49.038023456");
			assert_eq!(Format::Rfc2822(Timezone::Local).to_string_for(&ts), "Fri, 06 Mar 2026 11:43:49 -0300");
			assert_eq!(Format::Rfc3339(Timezone::Local, Precision::Seconds).to_string_for(&ts), "2026-03-06T11:43:49-03:00");
			assert_eq!(Format::Rfc3339(Timezone::Local, Precision::Milliseconds).to_string_for(&ts), "2026-03-06T11:43:49.038-03:00");
			assert_eq!(Format::Rfc3339(Timezone::Local, Precision::Nanoseconds).to_string_for(&ts), "2026-03-06T11:43:49.038023456-03:00");
			assert_eq!(Format::Iso8601(Timezone::Local, Precision::Seconds).to_string_for(&ts), "2026-03-06T11:43:49-03:00");
			assert_eq!(Format::Iso8601(Timezone::Local, Precision::Milliseconds).to_string_for(&ts), "2026-03-06T11:43:49.038-03:00");
			assert_eq!(Format::Iso8601(Timezone::Local, Precision::Nanoseconds).to_string_for(&ts), "2026-03-06T11:43:49.038023456-03:00");
			assert_eq!(Format::Rfc3164(Timezone::Local).to_string_for(&ts), "Mar  6 11:43:49");
		});
	}

	#[test]
	fn timestamp_string_len_for() {
		test_helpers::mocks::with_timezone("America/Montevideo", || {
			for ts in [
				Timestamp::try_from((2026, 3, 6, 14, 43, 49, 38, 23456)).expect("invalid parts for year 2026"),
				Timestamp::try_from((9999, 12, 31, 23, 59, 59, 999, 999999)).expect("invalid parts for last day of year 9999"),
				Timestamp::try_from((10000, 1, 1)).expect("invalid parts for first day of year 10000"),
				Timestamp::try_from((31562, 9, 15, 6, 13, 55, 777, 8888)).expect("invalid parts for year 31562"),
				Timestamp::try_from((54732890, 1, 26, 22, 50, 11, 111, 222)).expect("invalid parts for year 54732890"),
			] {
				for format in [
					Format::Date(Timezone::Local),
					Format::Date(Timezone::Utc),
					Format::DateTime(Timezone::Local, Precision::Milliseconds),
					Format::DateTime(Timezone::Local, Precision::Nanoseconds),
					Format::DateTime(Timezone::Local, Precision::Seconds),
					Format::DateTime(Timezone::Utc, Precision::Milliseconds),
					Format::DateTime(Timezone::Utc, Precision::Nanoseconds),
					Format::DateTime(Timezone::Utc, Precision::Seconds),
					Format::FileName(Timezone::Local),
					Format::FileName(Timezone::Utc),
					Format::HTTP,
					Format::Iso8601(Timezone::Local, Precision::Milliseconds),
					Format::Iso8601(Timezone::Local, Precision::Nanoseconds),
					Format::Iso8601(Timezone::Local, Precision::Seconds),
					Format::Iso8601(Timezone::Utc, Precision::Milliseconds),
					Format::Iso8601(Timezone::Utc, Precision::Nanoseconds),
					Format::Iso8601(Timezone::Utc, Precision::Seconds),
					Format::Rfc2822(Timezone::Local),
					Format::Rfc2822(Timezone::Utc),
					Format::Rfc3164(Timezone::Local),
					Format::Rfc3164(Timezone::Utc),
					Format::Rfc3339(Timezone::Local, Precision::Milliseconds),
					Format::Rfc3339(Timezone::Local, Precision::Nanoseconds),
					Format::Rfc3339(Timezone::Local, Precision::Seconds),
					Format::Rfc3339(Timezone::Utc, Precision::Milliseconds),
					Format::Rfc3339(Timezone::Utc, Precision::Nanoseconds),
					Format::Rfc3339(Timezone::Utc, Precision::Seconds),
					Format::Rfc7231,
					Format::Time(Timezone::Local, Precision::Milliseconds),
					Format::Time(Timezone::Local, Precision::Nanoseconds),
					Format::Time(Timezone::Local, Precision::Seconds),
					Format::Time(Timezone::Utc, Precision::Milliseconds),
					Format::Time(Timezone::Utc, Precision::Nanoseconds),
					Format::Time(Timezone::Utc, Precision::Seconds),
					Format::Timestamp(Precision::Milliseconds),
					Format::Timestamp(Precision::Nanoseconds),
					Format::Timestamp(Precision::Seconds),
				] {
					assert_eq!(format.string_len_for(&ts), format.to_string_for(&ts).len(), "{} for {}", format, ts);
				}
			}
		})
	}

	#[test]
	fn timestamp_as_integer() {
		let ts = Timestamp::try_from((2026, 1, 29, 7, 43, 19, 134, 943903)).expect("invalid parts");

		assert_eq!(Format::Timestamp(Precision::Seconds).to_integer_for(&ts), Some(1769672599_u128));
		assert_eq!(Format::Timestamp(Precision::Milliseconds).to_integer_for(&ts), Some(1769672599134_u128));
		assert_eq!(Format::Timestamp(Precision::Nanoseconds).to_integer_for(&ts), Some(1769672599134943903_u128));
		assert_eq!(Format::DateTime(Timezone::Utc, Precision::Seconds).to_integer_for(&ts), None);
		assert_eq!(Format::DateTime(Timezone::Utc, Precision::Nanoseconds).to_integer_for(&ts), None);
		assert_eq!(Format::Time(Timezone::Utc, Precision::Seconds).to_integer_for(&ts), None);
		assert_eq!(Format::Rfc2822(Timezone::Utc).to_integer_for(&ts), None);
		assert_eq!(Format::Iso8601(Timezone::Local, Precision::Nanoseconds).to_integer_for(&ts), None);
		assert_eq!(Format::HTTP.to_integer_for(&ts), None);
		assert_eq!(Format::Rfc7231.to_integer_for(&ts), None);
	}

	#[test]
	fn format_is_integer() {
		assert!(Format::Timestamp(Precision::Seconds).is_integer());
		assert!(Format::Timestamp(Precision::Milliseconds).is_integer());
		assert!(Format::Timestamp(Precision::Nanoseconds).is_integer());
		assert!(!Format::DateTime(Timezone::Utc, Precision::Seconds).is_integer());
		assert!(!Format::DateTime(Timezone::Utc, Precision::Nanoseconds).is_integer());
		assert!(!Format::Time(Timezone::Utc, Precision::Seconds).is_integer());
		assert!(!Format::Rfc2822(Timezone::Utc).is_integer());
		assert!(!Format::Iso8601(Timezone::Utc, Precision::Milliseconds).is_integer());
		assert!(!Format::HTTP.is_integer());
		assert!(!Format::Rfc7231.is_integer());
	}
}
