# Benchmarks 

## Integration test benchmarks

Basic benchmark tests, intended to gauge performance progress across versions, launched with
`cargo test --release --features=benchmark -- --show-output`.

All figures below were collected on 16-core AMD Ryzen 9 5950X system with 64GB of DDR4 memory.

### Latest: 2026-03-26, version 0.2.0

Initial release, (g)libc based implementation.

```
--- Benchmark: write serialized UTC timestamps ---
1000000 items in 21.124662ms, average 21ns/op

--- Benchmark: write serialized local timestamps ---
1000000 items in 49.085857ms, average 49ns/op

--- Benchmark: convert UTC timestamps to String ---
1000000 items in 205.405142ms, average 205ns/op

--- Benchmark: convert local timestamps to String ---
1000000 items in 258.136062ms, average 258ns/op
```
