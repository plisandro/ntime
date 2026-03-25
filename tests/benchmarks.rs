// TODO: switch over to 'cargo bench`, once that feature finally becomes stable >:(

// benchmarks should not be executed in parallel, so we declare them here, and
// launch them in sequence below.
#[cfg(test)]
mod benchmark {
	use std::io;

	use ntime::{Duration, StringFormat, Timestamp};

	const TOTAL_BENCHMARK_RUNS: u32 = 1000000;

	fn write_serialization() -> (u32, Duration) {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			start.write(&mut io::empty(), &StringFormat::LocalMillisDateTime).expect("benchmar timestamp write failed");
		}

		(count, Timestamp::now() - start)
	}

	fn to_string_serialization() -> (u32, Duration) {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			let _ = start.as_string(&StringFormat::UtcMillisDateTime);
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
				name: "write serialized timestamps".into(),
				func: write_serialization,
			},
			Benchmark {
				name: "convert timestamps to String".into(),
				func: to_string_serialization,
			},
		];

		for b in benchmarks {
			println!("--- Benchmark: {name} ---", name = b.name);
			let (total, runtime) = (b.func)();
			println!("{total} items in {runtime:?}, average {avg:?}/op\n", avg = runtime / total);
		}
	}
}
