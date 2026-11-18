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
//! since Unix epoch (1970-01-01 00:00 UTC). [`Timestamp`]s can be instantiated by...
//!
//!   * Current time: [`Timestamp::now()`]
//!   * A standard [`std::time::SystemTime`]: [`Timestamp::from_system_time()`]
//!   * A UTC date & time: [`Timestamp::try_from()`] with a `(year, month, day[, hour, minutes, seconds[, millis, nanos]])` tuple, or a [`UTCDateTime`]
//!   * From a timestamp in milliseconds/nanoseconds: [`Timestamp::new()`], [`Timestamp::from_secs()`], [`Timestamp::from_millis()`], [`Timestamp::from_nanos()`].
//!   * From a [`TimestampParts`] struct: [`Timestamp::from_parts()`]
//!
//! [`Timestamp`]s can efficiently be converted into a text representation, with multiple [`Format`]s supported,
//! and split into date + time [`TimestampParts`].
//!
//! [`Timestamp`] supports comparison, and a limited set of arithmetic operations (difference, for example), which
//! return a [`std::time::Duration`]; this type is re-exported in the module's namespace for convenience.
//!
//! Finally, NanoTime supports a number of convenience tools such as [`sleep()`].
//!
//! # Usage examples
//!
//! ## Initialization
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
//! ## String conversion and casting
//!
//! ```rust
//! use ntime::{Format, Precision, Timezone, Timestamp};
//!
//! let now = Timestamp::now();
//! println!("nanos since epoch: {}", now.as_nanos());
//! println!("debug:             {:?}", now);
//! println!("to_string():       {}", now.to_string());
//! println!("local time:        {}", now.as_string(&Format::DateTime(Timezone::Local, Precision::Milliseconds)));
//! println!("UTC time:          {}", now.as_string(&Format::DateTime(Timezone::UTC, Precision::Nanoseconds)));
//! println!("HTTP/1.1:          {}", now.as_string(&Format::RFC7231));
//! println!("local week day     {}", now.as_local_parts().week_day);
//! println!("UTC year day       {}", now.as_utc_parts().year_day);
//! ```
//! ```text
//! nanos since epoch: 1774369621732000558
//! debug:             Timestamp { seconds: 1774369621, nanoseconds: 732000558 }
//! to_string():       2026-03-24 16:27:01 +0000
//! local time:        2026-03-24 17:27:01.732 +0100
//! UTC time:          2026-03-24 16:27:01.732000558 +0000
//! HTTP/1.1:          Tue, 24 Mar 2026 16:27:01 GMT
//! local week day:    3
//! UTC year day:      83
//! ```
//!
//! ## Timestamp Arithmetic
//!
//! ```rust
//! use ntime::Timestamp;
//!
//! let a = Timestamp::try_from((2026, 3, 24, 17, 44, 48, 123, 456)).expect("invalid parameters");
//! let b = Timestamp::try_from((2026, 3, 24, 17, 25, 30, 789, 12)).expect("invalid parameters");
//! _ = dbg!(a - b);
//! ```
//! ```text
//! a - b = Ok(
//!     1157.334000444s,
//! )
//! ```
//!
//! ## Parts Decomposition
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
//! ## Sleeping
//!
//! ```rust
//! use ntime;
//!
//! // let's be lazy
//! ntime::sleep_millis(1500);
//! ```
//!
//! # Limitations and Caveats
//!
//!   * As noted, NanoTime is not well suited for applications requiring calendar operations, and/or flexible timezone management.
//!

#![deny(missing_docs)]
#![deny(dead_code)]

#[cfg_attr(target_env = "msvc", path = "c_bindings_win.rs")]
#[cfg_attr(not(target_env = "msvc"), path = "c_bindings_unix.rs")]
mod c_bindings;
mod constant;
mod format;
mod parts;
mod sleep;
#[cfg(test)]
mod test_helpers;
mod timestamp;
mod write;

// Public exported symbols.
pub use format::{Format, Precision, Timezone};
pub use parts::TimestampParts;
pub use sleep::*;
pub use std::time::Duration;
pub use timestamp::{Timestamp, UTCDateTime};
