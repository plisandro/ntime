use std::error::Error;
use std::fmt;
use std::time;

/// Failed to convert a type into a NanoTime type.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CastError {
	/// A general conversion failure - f.ex. invalid values for date/time tuple.
	Invalid(String),
	/// Invalid [`TimestampParts`][crate::parts::TimestampParts] contents
	TimestampParts(String),
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

impl From<String> for CastError {
	fn from(err: String) -> Self {
		Self::Invalid(err)
	}
}

impl From<&str> for CastError {
	fn from(err: &str) -> Self {
		Self::Invalid(String::from(err))
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

impl fmt::Display for CastError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Invalid(s) => write!(f, "{}", s),
			Self::TimestampParts(s) => write!(f, "TimestampParts: {}", s),
			Self::SystemTime(e) => write!(f, "SystemTime: {}", e),
		}
	}
}
