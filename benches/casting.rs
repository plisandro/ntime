use std::io;

use divan::{Bencher, counter};
use ntime::{Format, Timestamp};

const BENCHMARK_LOG_ITEMS: usize = 10000;

fn main() {
	divan::main();
}

mod writer {
	use super::*;

	#[divan::bench]
	fn nanoseconds(bench: Bencher) {
		let start = Timestamp::now();
		let mut sink = io::empty();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					start.write(&mut sink, &Format::TimestampNanoseconds).expect("benchmark timestamp write failed");
				}
			});
	}

	#[divan::bench]
	fn utc(bench: Bencher) {
		let start = Timestamp::now();
		let mut sink = io::empty();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					start.write(&mut sink, &Format::UtcMillisDateTime).expect("benchmark timestamp write failed");
				}
			});
	}

	#[divan::bench]
	fn local(bench: Bencher) {
		let start = Timestamp::now();
		let mut sink = io::empty();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					start.write(&mut sink, &Format::LocalMillisDateTime).expect("benchmark timestamp write failed");
				}
			});
	}
}

mod to_string {
	use super::*;

	#[divan::bench]
	fn nanoseconds_to_string(bench: Bencher) {
		let start = Timestamp::now();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					_ = start.as_string(&Format::TimestampNanoseconds);
				}
			});
	}

	#[divan::bench]
	fn utc_to_string(bench: Bencher) {
		let start = Timestamp::now();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					_ = start.as_string(&Format::UtcMillisDateTime);
				}
			});
	}

	#[divan::bench]
	fn local_to_string(bench: Bencher) {
		let start = Timestamp::now();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					_ = start.as_string(&Format::LocalMillisDateTime);
				}
			});
	}
}

mod as_integer {
	use super::*;

	#[divan::bench]
	fn as_integer(bench: Bencher) {
		let start = Timestamp::now();

		bench
			.counter(counter::ItemsCount::new(BENCHMARK_LOG_ITEMS))
			.with_inputs(|| BENCHMARK_LOG_ITEMS)
			.bench_local_refs(|total| {
				for _ in 0..*total {
					_ = start.as_integer(&Format::TimestampNanoseconds);
				}
			});
	}
}
