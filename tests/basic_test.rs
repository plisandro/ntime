//use ntime::Duration;
use ntime::Timestamp;
use ntime::format::StringFormat;

#[cfg(test)]
mod basic_test {
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

        println!("from {a} to {b}:");
        println!("{:?}", (a - b));
    }

    /*
    #[test]
    fn casting() {
        let ts = Timestamp::from_utc_date(2026, 03, 24, 17, 25, 30, 123, 456);

        assert_eq!(ts.as_secs(), 1774373130);
        assert_eq!(ts.as_millis(), 1774373130123);
        assert_eq!(ts.as_nanos(), 1774373130123000456);
        assert_eq!(
            ts.as_utc_parts(),
            TimestampParts {
                nanoseconds: 0,
                milliseconds: 1,
                seconds: 2,
                minutes: 3,
                hour: 4,
                month_day: 5,
                month: 6,
                year: 7,
                week_day: 8,
                year_day: 8,
                gmt_offset_secs: 1,
                timezone: "UTC",
            }
        );
    }
    */
}
