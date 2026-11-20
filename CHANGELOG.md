# Changelog

A list of important changes for relevant `ntime` releases.

## Version 1.0.0 (2026-11-22)

First major release 🙌🏻

  - New and improved public API: fallible type conversions with dedicated error
    types, a cleaner `Format` structure and much more consistent (and saner) method
    naming.
  - New `std::fmt`-less timestamp serializer. This **massively** improves `Format` conversion
    performance, with a throughput boost of 1.31x to 7.03x _(!!!)_ depending on the `Format`,
    and a mean of ~3.72x.
  - Formalize (and document) local TZ conversion limits for different platforms and (g)libc
    implementations.
  - NanoTime is now distributed under either the MIT or Apache 2.0 licence.
  - `Format::string_len()` now yields correct results for years 10,000 and higher.
  - New `TimestampParts::leap_years_since_epoch()` method.
  - Fix bug in RFC 7231 timestamp formatter.
  - Fix a formatting bug for RFC 3339 timestamps on local TZ.

## Version 0.8.0 (2026-11-01)

  - Multiple bugfixes.
  - `Timestamp` subtraction returns `Err` on underflow.

## Version 0.7.1 (2026-06-24)

Implement `Timestamp::copy_from()` and `Timestamp::string_len()`.

## Version 0.6.0 (2026-05-21)

Full Windows support, with timezone GMT offset and name resolution.

## Version 0.5.0 (2026-05-11)

  - Add multiple precisions for RFC 3339 datetime strings.
  - Add support for RFC 3164 syslog datetime strings.
  - Add a number of convenience methods for `Format` (f.ex. `is_integer()`).
  - Switch benchmarking to Divan.

## Version 0.4.0 (2026-04-06)

Make `TimestampParts` a publicly exported symbol, convert parts to timestamps without (g)libc.

## Version 0.3.0 (2026-04-02)

Improved formatting options, benchmarks and unit tests.

## Version 0.2.0 (2026-03-26)

First public-ready release.
