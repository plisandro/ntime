use std::io;

use ntime::StringFormat;
use ntime::Timestamp;

#[cfg(test)]
mod benchmarks {
	use super::*;

	const TOTAL_BENCHMARK_RUNS: u32 = 1000000;

	#[test]
	fn write_serialization() {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;
		//let mut black_hole = io::empty();

		for _ in 0..count {
			start.write(&mut io::empty(), &StringFormat::LocalMillisDateTime).expect("benchmar timestamp write failed");
		}

		let elapsed = Timestamp::now() - start;
		println!("wrote {count} serialized timestamps in {elapsed:?}, average {avg:?}/op", avg = elapsed / count,);
	}

	#[test]
	fn to_string_serialization() {
		let start = Timestamp::now();
		let count = TOTAL_BENCHMARK_RUNS;

		for _ in 0..count {
			let _ = start.as_string(&StringFormat::UtcMillisDateTime);
		}

		let elapsed = Timestamp::now() - start;
		println!("serialized {count} timestamp into string in {elapsed:?}, average {avg:?}/op", avg = elapsed / count,);
	}
}
