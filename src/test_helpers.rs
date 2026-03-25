#[cfg(test)]
pub mod mocks {
	use std::env;
	use std::panic;
	use std::sync::Mutex;

	use crate::c_bindings;

	const MOCK_TEST_LOCK: Mutex<()> = Mutex::new(());
	const TIMEZONE_ENV_VAR: &str = "TZ";

	pub fn with_timezone(mock_tz: &str, test_fn: fn()) {
		let bind = MOCK_TEST_LOCK;
		let _lock = bind.lock().unwrap();

		let orig_tz = match env::var(TIMEZONE_ENV_VAR) {
			Ok(v) => v,
			Err(_) => "".into(),
		};
		unsafe {
			env::set_var(TIMEZONE_ENV_VAR, mock_tz);
		}
		c_bindings::c_reload_tz_info();

		let res = panic::catch_unwind(test_fn);

		unsafe {
			env::set_var(TIMEZONE_ENV_VAR, orig_tz);
		}
		c_bindings::c_reload_tz_info();

		match res {
			Ok(_) => {}
			Err(panic) => std::panic::resume_unwind(panic),
		}
	}
}
