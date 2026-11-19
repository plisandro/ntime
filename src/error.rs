use std::error::Error;
use std::fmt;
use std::time;

/// Failed to convert a type into a NanoTime type.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CastError {
	/// A general conversion failure - f.ex. invalid values for date/time tuple.
	Invalid(&'static str),
	/// Tried to cast from a value which is too large for the target.
	Overflow(&'static str),
	/// Tried to cast from a value which is too small for the target.
	Underflow(&'static str),
	/// Invalid [`TimestampParts`][crate::parts::TimestampParts] contents
	TimestampParts(&'static str),
	/// A failed conversion from [`SystemTime`][time::SystemTime].
	SystemTime(time::SystemTimeError),
}

impl Error for CastError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::SystemTime(e) => Some(e),
			_ => None,
		}
	}
}

impl From<time::SystemTimeError> for CastError {
	fn from(err: time::SystemTimeError) -> Self {
		Self::SystemTime(err)
	}
}

impl PartialEq for CastError {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Self::Invalid(ss) => match other {
				Self::Invalid(os) => ss == os,
				_ => false,
			},
			Self::Overflow(ss) => match other {
				Self::Overflow(os) => ss == os,
				_ => false,
			},
			Self::Underflow(ss) => match other {
				Self::Underflow(os) => ss == os,
				_ => false,
			},
			Self::TimestampParts(ss) => match other {
				Self::TimestampParts(os) => ss == os,
				_ => false,
			},
			Self::SystemTime(se) => match other {
				// apparently the only way to compare SystemTimeError's
				Self::SystemTime(oe) => se.duration() == oe.duration(),
				_ => false,
			},
		}
	}
}

impl Eq for CastError {}

impl fmt::Display for CastError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Invalid(s) => write!(f, "{}", s),
			Self::Overflow(s) => write!(f, "Overflow: {}", s),
			Self::Underflow(s) => write!(f, "Underflow: {}", s),
			Self::TimestampParts(s) => write!(f, "TimestampParts: {}", s),
			Self::SystemTime(e) => write!(f, "SystemTime: {}", e),
		}
	}
}

/// Errors for operations with [`Timestamp`][crate::timestamp::Timestamp]s.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimestampOpError {
	/// A general operation failure.
	Invalid(&'static str),
	/// Overflow - f.ex. while adding [`Timestamp`][crate::timestamp::Timestamp]s.
	Overflow(&'static str),
	/// Underflow - f.ex. while subtracting [`Timestamp`][crate::timestamp::Timestamp]s.
	Underflow(&'static str),
}

impl Error for TimestampOpError {}

impl fmt::Display for TimestampOpError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Invalid(s) => write!(f, "{}", s),
			Self::Overflow(s) => write!(f, "Overflow: {}", s),
			Self::Underflow(s) => write!(f, "Underflow: {}", s),
		}
	}
}

impl From<CastError> for TimestampOpError {
	fn from(err: CastError) -> Self {
		match err {
			CastError::Invalid(e) | CastError::TimestampParts(e) => Self::Invalid(e),
			CastError::SystemTime(_) => Self::Invalid("SystemTime error"),
			CastError::Overflow(e) => Self::Overflow(e),
			CastError::Underflow(e) => Self::Underflow(e),
		}
	}
}
