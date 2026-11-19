//! NanoTime is a lightweight, high performance Rust library for nanosecond-precision timestamps.
//! It offers support for timestamp generation, arithmetic, comparison and casting to various string
//! representations, in either local or UTC timezones.
//!
//! NanoTime is optimized for performance - it has no external dependencies, and relies
//! on (g)libc to resolve local date information from timestamps. As a result, it has also a limited
//! scope; if you happen to need features such as date/time management, timezone conversion or
//! calendar operations, you're likely better off using [Chrono](https://docs.rs/chrono/latest/chrono/) or similar.
//!
//! # Features
//!
//! NanoTime introduces the [`Timestamp`] struct, which encapsulates a timestamp as nanoseconds
//! since Unix epoch (1970-01-01 00:00 UTC).
//!
//! [`Timestamp`]s can be very efficiently converted into a text representation, with multiple
//! [`Format`]s supported, and split into date + time [`TimestampParts`].
//!
//! # Usage examples
//!
//! ## Initialization
//!
//!  [`Timestamp`]s can be instantiated by...
//!
//!   * Current time: [`Timestamp::now()`]
//!   * A standard [`std::time::SystemTime`], via [`Timestamp::try_from()`].
//!   * A UTC date & time tuple: [`Timestamp::try_from()`] with `(year, month, day[, hour, minutes, seconds[, millis, nanos]])`.
//!   * A dedicated [`UtcDateTime`] struct, via [`Timestamp::try_from()`].
//!   * From a timestamp in milliseconds/nanoseconds: [`Timestamp::new()`], [`Timestamp::from_secs()`], [`Timestamp::from_millis()`], [`Timestamp::from_nanos()`].
//!   * From a [`TimestampParts`] struct: [`Timestamp::from_parts()`]
//!
//! ### From Current Time
//!
//!  ```rust
//! use ntime::Timestamp;
//!
//! let now = Timestamp::now();
//! dbg!(now);
//! ```
//! ```text
//! Timestamp { seconds: 1774369621, nanoseconds: 732000558 }
//! ```
//!
//! ### From UTC `(year, month, day[, hour, minutes, seconds[, millis, nanos]])` Tuples
//!
//!  ```rust
//! use ntime::Timestamp;
//!
//! let ts = Timestamp::try_from((2026, 11, 18, 22, 17, 19)).expect("invalid date/time values");
//! println!("initialized from date/time values to {ts}");
//! ```
//! ```text
//! initialized from date/time values to 2026-11-18 23:17:19 +0100
//! ```
//!
//! ### From [`UtcDateTime`]
//!
//!  ```rust
//! use ntime::{Timestamp, UtcDateTime};
//!
//! let udt = UtcDateTime {
//!     year: 2026,
//!     month: 3,
//!     day: 24,
//!     hour: 17,
//!     minutes: 44,
//!     seconds: 48,
//!     milliseconds: 123,
//!     nanoseconds: 456,
//! };
//!
//! let ts = Timestamp::try_from(&udt).expect("invalid UtcDateTime");
//! println!("{udt:?} converts to {ts}");
//! ```
//! ```text
//! UtcDateTime { year: 2026, month: 3, day: 24, hour: 17, minutes: 44, seconds: 48, milliseconds: 123, nanoseconds: 456 } converts to 2026-03-24 18:44:48 +0100
//! ```
//!
//! ## String conversion and casting
//!
//! NanoTime can serialize [`Timestamp`]s to a [`io::Write`][std::io::Write] or [`String`], with configurable [`Format`]ting.
//!
//! ```rust
//! use ntime::{Format, Precision, Timezone, Timestamp};
//!
//! let now = Timestamp::now();
//! println!("nanos since epoch: {}", now.as_nanos());
//! println!("debug:             {:?}", now);
//! println!("to_string():       {}", now.to_string());
//! println!("local time:        {}", now.as_string(&Format::DateTime(Timezone::Local, Precision::Milliseconds)));
//! println!("UTC time:          {}", now.as_string(&Format::DateTime(Timezone::Utc, Precision::Nanoseconds)));
//! println!("HTTP/1.1:          {}", now.as_string(&Format::Rfc7231));
//! println!("local week day     {}", now.as_local_parts().week_day);
//! println!("UTC year day       {}", now.as_utc_parts().year_day);
//! ```
//! ```text
//! nanos since epoch: 1774369621732000558
//! debug:             Timestamp { seconds: 1774369621, nanoseconds: 732000558 }
//! to_string():       2026-03-24 16:27:01.732
//! local time:        2026-03-24 17:27:01.732 +0100
//! UTC time:          2026-03-24 16:27:01.732000558 +0000
//! HTTP/1.1:          Tue, 24 Mar 2026 16:27:01 GMT
//! local week day:    3
//! UTC year day:      83
//! ```
//!
//! ## Timestamp Arithmetic
//!
//! [`Timestamp`] supports comparisons, and a limited set of arithmetic operations.
//!
//! ### Addition
//!
//! Adding a [`std::time::Duration`] to a [`Timestamp`] yields a new [`Timestamp`]. Note that
//! [`std::time::Duration`] is re-exported in the module's namespace for convenience.
//!
//! ```rust
//! use ntime::{Timestamp, Duration};
//!
//! let a = Timestamp::try_from((2026, 3, 24, 17, 44, 48, 123, 456)).expect("invalid parameters for timestamp");
//! let delta = Duration::from_hours(10);
//!
//! println!("[{a}] + {delta:?} = {}", a.add_duration(&delta).unwrap());
//! ```
//! ```text
//! [2026-03-24 17:44:48.123] + 36000s = 2026-03-25 03:44:48.123
//! ```
//!
//! ### Difference
//!
//! Subtracting [`Timestamp`]s yields a [`std::time::Duration`].
//!
//! ```rust
//! use ntime::Timestamp;
//!
//! let a = Timestamp::try_from((2026, 3, 24, 17, 44, 48, 123, 456)).expect("invalid parameters for timestamp A");
//! let b = Timestamp::try_from((2026, 3, 24, 17, 25, 30, 789, 12)).expect("invalid parameters for timestamp B");
//! println!("from {a} to {b} = {:?}", (a - b).unwrap());
//! ```
//! ```text
//! from 2026-03-24 17:44:48.123 to 2026-03-24 17:25:30.789 = 1157.334000444s
//! ```
//!
//! ## Parts Decomposition
//!
//! [`Timestamp`]s can be decomposed into date + time [`TimestampParts`], in both
//! [local][Timezone::Local] and [UTC][Timezone::Utc] timezones.
//!
//! ```rust
//! use ntime::Timestamp;
//!
//! let now = Timestamp::now();
//! let now_parts = now.as_utc_parts();
//!
//! println!(
//!     "today is {day_name} the {month_day}th of {month_name} {year}, \
//!     day {year_day} of the year in UTC",
//!     day_name = now_parts.day_name(),
//!     month_day = now_parts.month_day,
//!     month_name = now_parts.month_name(),
//!     year = now_parts.year,
//!     year_day = now_parts.year_day
//! );
//! ```
//! ```text
//! today is Mon the 6th of Apr 2026, day 96 of the year in UTC
//! ```
//!
//! ## Other
//!
//! NanoTime also provides convenience functionality not strictly related to timestamps.
//!
//! ### Sleeping
//!
//! ```rust
//! use ntime;
//!
//! // let's be lazy
//! ntime::sleep_millis(1500);
//! ```
//!
//! # Limitations
//!
//! ## Timestamp Ranges
//!
//! [`Timestamp`]s are internally stored as a duration since UNIX epoch, with a supported range of:
//!   - [Minimum time][Timestamp::MIN]: `1970-01-01 00:00:00.000000000 UTC` (UNIX epoch).
//!   - [Maximum time][Timestamp::MAX]: `584554051223-11-09 07:00:15.999999999 UTC`.
//!
//! ## Parts Conversion
//!
//! Conversion to [`TimestampParts`], and [`Format`]ting, are limited by the choice of [`Timezone`]
//! for the operation. Attempting to perform these on unsupported [`Timestamp`] values will
//! panic at runtime.
//!
//! ### [UTC][Timezone::Utc]
//!
//! For UTC, [`TimestampParts`] conversions are fully resolved with in-library logic, and
//! support all possible [`Timestamp`] values.
//!
//! ### [Local][Timezone::Local]
//!
//! Local timezone conversions are resolved via (g)libc calls, with type limitations imposed
//! by the C FFI. The maximum [`Timestamp`] which can be decomposed into [`TimestampParts`] is
//! then determined by the libc implementation and/or system architecture.
//!
//!   - 64-bit systems, and 32-bit systems with 64-bit libc time implementations (f.ex. [musl](https://musl.libc.org/)): `2147485547-12-31 23:59:59.999999999 +0000`
//!   - 32-bit systems: `2038-01-19 03:14:07.999999999 UTC`.
//!
//! These limitations apply also to formatting, so [`Timestamp`]s over these boundaries
//! will panic if serialized as strings.
//!
//! ## Error Handling
//!
//! A number of [`Timestamp`] operations return errors:
//!
//!   - All casting operations from types which are fallible (i.e. implementing `TryFrom`) return `CastError` upon
//!     failure - f.ex. invalid values in [`UtcDateTime`], or timestamps in milliseconds which are too big.
//!   - Arithmetic operations for [`Timestamp`]s (f.ex. adding a [`Duration`]) return `TimestampOpError`
//!     upon failure.
//!
//! ## Caveats
//!
//! As noted, NanoTime is not well suited for applications requiring calendar operations,
//! and/or flexible timezone management.
//!

#![deny(missing_docs)]
#![deny(dead_code)]

#[cfg_attr(target_env = "msvc", path = "c_bindings_win.rs")]
#[cfg_attr(not(target_env = "msvc"), path = "c_bindings_unix.rs")]
mod c_bindings;
mod constant;
mod error;
mod format;
mod parts;
mod sleep;
#[cfg(test)]
mod test_helpers;
mod timestamp;
mod write;

// Public exported symbols.
pub use error::{CastError, TimestampOpError};
pub use format::{Format, Precision, Timezone};
pub use parts::TimestampParts;
pub use sleep::*;
pub use std::time::Duration;
pub use timestamp::{Timestamp, UtcDateTime};
