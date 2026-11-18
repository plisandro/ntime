use core::ffi::CStr;
use core::ffi::{c_char, c_int, c_long};
use std::mem::MaybeUninit;

/* ----------------------- Bindings for C stdlib time functions ----------------------- */

// time_t is always a *signed* type, with a bit width depending on
// both platform and libc implementations :'(
//
//   - All 64-bit systems use 64-bit time_t.
//   - musl +1.2.0 supports 64-bit time_t on 32-bit systems:
//     https://musl.libc.org/time64.html
#[cfg(any(target_pointer_width = "64", target_env = "musl"))]
pub type CTime = i64;
#[cfg(not(any(target_pointer_width = "64", target_env = "musl")))]
pub type CTime = i32;

// Type used for years in c_tm; this determines the maximum timestamp
// which can be converted from and into c_tm structs.
pub type CTmYear = c_int;

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
	unsafe fn localtime_r(ts: *const CTime, tm: *mut c_tm) -> *mut c_tm;
	#[cfg(test)]
	// tzset() is only used to temporarily change the local timezone in tests
	unsafe fn tzset();
}

// Safe C function wrappers
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
	// SAFETY: Parsing a C pointer which is guaranteed to be initialized by (g)libc functions.
	let c_timezone = unsafe { CStr::from_ptr(tm.tm_zone).to_str() };

	match c_timezone {
		Ok(s) => s,
		Err(e) => panic!("failed to resolve TZ string from {tm:?}: {e}"),
	}
}
