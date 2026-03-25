use std::thread;
use std::time;

/// Sleeps the current thread a period of time expressed in [`time::Duration`].
pub fn sleep(duration: time::Duration) {
	thread::sleep(duration);
}

/// Sleeps the current thread for a number of seconds.
pub fn sleep_secs(secs: u64) {
	sleep(time::Duration::from_secs(secs));
}

/// Sleeps the current thread for a number of milliseconds.
pub fn sleep_millis(millis: u64) {
	sleep(time::Duration::from_millis(millis));
}

/// Sleeps the current thread for a number of nanoseconds.
pub fn sleep_nanos(nanos: u64) {
	sleep(time::Duration::from_nanos(nanos));
}
