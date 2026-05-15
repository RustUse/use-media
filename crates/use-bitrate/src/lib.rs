#![forbid(unsafe_code)]
//! Primitive bitrate helpers.
//!
//! These helpers perform small size and duration calculations for media
//! metadata.
//!
//! # Examples
//!
//! ```rust
//! use use_bitrate::{Bitrate, bitrate_from_size, bytes_for_duration};
//!
//! let bitrate = Bitrate::new(8_000_000).unwrap();
//!
//! assert_eq!(bytes_for_duration(8_000_000, 10.0).unwrap(), 10_000_000);
//! assert_eq!(bitrate_from_size(10_000_000, 10.0).unwrap(), 8_000_000);
//! assert_eq!(bitrate.bits_per_second(), 8_000_000);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitrate {
    bits_per_second: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitrateError {
    InvalidBitrate,
    InvalidDuration,
}

fn validate_bitrate(bits_per_second: u64) -> Result<u64, BitrateError> {
    if bits_per_second == 0 {
        Err(BitrateError::InvalidBitrate)
    } else {
        Ok(bits_per_second)
    }
}

fn validate_duration(duration_seconds: f64) -> Result<f64, BitrateError> {
    if !duration_seconds.is_finite() || duration_seconds <= 0.0 {
        Err(BitrateError::InvalidDuration)
    } else {
        Ok(duration_seconds)
    }
}

impl Bitrate {
    pub fn new(bits_per_second: u64) -> Result<Self, BitrateError> {
        Ok(Self {
            bits_per_second: validate_bitrate(bits_per_second)?,
        })
    }

    #[must_use]
    pub fn bits_per_second(&self) -> u64 {
        self.bits_per_second
    }

    #[must_use]
    pub fn kilobits_per_second(&self) -> f64 {
        kilobits_per_second(self.bits_per_second)
    }

    #[must_use]
    pub fn megabits_per_second(&self) -> f64 {
        megabits_per_second(self.bits_per_second)
    }
}

pub fn bits_for_duration(bits_per_second: u64, duration_seconds: f64) -> Result<u64, BitrateError> {
    let bits_per_second = validate_bitrate(bits_per_second)?;
    let duration_seconds = validate_duration(duration_seconds)?;
    Ok((bits_per_second as f64 * duration_seconds).round() as u64)
}

pub fn bytes_for_duration(
    bits_per_second: u64,
    duration_seconds: f64,
) -> Result<u64, BitrateError> {
    let bits_per_second = validate_bitrate(bits_per_second)?;
    let duration_seconds = validate_duration(duration_seconds)?;
    Ok(((bits_per_second as f64 * duration_seconds) / 8.0).round() as u64)
}

pub fn bitrate_from_size(size_bytes: u64, duration_seconds: f64) -> Result<u64, BitrateError> {
    let duration_seconds = validate_duration(duration_seconds)?;
    Ok(((size_bytes as f64 * 8.0) / duration_seconds).round() as u64)
}

#[must_use]
pub fn kilobits_per_second(bits_per_second: u64) -> f64 {
    bits_per_second as f64 / 1_000.0
}

#[must_use]
pub fn megabits_per_second(bits_per_second: u64) -> f64 {
    bits_per_second as f64 / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::{
        Bitrate, BitrateError, bitrate_from_size, bits_for_duration, bytes_for_duration,
        kilobits_per_second, megabits_per_second,
    };

    #[test]
    fn computes_bitrate_size_helpers() {
        let bitrate = Bitrate::new(8_000_000).unwrap();

        assert_eq!(bitrate.bits_per_second(), 8_000_000);
        assert_eq!(bits_for_duration(8_000_000, 10.0).unwrap(), 80_000_000);
        assert_eq!(bytes_for_duration(8_000_000, 10.0).unwrap(), 10_000_000);
        assert_eq!(bitrate_from_size(10_000_000, 10.0).unwrap(), 8_000_000);
        assert_eq!(kilobits_per_second(8_000_000), 8_000.0);
        assert_eq!(megabits_per_second(8_000_000), 8.0);
        assert_eq!(bitrate.kilobits_per_second(), 8_000.0);
        assert_eq!(bitrate.megabits_per_second(), 8.0);
    }

    #[test]
    fn rejects_invalid_bitrate_inputs() {
        assert_eq!(Bitrate::new(0), Err(BitrateError::InvalidBitrate));
        assert_eq!(
            bits_for_duration(0, 10.0),
            Err(BitrateError::InvalidBitrate)
        );
        assert_eq!(
            bytes_for_duration(8_000_000, 0.0),
            Err(BitrateError::InvalidDuration)
        );
        assert_eq!(
            bitrate_from_size(10_000_000, f64::NAN),
            Err(BitrateError::InvalidDuration)
        );
    }
}
