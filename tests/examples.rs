#[cfg(test)]
mod dummy_examples {
	use ntime::{Duration, Format, Precision, Timestamp, Timezone, UTCDateTime};

	#[test]
	fn now() {
		let now = Timestamp::now();

		println!("current time as nanos:  {}", now.as_nanos());
		println!("current time as debug:  {:?}", now);
		println!("current time as string: {}", now);
		println!("current time (local):   {}", now.as_string(&Format::DateTime(Timezone::Local, Precision::Milliseconds)));
		println!("current time (UTC):     {}", now.as_string(&Format::DateTime(Timezone::UTC, Precision::Milliseconds)));
		println!("week day (local):       {}", now.as_local_parts().week_day);
		println!("year day (UTC):         {}", now.as_utc_parts().year_day);
	}

	#[test]
	fn from_utc_datetime_tuple() {
		let ts = Timestamp::try_from((2026, 11, 18, 22, 17, 19)).expect("invalid date/time values");
		println!("initialized from date/time values to {ts}");
	}

	#[test]
	fn from_utcdatetime() {
		let udt = UTCDateTime {
			year: 2026,
			month: 3,
			day: 24,
			hour: 17,
			minutes: 44,
			seconds: 48,
			milliseconds: 123,
			nanoseconds: 456,
		};
		let ts = Timestamp::try_from(&udt).expect("invalid UTCDateTime");

		println!("{udt:?} converts to {ts}");
	}

	#[test]
	fn add_duration() {
		let a = Timestamp::try_from((2026, 3, 24, 17, 44, 48, 123, 456)).expect("invalid parameters for timestamp");
		let delta = Duration::from_hours(10);

		println!("[{a}] + {delta:?} = {}", a.add_duration(&delta).unwrap());
	}

	#[test]
	fn difference() {
		let a = Timestamp::try_from((2026, 3, 24, 17, 44, 48, 123, 456)).expect("invalid parameters for timestamp A");
		let b = Timestamp::try_from((2026, 3, 24, 17, 25, 30, 789, 12)).expect("invalid parameters for timestamp B");

		println!("from {a} to {b}: {:?}", (a - b).unwrap());
	}
}
