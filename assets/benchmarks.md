# Benchmarks 

## Integration test benchmarks

Basic benchmark tests, intended to gauge performance progress across versions, launched with
`cargo bench --profile=release`.

All figures below were collected on 16-core AMD Ryzen 9 5950X system with 64GB of DDR4 memory.

## Version 0.5.0 (2026-05-11)

```
Timer precision: 20 ns
casting                      fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ as_integer                              │               │               │               │         │
│  ╰─ as_integer             14.51 µs      │ 24.47 µs      │ 14.58 µs      │ 14.8 µs       │ 100     │ 100
│                            688.7 Mitem/s │ 408.5 Mitem/s │ 685.4 Mitem/s │ 675.6 Mitem/s │         │
├─ to_string                               │               │               │               │         │
│  ├─ local_to_string        2.608 ms      │ 3.506 ms      │ 2.674 ms      │ 2.702 ms      │ 100     │ 100
│  │                         3.833 Mitem/s │ 2.851 Mitem/s │ 3.739 Mitem/s │ 3.699 Mitem/s │         │
│  ├─ nanoseconds_to_string  330.9 µs      │ 452.2 µs      │ 342 µs        │ 349 µs        │ 100     │ 100
│  │                         30.21 Mitem/s │ 22.11 Mitem/s │ 29.23 Mitem/s │ 28.65 Mitem/s │         │
│  ╰─ utc_to_string          1.83 ms       │ 2.169 ms      │ 1.885 ms      │ 1.9 ms        │ 100     │ 100
│                            5.463 Mitem/s │ 4.609 Mitem/s │ 5.303 Mitem/s │ 5.261 Mitem/s │         │
╰─ writer                                  │               │               │               │         │
   ├─ local                  463.2 µs      │ 521.8 µs      │ 471.8 µs      │ 475.2 µs      │ 100     │ 100
   │                         21.58 Mitem/s │ 19.16 Mitem/s │ 21.19 Mitem/s │ 21.04 Mitem/s │         │
   ├─ nanoseconds            14.5 µs       │ 31.12 µs      │ 14.51 µs      │ 14.84 µs      │ 100     │ 100
   │                         689.1 Mitem/s │ 321.2 Mitem/s │ 688.7 Mitem/s │ 673.6 Mitem/s │         │
   ╰─ utc                    187.7 µs      │ 239.8 µs      │ 188.8 µs      │ 191.3 µs      │ 100     │ 100
                             53.25 Mitem/s │ 41.69 Mitem/s │ 52.94 Mitem/s │ 52.25 Mitem/s │         │
```

## Version 0.3.0 (2026-04-02)

Initial release, (g)libc based implementation. Ad-hoc tests launched with
`cargo test --release --features=benchmark -- --show-output`.

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
