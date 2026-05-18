use std::sync::LazyLock;
//use core::ffi::CStr;
use core::ffi::{c_char, c_int, c_long};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_long;
use std::ptr;

pub const NULL_C_CHAR: *mut c_char = ptr::null::<*mut c_char>() as *mut c_char;

/* ----------------------- Bindings for C stdlib time functions ----------------------- */

// time_t is platform-specific, so use the largest single-register type available
pub type CTime = usize;
// size_t is platform-specific, so use the largest single-register type available
// TODO: replace by core::ffi::c_size_t once stable.
pub type CSize = usize;

// TZ details are surprisingly expensive to resolve in Windows :( GMT offset is
// retrieved for every timestamp generation, as it can change over the duration of
// a process, but the TZ name is kept cached.
pub static MSVC_TZ_NAME: LazyLock<String> = LazyLock::new(|| c_tz_name());

// Windows MSVC timezone fields
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
}

// Windows MSVC standard time functions
// SAFETY: Wrapper for standard MSCV time functions. Callers must guarantee
// the correct type and initialization for all passed parameters.
unsafe extern "C" {
	unsafe fn _gmtime64_s(tm: *mut c_tm, ts: *const CTime) -> c_int;
	unsafe fn _localtime64_s(tm: *mut c_tm, ts: *const CTime) -> c_int;
	// Windows is stupid and doesn't return TZ information in tm structs, so...
	unsafe fn _get_timezone(seconds: *mut c_long) -> c_int;
	unsafe fn _get_tzname(pReturnValue: *mut CSize, timeZoneName: *mut c_char, sizeInBytes: CSize, index: c_int) -> c_int;
	unsafe fn _tzset();
}

// Safe C function wrappers
pub fn c_time_to_utc_tm(ts: CTime) -> Option<c_tm> {
	let ok: bool;
	let ts: *const CTime = &ts;
	let mut tm = MaybeUninit::<c_tm>::uninit();

	// SAFETY: Calling (g)libc functions with properly initialized types.
	unsafe {
		ok = _gmtime64_s(tm.as_mut_ptr(), ts) == 0;
	}
	if !ok {
		return None;
	}

	let tm = unsafe { tm.assume_init() };
	Some(tm)
}

pub fn c_time_to_local_tm(ts: CTime) -> Option<c_tm> {
	let ok: bool;
	let ts: *const CTime = &ts;
	let mut tm = MaybeUninit::<c_tm>::uninit();

	// SAFETY: Calling MSCV functions with properly initialized types.
	unsafe {
		ok = _localtime64_s(tm.as_mut_ptr(), ts) == 0;
	}
	if !ok {
		return None;
	}

	let tm = unsafe { tm.assume_init() };
	Some(tm)
}

#[cfg(test)]
/// Forces a libc reload of timezone information. Used only for testing.
pub fn c_reload_tz_info() {
	// SAFETY: Calling a MSVC functions without arguments nor return values.
	unsafe {
		_tzset();
	}
}

/// Resolves the current timezone GMT offset.
pub fn c_tz_gmt_offset() -> i16 {
	let mut gmt_offset: c_long = 5;

	// SAFETY: Calling MSVC functions with properly initialized types.
	unsafe {
		_tzset();
		println!(">>>>>>>>>>> gmt offset {}", gmt_offset);
		let errno = _get_timezone(&mut gmt_offset);
		if errno != 0 {
			panic!("failed to resolve TZ GMT offset: {errno}");
		}
		println!(">>>>>>>>>>> gmt offset {}", gmt_offset);
	}

	gmt_offset as i16
}

/// Resolves the current timezone GMT offset.
pub fn c_tz_name() -> String {
	let tz_name: String;
	// return abridged TZ name
	const INDEX: c_int = 0;

	// SAFETY: Calling MSVC functions with properly initialized types.
	unsafe {
		// this convoluted implementation comes directly from Microsoft:
		// https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/get-tzname
		_tzset();

		// get the size of buffer required to hold the TZ name
		let mut buf_size: CSize = 0;
		let errno = _get_tzname(&mut buf_size, NULL_C_CHAR, 0, INDEX);
		if errno != 0 {
			panic!("failed to resolve TZ name buffer size: {errno}");
		}
		println!(">>>>>>>>>>> buf {}", buf_size);

		// allocate a buffer for the tz name
		let c_tz_name = CString::from_vec_unchecked(Vec::<u8>::with_capacity(buf_size as usize));
		let c_tz_name_ptr = c_tz_name.into_raw();

		// load tz name in the buffer
		let errno = _get_tzname(&mut buf_size, c_tz_name_ptr as *mut c_char, buf_size, INDEX);
		if errno != 0 {
			panic!("failed to resolve TZ name: {errno}");
		}

		tz_name = match CString::from_raw(c_tz_name_ptr).into_string() {
			Ok(s) => s,
			Err(e) => panic!("failed to serialize TZ string from C buffer: {e}"),
		};

		println!(">>>>>>>>>>> tz name {} ({} -> {})", tz_name, buf_size, tz_name.len());
	}

	tz_name
}
