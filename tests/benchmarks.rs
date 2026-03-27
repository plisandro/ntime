// TODO: switch over to 'cargo bench`, once that feature finally becomes stable >:(

// benchmarks should not be executed in parallel, so we declare them and
// launch them in sequence below.
#[cfg(all(test, feature = "benchmark"))]
mod benchmark {
	use std::io;

	use ntime::{Duration, StringFormat, Timestamp};

	const TOTAL_BENCHMARK_RUNS: u32 = 1000000;

	fn utc_write_serialization() -> (u32, Duration) {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			start.write(&mut io::empty(), &StringFormat::UtcMillisDateTime).expect("benchmar timestamp write failed");
		}

		(count, Timestamp::now() - start)
	}

	fn local_write_serialization() -> (u32, Duration) {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			start.write(&mut io::empty(), &StringFormat::LocalMillisDateTime).expect("benchmar timestamp write failed");
		}

		(count, Timestamp::now() - start)
	}

	fn utc_to_string_serialization() -> (u32, Duration) {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			let _ = start.as_string(&StringFormat::UtcMillisDateTime);
		}

		(count, Timestamp::now() - start)
	}

	fn local_to_string_serialization() -> (u32, Duration) {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			let _ = start.as_string(&StringFormat::LocalMillisDateTime);
		}

		(count, Timestamp::now() - start)
	}

	#[test]
	fn run() {
		struct Benchmark {
			name: String,
			func: fn() -> (u32, Duration),
		}

		let benchmarks: [Benchmark; _] = [
			Benchmark {
				name: "write serialized UTC timestamps".into(),
				func: utc_write_serialization,
			},
			Benchmark {
				name: "write serialized local timestamps".into(),
				func: local_write_serialization,
			},
			Benchmark {
				name: "convert UTC timestamps to String".into(),
				func: utc_to_string_serialization,
			},
			Benchmark {
				name: "convert local timestamps to String".into(),
				func: local_to_string_serialization,
			},
		];

		for b in benchmarks {
			println!("--- Benchmark: {name} ---", name = b.name);
			let (total, runtime) = (b.func)();
			println!("{total} items in {runtime:?}, average {avg:?}/op\n", avg = runtime / total);
		}
	}
}
