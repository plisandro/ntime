//! NanoTime is a lightweight, high performance Rust library for nanosecond-precision timestamps.
//! It offers support for timestamp generation, arithmetics, comparsion and casting to various string
//! representations, in either local or UTC timezones.
//!
//! NanoTime is optimized for performance - it has no external dependencies, and relies solely
//! on (g)libc to resolve date information from timestamps. As a result, it has also a limited
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
//!   * A UTC date & time: [`Timestamp::from_utc_date()`]
//!   * From a timestamp in milliseconds/nanoseconds: [`Timestamp::new()`], [`Timestamp::from_secs()`], [`Timestamp::from_millis()`], [`Timestamp::from_nanos()`].
//!
//! [`Timestamp`]s can efficiently be converted into a text representation, with multiple formats supported.
//!
//! [`Timestamp`] supports comparison, and a limited set of aritmetic operations (difference, for example), which
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
//! use ntime::{Format, Timestamp};
//!
//! let now = Timestamp::now();
//! println!("nanos since epoch: {}", now.as_nanos());
//! println!("to_string:         {}", now.to_string());
//! println!("HTTP/1.1 (UTC):    {}", now.as_string(&Format::UtcRFC7231));
//! ```
//! ```text
//! nanos since epoch:  1774369621732000558
//! to_string:          2026-03-24 17:21:01 +0100
//! HTTP/1.1 (UTC):     Tue, 24 Mar 2026 16:27:01 UTC
//! ```
//!
//! ## Timestamp arithmetics
//!
//! ```rust
//! use ntime::Timestamp;
//!
//! let a = Timestamp::from_utc_date(2026, 03, 24, 17, 44, 48, 123, 456);
//! let b = Timestamp::from_utc_date(2026, 03, 24, 17, 25, 30, 789, 012);
//! dbg!(a - b);
//! ```
//! ```text
//! a - b = 1157.334000444s
//! ```
//! ## Sleeping
//!
//! ```rust
//! use ntime;
//!
//! // let's be lazy
//! ntime::sleep_millis(1500);
//! ```
//!
//! # Limitations and caveats
//!
//!   * As noted, NanoTime is not well suited for applications requiring calendar operations, and/or flexible timezone management.
//!   * Windows support is currently partial, lacking string conversion support for local timezones.
//!

#![deny(missing_docs)]
#![allow(dead_code)]

mod c_bindings;
mod constant;
mod sleep;
#[cfg(test)]
mod test_helpers;
mod timestamp;

use std::time;

// Public exported symbols.
pub use sleep::*;
pub use time::Duration;
pub use timestamp::Format;
pub use timestamp::Timestamp;
