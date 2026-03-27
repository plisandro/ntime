# Benchmarks 

## Integration test benchmarks

Basic benchmark tests, intended to gauge performance progress across versions, launched with
`cargo test --release --features=benchmark -- --show-output`.

All figures below were collected on 16-core AMD Ryzen 9 5950X system with 64GB of DDR4 memory.

### Latest: 2026-03-26, version 0.2.0

Initial release, (g)libc based implementation.

```
--- Benchmark: write serialized nanoseconds timestamps ---
1000000 items in 2.041761ms, average 2ns/op

--- Benchmark: write serialized UTC timestamps ---
1000000 items in 19.862931ms, average 19ns/op

--- Benchmark: write serialized local timestamps ---
1000000 items in 47.459056ms, average 47ns/op

--- Benchmark: convert UTC timestamps to String ---
1000000 items in 207.324203ms, average 207ns/op

--- Benchmark: convert local timestamps to String ---
1000000 items in 266.320416ms, average 266ns/op
```
