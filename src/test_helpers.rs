#[cfg(test)]
pub mod mocks {
	use std::sync::Mutex;

	const MOCK_TEST_LOCK: Mutex<()> = Mutex::new(());
	const TIMEZONE_ENV_VAR: &str = "TZ";

	pub fn with_timezone(mock_tz: &str, test_fn: fn()) {
		let bind = MOCK_TEST_LOCK;
		let _lock = bind.lock().unwrap();

		#[cfg(target_env = "msvc")]
		{
			// TODO: figure out a proper way to deal with mock timezones in Windows.
			println!("!!! mock timezones are not supported on Windows, skipping test {test_fn:?} with {TIMEZONE_ENV_VAR}=\"{mock_tz}\"...");
		}
		#[cfg(not(target_env = "msvc"))]
		{
			use crate::c_bindings;
			use std::env;
			use std::panic;

			let orig_tz = match env::var(TIMEZONE_ENV_VAR) {
				Ok(v) => v,
				Err(_) => "".into(),
			};

			// SAFETY: setting timezone environemt variable on a testing context, with
			// a mutex lock to ensure only one thread does at a time.
			unsafe {
				env::set_var(TIMEZONE_ENV_VAR, mock_tz);
			}
			c_bindings::c_reload_tz_info();

			let res = panic::catch_unwind(test_fn);

			// SAFETY: setting timezone environemt variable on a testing context, with
			// a mutex lock to ensure only one thread does at a time.
			unsafe {
				env::set_var(TIMEZONE_ENV_VAR, orig_tz);
			}
			c_bindings::c_reload_tz_info();

			match res {
				Ok(_) => (),
				Err(panic) => panic::resume_unwind(panic),
			}
		}
	}
}
