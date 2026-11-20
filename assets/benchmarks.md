# Benchmarks 

## Integration test benchmarks

Basic benchmark tests, intended to gauge performance progress across versions, launched with
`cargo bench --profile=release`. Note that, unless specified, methods serializing into
`std::io::Write` are benchmarked using `std::io::Empty`, which will yield better results
than real-world sinks.

All figures below were collected on 16-core AMD Ryzen 9 5950X system with 64GB of DDR4 memory.

## Version 1.0.0 (2026-11-22)

First major release. This version introduces a new highly optimized format serializer,
yielding a throughput boost of 1.31x to 7.03x _(!!!)_ depending on the `Format`, with
mean of ~3.72×.

```
Timer precision: 20 ns
casting               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ as_integer                       │               │               │               │         │
│  ├─ milliseconds    12.19 µs      │ 21.71 µs      │ 12.19 µs      │ 12.29 µs      │ 100     │ 100
│  │                  819.6 Mitem/s │ 460.4 Mitem/s │ 819.6 Mitem/s │ 813.1 Mitem/s │         │
│  ├─ nanoseconds     10.16 µs      │ 23.52 µs      │ 10.21 µs      │ 10.47 µs      │ 100     │ 100
│  │                  983.3 Mitem/s │ 424.9 Mitem/s │ 978.4 Mitem/s │ 954.2 Mitem/s │         │
│  ╰─ seconds         8.949 µs      │ 14.78 µs      │ 9.054 µs      │ 10.01 µs      │ 100     │ 100
│                     1.117 Gitem/s │ 676.1 Mitem/s │ 1.104 Gitem/s │ 998.3 Mitem/s │         │
├─ to_string                        │               │               │               │         │
│  ├─ local           873.5 µs      │ 1.516 ms      │ 928.5 µs      │ 959.2 µs      │ 100     │ 100
│  │                  11.44 Mitem/s │ 6.593 Mitem/s │ 10.76 Mitem/s │ 10.42 Mitem/s │         │
│  ├─ nanoseconds     298.1 µs      │ 546.7 µs      │ 310.8 µs      │ 321.3 µs      │ 100     │ 100
│  │                  33.54 Mitem/s │ 18.28 Mitem/s │ 32.16 Mitem/s │ 31.12 Mitem/s │         │
│  ╰─ utc             376.4 µs      │ 559.9 µs      │ 402.3 µs      │ 420.1 µs      │ 100     │ 100
│                     26.56 Mitem/s │ 17.85 Mitem/s │ 24.85 Mitem/s │ 23.79 Mitem/s │         │
╰─ write                            │               │               │               │         │
   ├─ io_empty                      │               │               │               │         │
   │  ├─ local        588.4 µs      │ 1.131 ms      │ 605.9 µs      │ 633.3 µs      │ 100     │ 100
   │  │               16.99 Mitem/s │ 8.837 Mitem/s │ 16.5 Mitem/s  │ 15.78 Mitem/s │         │
   │  ├─ nanoseconds  126.7 µs      │ 195.9 µs      │ 131 µs        │ 140.9 µs      │ 100     │ 100
   │  │               78.9 Mitem/s  │ 51.04 Mitem/s │ 76.28 Mitem/s │ 70.94 Mitem/s │         │
   │  ╰─ utc          125.1 µs      │ 147 µs        │ 129.5 µs      │ 130.4 µs      │ 100     │ 100
   │                  79.87 Mitem/s │ 68.02 Mitem/s │ 77.19 Mitem/s │ 76.63 Mitem/s │         │
   ╰─ mem_buffer                    │               │               │               │         │
      ├─ local        630.7 µs      │ 1.128 ms      │ 643.5 µs      │ 671.3 µs      │ 100     │ 100
      │               15.85 Mitem/s │ 8.863 Mitem/s │ 15.53 Mitem/s │ 14.89 Mitem/s │         │
      ├─ nanoseconds  147.9 µs      │ 240.8 µs      │ 154.5 µs      │ 165.3 µs      │ 100     │ 100
      │               67.6 Mitem/s  │ 41.51 Mitem/s │ 64.68 Mitem/s │ 60.46 Mitem/s │         │
      ╰─ utc          178.1 µs      │ 272.2 µs      │ 181 µs        │ 189.9 µs      │ 100     │ 100
                      56.11 Mitem/s │ 36.73 Mitem/s │ 55.22 Mitem/s │ 52.65 Mitem/s │         │
```

## Version 0.8.0 (2026-11-01)

```
Timer precision: 20 ns
casting               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ as_integer                       │               │               │               │         │
│  ├─ milliseconds    47.55 µs      │ 54.44 µs      │ 47.79 µs      │ 48.14 µs      │ 100     │ 100
│  │                  210.2 Mitem/s │ 183.6 Mitem/s │ 209.2 Mitem/s │ 207.7 Mitem/s │         │
│  ├─ nanoseconds     45.79 µs      │ 54.92 µs      │ 45.8 µs       │ 47.44 µs      │ 100     │ 100
│  │                  218.3 Mitem/s │ 182 Mitem/s   │ 218.2 Mitem/s │ 210.7 Mitem/s │         │
│  ╰─ seconds         45.79 µs      │ 66 µs         │ 45.8 µs       │ 46.2 µs       │ 100     │ 100
│                     218.3 Mitem/s │ 151.4 Mitem/s │ 218.2 Mitem/s │ 216.4 Mitem/s │         │
├─ to_string                        │               │               │               │         │
│  ├─ local           2.428 ms      │ 3.166 ms      │ 2.466 ms      │ 2.49 ms       │ 100     │ 100
│  │                  4.118 Mitem/s │ 3.157 Mitem/s │ 4.055 Mitem/s │ 4.014 Mitem/s │         │
│  ├─ nanoseconds     318 µs        │ 385.7 µs      │ 327.5 µs      │ 327.6 µs      │ 100     │ 100
│  │                  31.44 Mitem/s │ 25.92 Mitem/s │ 30.53 Mitem/s │ 30.51 Mitem/s │         │
│  ╰─ utc             1.814 ms      │ 2.01 ms       │ 1.827 ms      │ 1.834 ms      │ 100     │ 100
│                     5.512 Mitem/s │ 4.974 Mitem/s │ 5.47 Mitem/s  │ 5.451 Mitem/s │         │
╰─ write                            │               │               │               │         │
   ├─ io_empty                      │               │               │               │         │
   │  ├─ local        436.3 µs      │ 455.1 µs      │ 442.1 µs      │ 442.7 µs      │ 100     │ 100
   │  │               22.91 Mitem/s │ 21.96 Mitem/s │ 22.61 Mitem/s │ 22.58 Mitem/s │         │
   │  ├─ nanoseconds  14.28 µs      │ 18.16 µs      │ 14.29 µs      │ 14.35 µs      │ 100     │ 100
   │  │               699.8 Mitem/s │ 550.3 Mitem/s │ 699.3 Mitem/s │ 696.4 Mitem/s │         │
   │  ╰─ utc          173.5 µs      │ 181.3 µs      │ 175.7 µs      │ 175.9 µs      │ 100     │ 100
   │                  57.61 Mitem/s │ 55.15 Mitem/s │ 56.9 Mitem/s  │ 56.82 Mitem/s │         │
   ╰─ mem_buffer                    │               │               │               │         │
      ├─ local        1.896 ms      │ 2.003 ms      │ 1.94 ms       │ 1.938 ms      │ 100     │ 100
      │               5.272 Mitem/s │ 4.991 Mitem/s │ 5.153 Mitem/s │ 5.159 Mitem/s │         │
      ├─ nanoseconds  202.2 µs      │ 222.8 µs      │ 206.6 µs      │ 207 µs        │ 100     │ 100
      │               49.44 Mitem/s │ 44.87 Mitem/s │ 48.38 Mitem/s │ 48.28 Mitem/s │         │
      ╰─ utc          1.208 ms      │ 1.454 ms      │ 1.226 ms      │ 1.235 ms      │ 100     │ 100
                      8.275 Mitem/s │ 6.877 Mitem/s │ 8.153 Mitem/s │ 8.093 Mitem/s │         │
```

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
