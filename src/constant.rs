pub const DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const MONTH_NAMES: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

pub const TIMEZONE_UTC: &str = "UTC";

// dumb conversion constants, but these *really* help redability
pub const U8_DAYS_IN_WEEK: u8 = 7;
pub const U8_MONTHS_IN_YEAR: u8 = 12;
pub const U16_SECONDS_IN_MINUTE: u16 = 60;
pub const U16_MINUTES_IN_HOUR: u16 = 60;
pub const U16_SECONDS_IN_HOUR: u16 = U16_MINUTES_IN_HOUR * U16_SECONDS_IN_MINUTE;
pub const U32_MILLIS_IN_SECOND: u32 = 1000;
pub const U32_NANOS_IN_MILLI: u32 = 1000 * 1000;
pub const U32_NANOS_IN_SECOND: u32 = U32_MILLIS_IN_SECOND * U32_NANOS_IN_MILLI;
pub const U64_MILLIS_IN_SECOND: u64 = U32_MILLIS_IN_SECOND as _;
pub const U64_NANOS_IN_MILLI: u64 = U32_NANOS_IN_MILLI as _;
pub const U128_MILLIS_IN_SECOND: u128 = U32_MILLIS_IN_SECOND as _;
pub const U128_NANOS_IN_MILLI: u128 = U32_NANOS_IN_MILLI as _;
pub const U128_NANOS_IN_SECOND: u128 = U32_NANOS_IN_SECOND as _;
