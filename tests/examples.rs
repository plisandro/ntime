//use ntime::Duration;
use ntime::Timestamp;
use ntime::format::StringFormat;

#[cfg(test)]
mod dummy_examples {
    use super::*;

    #[test]
    fn now() {
        let now = Timestamp::now();
        println!(
            "current time (local): {}",
            now.as_string(&StringFormat::LocalMillisDateTime),
        );
        println!(
            "current time (UTC): {}",
            now.as_string(&StringFormat::UtcMillisDateTime),
        );
    }

    #[test]
    fn duration() {
        let a = Timestamp::from_utc_date(2026, 03, 24, 17, 44, 48, 123, 456);
        let b = Timestamp::from_utc_date(2026, 03, 24, 17, 25, 30, 789, 012);

        print!("from {a} to {b}: ");
        println!("{:?}", (a - b));
    }
}
