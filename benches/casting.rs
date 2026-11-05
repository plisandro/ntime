use std::hint;
use std::io;

use divan::{Bencher, counter};
use ntime::{Format, FormatPrecision, FormatTimezone, Timestamp};

const BENCHMARK_LOG_ITEMS: usize = 10000;

fn main() {
	divan::main();
}

mod write {
	use super::*;

	mod io_empty {
		use super::*;

		#[inline]
		fn run(bench: Bencher, format: &Format) {
			let now = Timestamp::now();
			let mut sink = io::empty();

			bench
				.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
				.with_inputs(|| BENCHMARK_LOG_ITEMS)
				.bench_local_refs(|total| {
					for _ in 0..*total {
						hint::black_box(&now).write(&mut sink, format).expect("benchmark timestamp write failed");
					}
				});
		}

		#[divan::bench]
		fn nanoseconds(bench: Bencher) {
			run(bench, &Format::Timestamp(FormatPrecision::Nanoseconds));
		}

		#[divan::bench]
		fn utc(bench: Bencher) {
			run(bench, &Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds));
		}

		#[divan::bench]
		fn local(bench: Bencher) {
			run(bench, &Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds));
		}
	}

	mod mem_buffer {
		use super::*;

		#[inline]
		fn run(bench: Bencher, format: &Format) {
			let now = Timestamp::now();
			// buffer is given more more than enough capacity to host all timestamp formats, in order to avoid resizes.
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

		#[divan::bench]
		fn nanoseconds(bench: Bencher) {
			run(bench, &Format::Timestamp(FormatPrecision::Nanoseconds));
		}

		#[divan::bench]
		fn utc(bench: Bencher) {
			run(bench, &Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds));
		}

		#[divan::bench]
		fn local(bench: Bencher) {
			run(bench, &Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds));
		}
	}
}

mod to_string {
	use super::*;

	#[inline]
	fn run(bench: Bencher, format: &Format) {
		let now = Timestamp::now();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					let res = hint::black_box(&now).as_string(format);
					hint::black_box(res);
				}
			});
	}

	#[divan::bench]
	fn nanoseconds(bench: Bencher) {
		run(bench, &Format::Timestamp(FormatPrecision::Nanoseconds));
	}

	#[divan::bench]
	fn utc(bench: Bencher) {
		run(bench, &Format::DateTime(FormatTimezone::UTC, FormatPrecision::Milliseconds));
	}

	#[divan::bench]
	fn local(bench: Bencher) {
		run(bench, &Format::DateTime(FormatTimezone::Local, FormatPrecision::Milliseconds));
	}
}

mod as_integer {
	use super::*;

	#[inline]
	fn run(bench: Bencher, format: &Format) {
		let now = Timestamp::now();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					let res = hint::black_box(&now).as_integer(format);
					hint::black_box(res);
				}
			});
	}

	#[divan::bench]
	fn seconds(bench: Bencher) {
		run(bench, &Format::Timestamp(FormatPrecision::Seconds));
	}

	#[divan::bench]
	fn milliseconds(bench: Bencher) {
		run(bench, &Format::Timestamp(FormatPrecision::Milliseconds));
	}

	#[divan::bench]
	fn nanoseconds(bench: Bencher) {
		run(bench, &Format::Timestamp(FormatPrecision::Nanoseconds));
	}
}
