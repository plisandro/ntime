use std::thread;
use std::time::Duration;

pub fn sleep(duration: Duration) {
	thread::sleep(duration);
}

pub fn sleep_secs(secs: u64) {
	sleep(Duration::from_secs(secs));
}

pub fn sleep_millis(millis: u64) {
	sleep(Duration::from_millis(millis));
}

pub fn sleep_nanos(nanos: u64) {
	sleep(Duration::from_nanos(nanos));
}
