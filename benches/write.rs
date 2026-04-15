use std::io;

use divan::Bencher;
use ntime::{Format, Timestamp};

fn main() {
	divan::main();
}

#[divan::bench]
fn nanoseconds(bench: Bencher) {
	let start = Timestamp::now();
	let mut sink = io::empty();

	bench.bench_local(|| {
		start.write(&mut sink, &Format::TimestampNanoseconds).expect("benchmark timestamp write failed");
	});
}

#[divan::bench]
fn utc(bench: Bencher) {
	let start = Timestamp::now();
	let mut sink = io::empty();

	bench.bench_local(|| {
		start.write(&mut sink, &Format::UtcMillisDateTime).expect("benchmark timestamp write failed");
	});
}

#[divan::bench]
fn local(bench: Bencher) {
	let start = Timestamp::now();
	let mut sink = io::empty();

	bench.bench_local(|| {
		start.write(&mut sink, &Format::LocalMillisDateTime).expect("benchmark timestamp write failed");
	});
}

#[divan::bench]
fn nanoseconds_to_string(bench: Bencher) {
	let start = Timestamp::now();

	bench.bench_local(|| {
		_ = start.as_string(&Format::TimestampNanoseconds);
	});
}

#[divan::bench]
fn utc_to_string(bench: Bencher) {
	let start = Timestamp::now();

	bench.bench_local(|| {
		_ = start.as_string(&Format::UtcMillisDateTime);
	});
}

#[divan::bench]
fn local_to_string(bench: Bencher) {
	let start = Timestamp::now();

	bench.bench_local(|| {
		_ = start.as_string(&Format::LocalMillisDateTime);
	});
}

#[divan::bench]
fn as_integer(bench: Bencher) {
	let start = Timestamp::now();

	bench.bench_local(|| {
		_ = start.as_integer(&Format::TimestampNanoseconds);
	});
}
