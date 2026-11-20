#![no_std]
//! Core conversion library for the **everdude** time system.
//!
//! - **Epoch**: `2001-01-01 00:00:00 UTC` (Monday), corresponding to Unix timestamp
//!   `978_307_200` seconds (`978_307_200_000` milliseconds).
//! - **Standard unit**: Half-week (`h`), where `1` half-week = `3.5` days = `302_400` seconds
//!   (`302_400_000` milliseconds).
//!   - Even integers of `h` correspond to Monday `00:00 UTC`.
//!   - Odd integers of `h` correspond to Thursday `12:00 UTC`.
//! - **Underlying time**: Unix UTC without leap seconds or DST, represented as `f64`.

/// everdude epoch in Unix seconds (`2001-01-01T00:00:00Z`, Monday).
pub const EPOCH_UNIX_SECS: i64 = 978_307_200;

/// everdude epoch in Unix milliseconds (`2001-01-01T00:00:00.000Z`, Monday).
pub const EPOCH_UNIX_MS: i64 = 978_307_200_000;

/// everdude epoch in Unix microseconds.
pub const EPOCH_UNIX_US: i128 = 978_307_200_000_000;

/// everdude epoch in Unix nanoseconds.
pub const EPOCH_UNIX_NS: i128 = 978_307_200_000_000_000;

/// Duration of one half-week in seconds (`3.5 * 86_400`).
pub const HALF_WEEK_SECS: f64 = 302_400.0;

/// Duration of one half-week in milliseconds (`3.5 * 86_400_000`).
pub const HALF_WEEK_MS: f64 = 302_400_000.0;

/// Duration of one half-week in microseconds.
pub const HALF_WEEK_US: f64 = 302_400_000_000.0;

/// Duration of one half-week in nanoseconds.
pub const HALF_WEEK_NS: f64 = 302_400_000_000_000.0;

/// Normalizes `-0.0` to `+0.0`.
#[inline(always)]
pub const fn normalize_zero(v: f64) -> f64 {
    if v == 0.0 { 0.0 } else { v }
}

/// Converts Unix timestamp in seconds (`f64`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_secs(secs: f64) -> f64 {
    normalize_zero((secs - EPOCH_UNIX_SECS as f64) / HALF_WEEK_SECS)
}

/// Converts integer Unix timestamp in seconds (`i64`) to everdude half-weeks (`h`).
///
/// Subtracts the epoch in integer arithmetic first to preserve full `f64` precision.
#[inline(always)]
pub fn from_unix_secs_i64(secs: i64) -> f64 {
    from_unix_secs_i128(secs as i128)
}

/// Converts integer Unix timestamp in seconds (`i128`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_secs_i128(secs: i128) -> f64 {
    normalize_zero(((secs - EPOCH_UNIX_SECS as i128) as f64) / HALF_WEEK_SECS)
}

/// Converts integer Unix timestamp in milliseconds (`i64`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_ms(ms: i64) -> f64 {
    from_unix_ms_i128(ms as i128)
}

/// Converts integer Unix timestamp in milliseconds (`i128`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_ms_i128(ms: i128) -> f64 {
    normalize_zero(((ms - EPOCH_UNIX_MS as i128) as f64) / HALF_WEEK_MS)
}

/// Converts Unix timestamp in milliseconds (`f64`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_ms_f64(ms: f64) -> f64 {
    normalize_zero((ms - EPOCH_UNIX_MS as f64) / HALF_WEEK_MS)
}

/// Converts integer Unix timestamp in microseconds (`i128`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_us_i128(us: i128) -> f64 {
    normalize_zero(((us - EPOCH_UNIX_US) as f64) / HALF_WEEK_US)
}

/// Converts integer Unix timestamp in nanoseconds (`i128`) to everdude half-weeks (`h`).
#[inline(always)]
pub fn from_unix_ns_i128(ns: i128) -> f64 {
    normalize_zero(((ns - EPOCH_UNIX_NS) as f64) / HALF_WEEK_NS)
}

/// Converts everdude half-weeks (`h`) to Unix timestamp in seconds (`f64`).
#[inline(always)]
pub fn to_unix_secs(h: f64) -> f64 {
    normalize_zero(h * HALF_WEEK_SECS + EPOCH_UNIX_SECS as f64)
}

/// Converts everdude half-weeks (`h`) to Unix timestamp in milliseconds (`f64`).
#[inline(always)]
pub fn to_unix_ms(h: f64) -> f64 {
    normalize_zero(h * HALF_WEEK_MS + EPOCH_UNIX_MS as f64)
}

#[cfg(feature = "capi")]
pub mod ffi {
    use super::*;

    #[unsafe(no_mangle)]
    pub extern "C" fn everdude_from_unix_secs(secs: f64) -> f64 {
        from_unix_secs(secs)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn everdude_from_unix_ms(ms: i64) -> f64 {
        from_unix_ms(ms)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn everdude_to_unix_secs(h: f64) -> f64 {
        to_unix_secs(h)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn everdude_to_unix_ms(h: f64) -> f64 {
        to_unix_ms(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_is_zero() {
        assert_eq!(from_unix_secs(978_307_200.0), 0.0);
        assert_eq!(from_unix_secs_i64(978_307_200), 0.0);
        assert_eq!(from_unix_ms(978_307_200_000), 0.0);
        assert_eq!(from_unix_us_i128(978_307_200_000_000), 0.0);
        assert_eq!(from_unix_ns_i128(978_307_200_000_000_000), 0.0);
        assert_eq!(to_unix_secs(0.0), 978_307_200.0);
        assert_eq!(to_unix_ms(0.0), 978_307_200_000.0);
    }

    #[test]
    fn half_week_parity_matches_days() {
        // h = 1 -> 2001-01-04 12:00:00 UTC (Thursday 12:00)
        assert_eq!(from_unix_secs_i64(978_307_200 + 302_400), 1.0);
        assert_eq!(to_unix_secs(1.0), 978_609_600.0);

        // h = 2 -> 2001-01-08 00:00:00 UTC (Monday 00:00)
        assert_eq!(from_unix_secs_i64(978_307_200 + 604_800), 2.0);
        assert_eq!(to_unix_secs(2.0), 978_912_000.0);

        // h = -1 -> 2000-12-28 12:00:00 UTC (Thursday 12:00)
        assert_eq!(from_unix_secs_i64(978_307_200 - 302_400), -1.0);
        assert_eq!(to_unix_secs(-1.0), 978_004_800.0);
    }

    #[test]
    fn no_negative_zero() {
        assert!(!from_unix_secs(978_307_200.0).is_sign_negative());
        assert!(!to_unix_secs(0.0).is_sign_negative());
    }

    #[test]
    fn roundtrip_ms_and_secs() {
        let samples = [0i64, 978_307_200, 1_700_000_000, 1_795_189_200, -100_000];
        for s in samples {
            let h = from_unix_secs_i64(s);
            let back = to_unix_secs(h);
            assert!((back - s as f64).abs() < 1e-6);
        }
    }
}
