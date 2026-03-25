# NanoTime

NanoTime is a lightweight, high-performance Rust library for nanosecond-precision timestamps.
It offers support for timestamp generation, comparsion and casting to various string
representations, in either local or UTC timezones.

## Usage 

To use the ntime crate, add this to your `Cargo.toml` file:

```toml
[dependencies]
ntime = "0.0.1"
```

## Basic examples

NanoTime can resolve timestamps, and serialize them into local or UTC date strings.

```rust
let now = Timestamp::now();
println!("current time (local): {}", now.as_string(StringFormat::LocalMillisDateTime));
println!("current time (UTC):   {}", now.as_string(StringFormat::UtcRFC7231));
```
```
Current time (local): 2026-03-24 17:27:01.732 +0100
Current time (UTC):   Tue, 24 Mar 2026 16:27:01 UTC
```

It can also compute durations between timestamps. And it's _blazing_ fast!

```rust
let start = Timestamp::now();
start.write(&mut io::empty(), StringFormat::UtcMillisDateTime).expect("oh no the write failed");
println!("wrote a serialized timestam in {elapsed:?}", elapsed = Timestamp::now() - start);
```
```
wrote a serialized timestamp in 133ns.
```

## Limitations

  * NanoTime is intended mainly to deal with precision timestamps. If you need date/time management
    with full support for timezone and calendar operations, see [Chrono](https://docs.rs/chrono/latest/chrono/) instead.
  * Supports both Unix and Windows platforms. Windows support is partial and under developement thou.

## Documentation

**[API documentation][api-docs]**

## License

NanoTime is distrubuted under the [MIT license][mit].

[crates-io]: https://crates.io/crates/ntime
[CHANGELOG file]: CHANGELOG.md
[mit]: LICENSE
