use std::io;
#[cfg(test)]
use std::str;

const STR_SINGLE_DIGIT: &[u8; 10] = b"0123456789";
const STR_DOUBLE_DIGIT: &[&[u8; 2]; 100] = &[
	b"00", b"01", b"02", b"03", b"04", b"05", b"06", b"07", b"08", b"09", b"10", b"11", b"12", b"13", b"14", b"15", b"16", b"17", b"18", b"19", b"20", b"21", b"22", b"23", b"24", b"25", b"26", b"27",
	b"28", b"29", b"30", b"31", b"32", b"33", b"34", b"35", b"36", b"37", b"38", b"39", b"40", b"41", b"42", b"43", b"44", b"45", b"46", b"47", b"48", b"49", b"50", b"51", b"52", b"53", b"54", b"55",
	b"56", b"57", b"58", b"59", b"60", b"61", b"62", b"63", b"64", b"65", b"66", b"67", b"68", b"69", b"70", b"71", b"72", b"73", b"74", b"75", b"76", b"77", b"78", b"79", b"80", b"81", b"82", b"83",
	b"84", b"85", b"86", b"87", b"88", b"89", b"90", b"91", b"92", b"93", b"94", b"95", b"96", b"97", b"98", b"99",
];
const STR_DOUBLE_DIGIT_SPACE_PADDED: &[&[u8; 2]; 10] = &[b" 0", b" 1", b" 2", b" 3", b" 4", b" 5", b" 6", b" 7", b" 8", b" 9"];

const WRITE_BUFFER_MAX_SIZE: usize = 64;

/// A compact write buffer for timestamp output, intended for stack allocation.
///
/// Note that, for performance reasons, no operations for this struct perform
/// index bound nor capacity checks, and will panic if used outside their
/// documented envelope.
pub struct Buffer {
	buf: [u8; WRITE_BUFFER_MAX_SIZE],
	idx: usize,
}

impl Buffer {
	pub fn new() -> Self {
		Self {
			buf: [0; WRITE_BUFFER_MAX_SIZE],
			idx: 0,
		}
	}

	#[cfg(test)]
	/// Returns the [`Buffer`] contents as a [`&str`].
	pub fn as_str(&self) -> &str {
		str::from_utf8(&self.buf[0..self.idx]).expect("invalid string Buffer")
	}

	/// Serializes a single [`u8`] into the [`Buffer`].
	#[inline(always)]
	pub fn u8(&mut self, c: u8) {
		self.buf[self.idx] = c;
		self.idx += 1;
	}

	/// Serializes a space (` `) into the [`Buffer`].
	#[inline(always)]
	pub fn space(&mut self) {
		self.u8(b' ');
	}

	/// Serializes a colon (`:`) into the [`Buffer`].
	#[inline(always)]
	pub fn colon(&mut self) {
		self.u8(b':');
	}

	/// Serializes a hyphen (`-`) into the [`Buffer`].
	#[inline(always)]
	pub fn hyphen(&mut self) {
		self.u8(b'-');
	}

	/// Serializes a dot (`.`) into the [`Buffer`].
	#[inline(always)]
	pub fn dot(&mut self) {
		self.u8(b'.');
	}

	/// Serializes an underscore (`_`) into the [`Buffer`].
	#[inline(always)]
	pub fn underscore(&mut self) {
		self.u8(b'_');
	}

	/// Serializes a [`&str`] into the [`Buffer`].
	#[inline(always)]
	pub fn str(&mut self, s: &str) {
		let len = s.len();
		self.buf[self.idx..self.idx + len].copy_from_slice(s.as_bytes());
		self.idx += len;
	}

	/// Computes the number of digits for a [`u64`].
	#[inline(always)]
	fn u64_digits(n: u64) -> usize {
		n.checked_ilog10().unwrap_or(0) as usize + 1
	}

	/// Serializes a [`u8`] as a decimal string into the [`Buffer`], with configurable padding.
	/// Expects the provided number to fit the (supported) pad size.
	#[inline(always)]
	pub fn u8_dec_padded(&mut self, n: u8, pad: u8, pad_size: usize) {
		if pad_size == 2 {
			if pad == b'0' {
				self.buf[self.idx..self.idx + 2].copy_from_slice(STR_DOUBLE_DIGIT[n as usize]);
				self.idx += 2;
				return;
			}
			if pad == b' ' {
				let digits = if n < 10 { STR_DOUBLE_DIGIT_SPACE_PADDED[n as usize] } else { STR_DOUBLE_DIGIT[n as usize] };
				self.buf[self.idx..self.idx + 2].copy_from_slice(digits);
				self.idx += 2;
				return;
			}
		}

		panic!("unsupported pad {pad}/{pad_size} for u8");
	}

	/// Serializes a [`u64`] as a decimal string into the [`Buffer`].
	#[inline(always)]
	pub fn u64_dec(&mut self, n: u64) {
		match n {
			0..10 => {
				// fast implementations for most commonly used number types; single digit...
				self.buf[self.idx] = STR_SINGLE_DIGIT[n as usize];
				self.idx += 1;
			}
			10..100 => {
				// ...double-digit...
				self.buf[self.idx..self.idx + 2].copy_from_slice(STR_DOUBLE_DIGIT[n as usize]);
				self.idx += 2;
			}
			100..1000 => {
				// ...3 digits...
				let high = (n / 100) as usize;
				let low = (n % 100) as usize;
				self.buf[self.idx] = STR_SINGLE_DIGIT[high];
				self.buf[self.idx + 1..self.idx + 3].copy_from_slice(STR_DOUBLE_DIGIT[low]);
				self.idx += 3;
			}
			1000..10000 => {
				// ...and 4 digits; we do a lot of years.
				let high = (n / 100) as usize;
				let low = (n % 100) as usize;
				self.buf[self.idx..self.idx + 2].copy_from_slice(STR_DOUBLE_DIGIT[high]);
				self.buf[self.idx + 2..self.idx + 4].copy_from_slice(STR_DOUBLE_DIGIT[low]);
				self.idx += 4;
			}
			_ => {
				// iterative output for everything else
				let mut n = n;
				let digits = Self::u64_digits(n);
				let mut rev_idx = self.idx + digits;

				while n >= 100 {
					let pair = (n % 100) as usize;
					n /= 100;
					self.buf[rev_idx - 2..rev_idx].copy_from_slice(STR_DOUBLE_DIGIT[pair]);
					rev_idx -= 2;
				}
				match n {
					0..10 => self.buf[rev_idx - 1] = STR_SINGLE_DIGIT[n as usize],
					_ => self.buf[rev_idx - 2..rev_idx].copy_from_slice(STR_DOUBLE_DIGIT[n as usize]),
				}

				self.idx += digits;
			}
		}
	}

	/// Serializes a [`u64`] as a decimal string into the [`Buffer`], with configurable padding.
	///
	/// Optimized for `pad` = `0`, where exactly pad_size digits will be written. For other pad
	/// types, the output will be `max(<digits in n>, pad_size)` in length.
	#[inline(always)]
	pub fn u64_dec_padded(&mut self, n: u64, pad: u8, pad_size: usize) {
		match pad {
			b'0' => {
				// optimize zero-padded writes by relying on double-digit representations
				let mut rev_idx = self.idx + pad_size;
				let mut n = n;

				while rev_idx >= self.idx + 2 {
					let pair = (n % 100) as usize;
					n /= 100;
					self.buf[rev_idx - 2..rev_idx].copy_from_slice(STR_DOUBLE_DIGIT[pair]);
					rev_idx -= 2;
				}
				if rev_idx > self.idx {
					self.buf[rev_idx - 1] = STR_SINGLE_DIGIT[n as usize];
				}

				self.idx += pad_size;
			}
			_ => {
				// slow fallback version for other pad types
				let digits = Self::u64_digits(n);
				if pad_size > digits {
					let pad_size = pad_size - digits;
					self.buf[self.idx..self.idx + pad_size].fill(pad);
					self.idx += pad_size;
				}
				self.u64_dec(n);
			}
		}
	}

	/// Serializes a [`u128`] as a decimal string into the [`Buffer`].
	#[inline(always)]
	pub fn u128_dec(&mut self, n: u128) {
		match u64::try_from(n) {
			Ok(n) => self.u64_dec(n),
			Err(_) => {
				// yikes, an actual 128-bit uint :'( split the number every 15
				// digits (max 39 for a decimal u128), and serialize in chunks.
				const DIGITS_SPLIT: usize = 15;
				const TEN_15: u128 = 1_000_000_000_000_000;

				let (high, low) = (n / TEN_15, n % TEN_15);
				let (high, mid) = (high / TEN_15, high % TEN_15);

				if high != 0 {
					self.u64_dec(high as u64);
					self.u64_dec_padded(mid as u64, b'0', DIGITS_SPLIT);
					self.u64_dec_padded(low as u64, b'0', DIGITS_SPLIT);
				} else if mid != 0 {
					self.u64_dec(mid as u64);
					self.u64_dec_padded(low as u64, b'0', DIGITS_SPLIT);
				} else {
					self.u64_dec(low as u64);
				}
			}
		}
	}

	/// Serializes a [`u16`] as a decimal string into the [`Buffer`], with configurable padding.
	#[inline(always)]
	pub fn u16_dec_padded(&mut self, n: u16, pad: u8, pad_size: usize) {
		self.u64_dec_padded(n as u64, pad, pad_size);
	}

	/// Serializes a [`u32`] as a decimal string into the [`Buffer`], with configurable padding.
	#[inline(always)]
	pub fn u32_dec_padded(&mut self, n: u32, pad: u8, pad_size: usize) {
		self.u64_dec_padded(n as u64, pad, pad_size);
	}

	/// Writes the contents of the [`Buffer`] into a given [`std::io::Write`].
	#[inline(always)]
	pub fn write<T: io::Write>(&self, out: &mut T) -> io::Result<()> {
		out.write_all(&self.buf[0..self.idx])
	}
}

/* ----------------------- Tests ----------------------- */

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn basic() {
		let mut buf = Buffer::new();

		buf.u8(b'a');
		buf.u8(b'B');
		buf.u8(b'c');
		buf.u8(b'D');
		buf.space();
		buf.str("lalalala");

		assert_eq!(buf.as_str(), "aBcD lalalala");
	}

	#[test]
	fn u8_dec_padded() {
		for (n, pad, pad_size, want) in [(0, b'0', 2, "00"), (0, b' ', 2, " 0"), (5, b'0', 2, "05"), (5, b' ', 2, " 5"), (99, b'0', 2, "99"), (99, b' ', 2, "99")] {
			let mut buf = Buffer::new();

			buf.u8_dec_padded(n, pad, pad_size);
			assert_eq!(buf.as_str(), want)
		}
	}

	#[test]
	#[should_panic]
	fn u8_dec_padded_invalid() {
		let mut buf = Buffer::new();
		buf.u8_dec_padded(167, b'0', 2);
	}

	#[test]
	fn u64_dec() {
		for (n, want) in [
			(0, "0"),
			(9, "9"),
			(10, "10"),
			(999, "999"),
			(1000, "1000"),
			(2026, "2026"),
			(9999, "9999"),
			(10000, "10000"),
			(9383766512304, "9383766512304"),
			(u64::MAX, "18446744073709551615"),
		] {
			let mut buf = Buffer::new();

			buf.u64_dec(n);
			assert_eq!(buf.as_str(), want)
		}
	}

	#[test]
	fn u64_dec_padded() {
		for (n, pad, pad_size, want) in [
			(0, b'0', 1, "0"),
			(0, b'x', 1, "0"),
			(0, b'0', 2, "00"),
			(0, b'y', 2, "y0"),
			(0, b'0', 3, "000"),
			(0, b'z', 3, "zz0"),
			(0, b'0', 4, "0000"),
			(0, b'-', 4, "---0"),
			(0, b'0', 6, "000000"),
			(0, b'/', 6, "/////0"),
			// different behavior based on pad length & type
			(386217, b'0', 6, "386217"),
			(386217, b'0', 4, "6217"),
			(386217, b'0', 10, "0000386217"),
			(386217, b'_', 6, "386217"),
			(386217, b'_', 4, "386217"),
			(386217, b'_', 10, "____386217"),
		] {
			let mut buf = Buffer::new();

			buf.u64_dec_padded(n, pad, pad_size);
			assert_eq!(buf.as_str(), want)
		}
	}

	#[test]
	fn u128_dec() {
		for (n, want) in [
			(0, "0"),
			(9, "9"),
			(10, "10"),
			(2026, "2026"),
			(9383766512304, "9383766512304"),
			(u64::MAX as u128, "18446744073709551615"),
			(u64::MAX as u128 + 9383766512304, "18446753457476063919"),
			(u128::MAX, "340282366920938463463374607431768211455"),
		] {
			let mut buf = Buffer::new();

			buf.u128_dec(n);
			assert_eq!(buf.as_str(), want)
		}
	}
}
