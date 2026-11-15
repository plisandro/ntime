use std::hint;

use divan::{Bencher, counter};
use ntime::{Format, FormatPrecision, FormatTimezone, Timestamp};

const BENCHMARK_LOG_ITEMS: usize = 10000;

const NUMERIC_FORMATS: &[Format] = &[
	Format::Timestamp(FormatPrecision::Seconds),
	Format::Timestamp(FormatPrecision::Milliseconds),
	Format::Timestamp(FormatPrecision::Nanoseconds),
];

const UTC_FORMATS: &[Format] = &[
	Format::DateTime(FormatTimezone::UTC, FormatPrecision::Seconds),
	Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds),
	Format::DateTime(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
	Format::Date(FormatTimezone::UTC),
	Format::Time(FormatTimezone::UTC, FormatPrecision::Seconds),
	Format::Time(FormatTimezone::UTC, FormatPrecision::Milliseconds),
	Format::Time(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
	Format::FileName(FormatTimezone::UTC),
	Format::RFC2822(FormatTimezone::UTC),
	Format::RFC3164(FormatTimezone::UTC),
	Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Seconds),
	Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Milliseconds),
	Format::RFC3339(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
	Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Seconds),
	Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Milliseconds),
	Format::ISO8601(FormatTimezone::UTC, FormatPrecision::Nanoseconds),
	// Format::HTTP,
	Format::RFC7231,
];

const LOCAL_FORMATS: &[Format] = &[
	Format::DateTime(FormatTimezone::Local, FormatPrecision::Seconds),
	Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds),
	Format::DateTime(FormatTimezone::Local, FormatPrecision::Nanoseconds),
	Format::Date(FormatTimezone::Local),
	Format::Time(FormatTimezone::Local, FormatPrecision::Seconds),
	Format::Time(FormatTimezone::Local, FormatPrecision::Milliseconds),
	Format::Time(FormatTimezone::Local, FormatPrecision::Nanoseconds),
	Format::FileName(FormatTimezone::Local),
	Format::RFC2822(FormatTimezone::Local),
	Format::RFC3164(FormatTimezone::Local),
	Format::RFC3339(FormatTimezone::Local, FormatPrecision::Seconds),
	Format::RFC3339(FormatTimezone::Local, FormatPrecision::Milliseconds),
	Format::RFC3339(FormatTimezone::Local, FormatPrecision::Nanoseconds),
	Format::ISO8601(FormatTimezone::Local, FormatPrecision::Seconds),
	Format::ISO8601(FormatTimezone::Local, FormatPrecision::Milliseconds),
	Format::ISO8601(FormatTimezone::Local, FormatPrecision::Nanoseconds),
];

fn main() {
	divan::main();
}

mod utc {
	use super::*;

	#[inline]
	fn run(bench: Bencher, format: &Format) {
		let now = Timestamp::now();
		// buffer is given more than enough capacity to host all timestamp formats, in order to avoid resizes.
		let mut buffer: Vec<u8> = Vec::with_capacity(128);

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					hint::black_box(&now).write(&mut buffer, format).expect("benchmark timestamp write failed");
					buffer.clear();
				}
			});
	}

	#[divan::bench(args=NUMERIC_FORMATS)]
	fn numeric(bench: Bencher, format: &Format) {
		run(bench, format);
	}

	#[divan::bench(args=UTC_FORMATS)]
	fn utc(bench: Bencher, format: &Format) {
		run(bench, format);
	}

	#[divan::bench(args=LOCAL_FORMATS)]
	fn local(bench: Bencher, format: &Format) {
		run(bench, format);
	}
}
