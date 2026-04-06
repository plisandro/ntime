pub const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const MONTH_NAMES: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
pub const DAYS_IN_MONTH_COMMON_YEAR: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_IN_MONTH_LEAP_YEAR: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const MONTH_TO_DAYS_COMMON_YEAR: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
pub const MONTH_TO_DAYS_LEAP_YEAR: [u16; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

pub const TIMEZONE_UTC: &str = "UTC";

/* ----------------------- Dumb conversion constants ----------------------- */
// these *really* help redability

pub const U8_DAYS_IN_WEEK: u8 = 7;
pub const U8_MONTHS_IN_YEAR: u8 = 12;
pub const U8_MINUTES_IN_HOUR: u8 = 60;
pub const U8_HOURS_IN_DAY: u8 = 24;
pub const U8_SECONDS_IN_MINUTE: u8 = 60;

pub const U16_MILLIS_IN_SECOND: u16 = 1000;
pub const U16_MINUTES_IN_HOUR: u16 = U8_MINUTES_IN_HOUR as _;
pub const U16_SECONDS_IN_HOUR: u16 = U16_MINUTES_IN_HOUR * U16_SECONDS_IN_MINUTE;
pub const U16_SECONDS_IN_MINUTE: u16 = U8_SECONDS_IN_MINUTE as _;
pub const U16_DAYS_IN_COMMON_YEAR: u16 = 365;
pub const U16_DAYS_IN_LEAP_YEAR: u16 = 366;
pub const U16_UNIX_EPOCH_YEAR: u16 = 1970;
pub const U16_LEAP_YEARS_BEFORE_EPOCH: u16 = 477;

pub const U32_MILLIS_IN_SECOND: u32 = 1000;
pub const U32_NANOS_IN_MILLI: u32 = 1000 * 1000;
pub const U32_NANOS_IN_SECOND: u32 = U32_MILLIS_IN_SECOND * U32_NANOS_IN_MILLI;
pub const U32_SECONDS_IN_DAY: u32 = U16_SECONDS_IN_HOUR as u32 * U8_HOURS_IN_DAY as u32;

pub const U64_MILLIS_IN_SECOND: u64 = U16_MILLIS_IN_SECOND as _;
pub const U64_NANOS_IN_MILLI: u64 = U32_NANOS_IN_MILLI as _;
pub const U64_SECONDS_IN_MINUTE: u64 = U8_SECONDS_IN_MINUTE as _;
pub const U64_SECONDS_IN_HOUR: u64 = U16_SECONDS_IN_HOUR as _;
pub const U64_SECONDS_IN_DAY: u64 = U32_SECONDS_IN_DAY as _;
pub const U64_SECONDS_IN_COMMON_YEAR: u64 = U16_DAYS_IN_COMMON_YEAR as u64 * U64_SECONDS_IN_DAY;
pub const U64_LEAP_YEARS_BEFORE_EPOCH: u64 = U16_LEAP_YEARS_BEFORE_EPOCH as _;

pub const U128_MILLIS_IN_SECOND: u128 = U16_MILLIS_IN_SECOND as _;
pub const U128_NANOS_IN_MILLI: u128 = U32_NANOS_IN_MILLI as _;
pub const U128_NANOS_IN_SECOND: u128 = U32_NANOS_IN_SECOND as _;
