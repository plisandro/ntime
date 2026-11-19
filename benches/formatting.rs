use std::hint;

use divan::{Bencher, counter};
use ntime::{Format, Precision, Timestamp, Timezone};

const BENCHMARK_LOG_ITEMS: usize = 10000;

const NUMERIC_FORMATS: &[Format] = &[
	Format::Timestamp(Precision::Seconds),
	Format::Timestamp(Precision::Milliseconds),
	Format::Timestamp(Precision::Nanoseconds),
];

const UTC_FORMATS: &[Format] = &[
	Format::DateTime(Timezone::Utc, Precision::Seconds),
	Format::DateTime(Timezone::Utc, Precision::Milliseconds),
	Format::DateTime(Timezone::Utc, Precision::Nanoseconds),
	Format::Date(Timezone::Utc),
	Format::Time(Timezone::Utc, Precision::Seconds),
	Format::Time(Timezone::Utc, Precision::Milliseconds),
	Format::Time(Timezone::Utc, Precision::Nanoseconds),
	Format::FileName(Timezone::Utc),
	Format::Rfc2822(Timezone::Utc),
	Format::Rfc3164(Timezone::Utc),
	Format::Rfc3339(Timezone::Utc, Precision::Seconds),
	Format::Rfc3339(Timezone::Utc, Precision::Milliseconds),
	Format::Rfc3339(Timezone::Utc, Precision::Nanoseconds),
	Format::Iso8601(Timezone::Utc, Precision::Seconds),
	Format::Iso8601(Timezone::Utc, Precision::Milliseconds),
	Format::Iso8601(Timezone::Utc, Precision::Nanoseconds),
	Format::Rfc7231,
];

const LOCAL_FORMATS: &[Format] = &[
	Format::DateTime(Timezone::Local, Precision::Seconds),
	Format::DateTime(Timezone::Local, Precision::Milliseconds),
	Format::DateTime(Timezone::Local, Precision::Nanoseconds),
	Format::Date(Timezone::Local),
	Format::Time(Timezone::Local, Precision::Seconds),
	Format::Time(Timezone::Local, Precision::Milliseconds),
	Format::Time(Timezone::Local, Precision::Nanoseconds),
	Format::FileName(Timezone::Local),
	Format::Rfc2822(Timezone::Local),
	Format::Rfc3164(Timezone::Local),
	Format::Rfc3339(Timezone::Local, Precision::Seconds),
	Format::Rfc3339(Timezone::Local, Precision::Milliseconds),
	Format::Rfc3339(Timezone::Local, Precision::Nanoseconds),
	Format::Iso8601(Timezone::Local, Precision::Seconds),
	Format::Iso8601(Timezone::Local, Precision::Milliseconds),
	Format::Iso8601(Timezone::Local, Precision::Nanoseconds),
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
