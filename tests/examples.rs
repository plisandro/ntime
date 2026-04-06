#[cfg(test)]
mod dummy_examples {
	use ntime::{Format, Timestamp};

	#[test]
	fn now() {
		let now = Timestamp::now();

		println!("current time as nanos:  {}", now.as_nanos());
		println!("current time as debug:  {:?}", now);
		println!("current time as string: {}", now.to_string(),);
		println!("current time (local):   {}", now.as_string(&Format::LocalMillisDateTime),);
		println!("current time (UTC):     {}", now.as_string(&Format::UtcMillisDateTime),);
		println!("week day (local):       {}", now.as_local_parts().week_day);
		println!("year day (UTC):         {}", now.as_utc_parts().year_day);
	}

	#[test]
	fn duration() {
		let a = Timestamp::from_utc_date(2026, 03, 24, 17, 44, 48, 123, 456);
		let b = Timestamp::from_utc_date(2026, 03, 24, 17, 25, 30, 789, 012);

		print!("from {a} to {b}: ");
		println!("{:?}", (a - b));
	}
}
