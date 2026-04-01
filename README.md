# NanoTime

[![](https://img.shields.io/crates/v/ntime.svg)][crates-io]
[![](https://docs.rs/ntime/badge.svg)][api-docs]

NanoTime is a lightweight, high-performance Rust library for nanosecond-precision timestamps.
It offers support for timestamp generation, arithmetics, comparsion and casting to various string
representations, in either local or UTC timezones.

NanoTime has no external Rust dependencies, and runs on all Unix and Windows platforms.

## Usage 

Latest stable release is **v0.3.0**. To use it, add the `ntime` crate to your `Cargo.toml` file:

```toml
[dependencies]
ntime = "0.3.0"
```

## Basic examples

NanoTime can resolve timestamps, and serialize them as multiple formats.

```rust
let now = Timestamp::now();
println!("current time, as nanos since epoch: {}", now.as_nanos());
println!("current time as debug:              {:?}", now);
println!("current time as string:             {}", now.to_string());
println!("current time (local):               {}", now.as_string(&Format::LocalMillisDateTime));
println!("current time (UTC):                 {}", now.as_string(&Format::UtcRFC7231));
```
```
current time, as nanos since epoch:  1774369621732000558
current time as debug:               Timestamp { seconds: 1774369621, nanoseconds: 732000558 },
current time as string:              2026-03-24 17:21:01 +0100
current time (local):                2026-03-24 17:27:01.732 +0100
current time (UTC):                  Tue, 24 Mar 2026 16:27:01 UTC
```

It can also compute durations between timestamps. And it's _blazing_ fast, too - see the 
[benchmarks] page for details - with lock time being dictated mostly by (g)libc calls.

```rust
let start = Timestamp::now();
start.write(&mut io::empty(), &Format::UtcMillisDateTime).expect("oh no the write failed");
println!("wrote a serialized timestamp in {elapsed:?}", elapsed = Timestamp::now() - start);
```
```
wrote a serialized timestamp in 21ns.
```

## Limitations

  * NanoTime is intended mainly to deal with precision timestamps. If you need date/time management
    with full support for timezone and calendar operations, see [Chrono](https://docs.rs/chrono/latest/chrono/) instead.
  * Windows support is partial, and under developement.

## Documentation

  * [API documentation][api-docs]
  * [CHANGELOG]
  * [Real-world benchmarks][benchmarks]

## License

NanoTime is distrubuted under the [MIT license][mit].

<img src="assets/Developed-By-a-Human-Not-By-AI-Badge-white.svg" alt="Courtesy of https://notbyai.fyi/" height="80px"/>

[api-docs]: https://docs.rs/ntime
[crates-io]: https://crates.io/crates/ntime
[CHANGELOG]: CHANGELOG.md
[benchmarks]: assets/benchmarks.md
[mit]: LICENSE
