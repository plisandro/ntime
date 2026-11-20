<div align="center">
  <p>
      <picture>
        <img src="https://raw.githubusercontent.com/plisandro/ntime/master/assets/nanotime_title_horizontal.svg" width="400px" />
      </picture>
      <br>
  </p>
  </br>
  <p>
      <a href="https://crates.io/crates/ntime"><img src="https://img.shields.io/crates/v/ntime.svg" alt="ntime crate" /></a>
       <a href="https://github.com/plisandro/ntime/actions"><img src="https://github.com/plisandro/ntime/workflows/build/badge.svg" alt="Build status" /></a>
      <a href="https://docs.rs/ntime/latest/ntime/"><img src="https://docs.rs/ntime/badge.svg" alt="API docs" /></a>
  </p>
  </br>
</div>

NanoTime is a lightweight, [high performance][benchmarks] Rust library for nanosecond-precision timestamps.
It offers support for timestamp generation, arithmetic, comparison and casting to various string
representations, in either local or UTC timezones.

NanoTime has no external Rust dependencies, and runs on all Unix and Windows platforms.

## Usage 

Latest stable release is **v1.0.0**. To use it, add the `ntime` crate to your `Cargo.toml` file:

```toml
[dependencies]
ntime = "1.0.0"
```

## Basic examples

NanoTime can resolve timestamps, break them down in date+time parts, and serialize into multiple formats.

```rust
use ntime::{Format, Precision, Timezone, Timestamp};

let now = Timestamp::now();
println!("nanos since epoch: {}", now.as_nanos());
println!("debug:             {:?}", now);
println!("to_string():       {}", now.to_string());
println!("local time:        {}", now.to_string_as(&Format::DateTime(Timezone::Local, Precision::Milliseconds)));
println!("UTC time:          {}", now.to_string_as(&Format::DateTime(Timezone::Utc, Precision::Nanoseconds)));
println!("HTTP/1.1:          {}", now.to_string_as(&Format::Rfc7231));
println!("local week day:    {}", now.to_local_parts().week_day);
println!("UTC year day:      {}", now.to_utc_parts().year_day);
```
```text
nanos since epoch: 1774369621732000558
debug:             Timestamp { seconds: 1774369621, nanoseconds: 732000558 }
to_string():       2026-03-24 16:27:01.732
local time:        2026-03-24 17:27:01.732 +0100
UTC time:          2026-03-24 16:27:01.732000558
HTTP/1.1:          Tue, 24 Mar 2026 16:27:01 GMT
local week day:    3
UTC year day:      83
```

It can also compute durations between timestamps. And it's _blazing_ fast - see the 
[benchmarks] page for details - with lock times being dictated mostly by (g)libc calls.

```rust
let start = Timestamp::now();
start.write(&mut io::empty(), &Format::DateTime(Timezone::Utc, Precision::Milliseconds)).expect("UTC write failed");
println!("wrote a UTC timestamp to Empty in {elapsed:?}", elapsed = Timestamp::now() - start);
```
```
wrote a UTC timestamp to Empty in 4.17µs
```

## Limitations

  * NanoTime is intended to efficiently deal with precision timestamps. If you need date/time management with full
    support for timezone and calendar operations, consider using [Chrono](https://docs.rs/chrono/latest/chrono/) instead.
  * Local timezone conversions are resolved via (g)libc calls, with type limitations imposed by the C FFI.
    As a result, the maximum timestamp which can be formatted in local TZ is smaller than the largest timestamp
    NanoTime can handle; on 64-bit systems, that limit is still large enough for all practical purposes (well
    into the year `2,147,485,547`) but, on 32-bit systems, that limit is
    [the year 2038](https://en.wikipedia.org/wiki/Year_2038_problem).

## Documentation

  * [API documentation][api-docs]
  * [CHANGELOG]
  * [Real-world benchmarks][benchmarks]

## Support

Comments, feedback and bug reports are always welcome!

Contributions will be accepted under the [project's license](#license).

## License

NanoTime is distributed under the terms of the [MIT][mit] or [Apache 2.0][apache] license, at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in NanoTime
by you, as defined in the [Apache 2.0][apache] license, shall be dual licensed as above, without any
additional terms or conditions.

<img src="assets/Developed-By-a-Human-Not-By-AI-Badge-white.svg" title="Courtesy of https://notbyai.fyi/" height="80px"/>

[api-docs]: https://docs.rs/ntime
[build-status]: https://github.com/plisandro/ntime/actions?query=workflow%3Abuild+branch%3Amaster
[crates-io]: https://crates.io/crates/ntime
[CHANGELOG]: CHANGELOG.md
[benchmarks]: assets/benchmarks.md
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
