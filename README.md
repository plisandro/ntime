# NanoTime

NanoTime is a lightweight, high-performance Rust library for nanosecond-precision timestamps.
It offers support for timestamp generation, arithmetics, comparsion and casting to various string
representations, in either local or UTC timezones.

NanoTime has no external dependencies, and runs on all Unix and Windows platforms.

## Usage 

To use the ntime crate, add this to your `Cargo.toml` file:

```toml
[dependencies]
ntime = "0.0.1"
```

## Basic examples

NanoTime can resolve timestamps, and serialize them as multiple formats.

```rust
let now = Timestamp::now();
println!("current time, as nanos since epoch: {}", now.as_nanos());
println!("current time as debug:              {:?}", now);
println!("current time as string:             {}", now.to_string());
println!("current time (local):               {}", now.as_string(&StringFormat::LocalMillisDateTime));
println!("current time (UTC):                 {}", now.as_string(&StringFormat::UtcRFC7231));
```
```
current time, as nanos since epoch:  1774369621732000558
current time as debug:               Timestamp { seconds: 1774369621, nanoseconds: 732000558 },
current time as string:              2026-03-24 17:21:01 +0100
current time (local):                2026-03-24 17:27:01.732 +0100
current time (UTC):                  Tue, 24 Mar 2026 16:27:01 UTC
```

It can also compute durations between timestamps. And it's _fast_ - on modern x64 systems,
most of NanoTime's overhead is introduced solely by (g)libc primitives.

```rust
let start = Timestamp::now();
start.write(&mut io::empty(), &StringFormat::UtcMillisDateTime).expect("oh no the write failed");
println!("wrote a serialized timestamp in {elapsed:?}", elapsed = Timestamp::now() - start);
```
```
wrote a serialized timestamp in 121ns.
```

## Limitations

  * NanoTime is intended mainly to deal with precision timestamps. If you need date/time management
    with full support for timezone and calendar operations, see [Chrono](https://docs.rs/chrono/latest/chrono/) instead.
  * Windows support is partial, and under developement.

## Documentation

**[API documentation][api-docs]**

## License

NanoTime is distrubuted under the [MIT license][mit].

[crates-io]: https://crates.io/crates/ntime
[CHANGELOG file]: CHANGELOG.md
[mit]: LICENSE
