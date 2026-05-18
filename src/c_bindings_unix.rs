use core::ffi::CStr;
use core::ffi::{c_char, c_int, c_long};
use std::mem::MaybeUninit;
use std::ptr;

pub const NULL_C_CHAR: *mut c_char = ptr::null::<*mut c_char>() as *mut c_char;

/* ----------------------- Bindings for C stdlib time functions ----------------------- */

// time_t is platform-specific, so use the largest single-register type available
pub type CTime = usize;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct c_tm {
	pub tm_sec: c_int,
	pub tm_min: c_int,
	pub tm_hour: c_int,
	pub tm_mday: c_int,
	pub tm_mon: c_int,
	pub tm_year: c_int,
	pub tm_wday: c_int,
	pub tm_yday: c_int,
	pub tm_isdst: c_int,
	pub tm_gmtoff: c_long,
	pub tm_zone: *mut c_char,
}

// *nix C standard time functions
// SAFETY: Wrapper for standard (g)libc time functions. Callers must guarantee
// the correct type and initialization for all passed parameters.
unsafe extern "C" {
	unsafe fn gmtime_r(ts: *const CTime, tm: *mut c_tm) -> *mut c_tm;
	unsafe fn localtime_r(ts: *const CTime, tm: *mut c_tm) -> *mut c_tm;
	#[cfg(test)]
	// tzet() is only used to temporarily change the local timezone in tests
	unsafe fn tzset();
}

// Safe C function wrappers
pub fn c_time_to_utc_tm(ts: CTime) -> Option<c_tm> {
	let ts: *const CTime = &ts;
	let mut tm = MaybeUninit::<c_tm>::uninit();

	// SAFETY: Calling (g)libc functions with properly initialized types.
	unsafe {
		if gmtime_r(ts, tm.as_mut_ptr()).is_null() {
			return None;
		}
	}

	let tm = unsafe { tm.assume_init() };
	Some(tm)
}

pub fn c_time_to_local_tm(ts: CTime) -> Option<c_tm> {
	let ts: *const CTime = &ts;
	let mut tm = MaybeUninit::<c_tm>::uninit();

	// SAFETY: Calling (g)libc functions with properly initialized types.
	unsafe {
		if localtime_r(ts, tm.as_mut_ptr()).is_null() {
			return None;
		}
	}

	let tm = unsafe { tm.assume_init() };
	Some(tm)
}

#[cfg(test)]
/// Forces a libc reload of timezone information. Used only for testing.
pub fn c_reload_tz_info() {
	// SAFETY: Calling a (g)libc functions without arguments nor return values.
	unsafe {
		tzset();
	}
}

/// Extracts a timezone string from a c_tm struct.
pub fn c_timezone_from_tm<'f>(tm: &c_tm) -> &'f str {
	// SAFETY: Parsing a C pointer which is guaranteed to be intialized by (g)libc functions.
	let c_timezone = unsafe { CStr::from_ptr(tm.tm_zone).to_str() };

	match c_timezone {
		Ok(s) => s,
		Err(e) => panic!("failed to resolve TZ string from {tm:?}: {e}"),
	}
}
