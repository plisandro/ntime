pub const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const MONTH_NAMES: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
pub const DAYS_IN_MONTH_COMMON_YEAR: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const DAYS_IN_MONTH_LEAP_YEAR: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub const TIMEZONE_UTC: &str = "UTC";

// dumb conversion constants, but these *really* help redability
pub const U8_DAYS_IN_WEEK: u8 = 7;
pub const U8_MONTHS_IN_YEAR: u8 = 12;
pub const U8_MINUTES_IN_HOUR: u8 = 60;
pub const U8_HOURS_IN_DAY: u8 = 24;
pub const U8_SECONDS_IN_MINUTE: u8 = 60;
pub const U16_MILLIS_IN_SECOND: u16 = 1000;
pub const U16_MINUTES_IN_HOUR: u16 = 60;
pub const U16_SECONDS_IN_HOUR: u16 = U16_MINUTES_IN_HOUR * U16_SECONDS_IN_MINUTE;
pub const U16_SECONDS_IN_MINUTE: u16 = 60;
pub const U16_DAYS_IN_COMMON_YEAR: u16 = 365;
pub const U16_DAYS_IN_LEAP_YEAR: u16 = 366;
pub const U16_UNIX_EPOCH_YEAR: u16 = 1970;
pub const U32_MILLIS_IN_SECOND: u32 = 1000;
pub const U32_NANOS_IN_MILLI: u32 = 1000 * 1000;
pub const U32_NANOS_IN_SECOND: u32 = U32_MILLIS_IN_SECOND * U32_NANOS_IN_MILLI;
pub const U64_MILLIS_IN_SECOND: u64 = U32_MILLIS_IN_SECOND as _;
pub const U64_NANOS_IN_MILLI: u64 = U32_NANOS_IN_MILLI as _;
pub const U128_MILLIS_IN_SECOND: u128 = U32_MILLIS_IN_SECOND as _;
pub const U128_NANOS_IN_MILLI: u128 = U32_NANOS_IN_MILLI as _;
pub const U128_NANOS_IN_SECOND: u128 = U32_NANOS_IN_SECOND as _;
