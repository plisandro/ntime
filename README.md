# NanoTime

<p>
    <picture>
      <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/plisandro/ntime/master/assets/nanotime_title_light_horizontal.png" width="350px">
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/plisandro/ntime/master/assets/nanotime_title_dark_horizontal.png" width="350px">
      <img src="https://raw.githubusercontent.com/plisandro/ntime/master/assets/nanotime_title_light_horizontal.png" width="350px" />
    </picture>
    <br>
</p>

[![](https://img.shields.io/crates/v/ntime.svg)][crates-io]
[![](https://docs.rs/ntime/badge.svg)][api-docs]

NanoTime is a lightweight, high performance Rust library for nanosecond-precision timestamps.
It offers support for timestamp generation, arithmetics, comparsion and casting to various string
representations, in either local or UTC timezones.

NanoTime has no external Rust dependencies, and runs on all Unix and Windows platforms.

## Usage 

Latest stable release is **v0.4.0**. To use it, add the `ntime` crate to your `Cargo.toml` file:

```toml
[dependencies]
ntime = "0.4.0"
```

## Basic examples

NanoTime can resolve timestamps, break them down in date+time parts, and serialize in multiple formats.

```rust
let now = Timestamp::now();
println!("current time, as nanos since epoch: {}", now.as_nanos());
println!("current time as debug:              {:?}", now);
println!("current time as string:             {}", now.to_string());
println!("current time (local):               {}", now.as_string(&Format::LocalMillisDateTime));
println!("current time (UTC):                 {}", now.as_string(&Format::UtcRFC7231));
println!("week day (local):                   {}", now.as_local_parts().week_day);
println!("year day (UTC):                     {}", now.as_utc_parts().year_day);
```
```
current time, as nanos since epoch: 1774369621732000558
current time as debug:              Timestamp { seconds: 1774369621, nanoseconds: 732000558 }
current time as string:             2026-03-24 16:27:01 +0000
current time (local):               2026-03-24 16:27:01.732 +0000
current time (UTC):                 Tue, 24 Mar 2026 16:27:01 UTC
week day (local):                   3
year day (UTC):                     83
```

It can also compute durations between timestamps. And it's _blazing_ fast - see the 
[benchmarks] page for details - with lock times being dictated mostly by (g)libc calls.

```rust
let start = Timestamp::now();
start.write(&mut io::empty(), &Format::UtcMillisDateTime).expect("oh no the write failed");
println!("wrote a serialized timestamp in {elapsed:?}", elapsed = Timestamp::now() - start);
```
```
wrote a serialized timestamp in 21ns.
```

## Limitations

  * NanoTime is intended to efficiently deal with precision timestamps. If you need date/time management with full
    support for timezone and calendar operations, consider using [Chrono](https://docs.rs/chrono/latest/chrono/) instead.
  * Windows support is partial, and under developement.

## Documentation

  * [API documentation][api-docs]
  * [CHANGELOG]
  * [Real-world benchmarks][benchmarks]

## License

NanoTime is distrubuted under the [MIT license][mit].

<img src="assets/Developed-By-a-Human-Not-By-AI-Badge-white.svg" title="Courtesy of https://notbyai.fyi/" height="80px"/>

[api-docs]: https://docs.rs/ntime
[crates-io]: https://crates.io/crates/ntime
[CHANGELOG]: CHANGELOG.md
[benchmarks]: assets/benchmarks.md
[mit]: LICENSE
