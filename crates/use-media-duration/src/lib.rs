#![forbid(unsafe_code)]
//! Primitive media duration helpers.
//!
//! These helpers keep durations explicit without requiring a broader time
//! framework.
//!
//! # Examples
//!
//! ```rust
//! use use_media_duration::{MediaDuration, duration_from_samples, format_duration_hms};
//!
//! let duration = MediaDuration::new(90.5).unwrap();
//!
//! assert_eq!(duration.millis(), 90_500);
//! assert_eq!(format_duration_hms(3661.0).unwrap(), "01:01:01");
//! assert!((duration_from_samples(96_000, 48_000.0).unwrap() - 2.0).abs() < 1.0e-12);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MediaDuration {
    seconds: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaDurationError {
    InvalidSeconds,
    InvalidSampleRate,
}

fn validate_seconds(seconds: f64) -> Result<f64, MediaDurationError> {
    if !seconds.is_finite() || seconds < 0.0 {
        Err(MediaDurationError::InvalidSeconds)
    } else {
        Ok(seconds)
    }
}

fn validate_sample_rate(sample_rate_hz: f64) -> Result<f64, MediaDurationError> {
    if !sample_rate_hz.is_finite() || sample_rate_hz <= 0.0 {
        Err(MediaDurationError::InvalidSampleRate)
    } else {
        Ok(sample_rate_hz)
    }
}

impl MediaDuration {
    pub fn new(seconds: f64) -> Result<Self, MediaDurationError> {
        Ok(Self {
            seconds: validate_seconds(seconds)?,
        })
    }

    #[must_use]
    pub fn seconds(&self) -> f64 {
        self.seconds
    }

    #[must_use]
    pub fn millis(&self) -> u64 {
        (self.seconds * 1_000.0).round() as u64
    }

    #[must_use]
    pub fn minutes(&self) -> f64 {
        self.seconds / 60.0
    }

    #[must_use]
    pub fn hours(&self) -> f64 {
        self.seconds / 3_600.0
    }
}

pub fn seconds_to_millis(seconds: f64) -> Result<u64, MediaDurationError> {
    Ok((validate_seconds(seconds)? * 1_000.0).round() as u64)
}

#[must_use]
pub fn millis_to_seconds(millis: u64) -> f64 {
    millis as f64 / 1_000.0
}

pub fn format_duration_hms(seconds: f64) -> Result<String, MediaDurationError> {
    let total_seconds = validate_seconds(seconds)?.floor() as u64;
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    Ok(format!("{hours:02}:{minutes:02}:{seconds:02}"))
}

pub fn duration_from_samples(
    sample_count: usize,
    sample_rate_hz: f64,
) -> Result<f64, MediaDurationError> {
    Ok(sample_count as f64 / validate_sample_rate(sample_rate_hz)?)
}

#[cfg(test)]
mod tests {
    use super::{
        MediaDuration, MediaDurationError, duration_from_samples, format_duration_hms,
        millis_to_seconds, seconds_to_millis,
    };

    #[test]
    fn computes_duration_helpers() {
        let duration = MediaDuration::new(90.5).unwrap();

        assert_eq!(duration.seconds(), 90.5);
        assert_eq!(duration.millis(), 90_500);
        assert!((duration.minutes() - 1.508_333_333_333_333_3).abs() < 1.0e-12);
        assert!((duration.hours() - 0.025_138_888_888_888_89).abs() < 1.0e-12);
        assert_eq!(seconds_to_millis(1.5).unwrap(), 1_500);
        assert_eq!(millis_to_seconds(1_500), 1.5);
    }

    #[test]
    fn formats_duration_and_samples() {
        assert_eq!(format_duration_hms(3661.0).unwrap(), "01:01:01");
        assert!((duration_from_samples(96_000, 48_000.0).unwrap() - 2.0).abs() < 1.0e-12);
        assert_eq!(duration_from_samples(0, 48_000.0).unwrap(), 0.0);
    }

    #[test]
    fn rejects_invalid_duration_inputs() {
        assert_eq!(
            MediaDuration::new(-1.0),
            Err(MediaDurationError::InvalidSeconds)
        );
        assert_eq!(
            seconds_to_millis(f64::NAN),
            Err(MediaDurationError::InvalidSeconds)
        );
        assert_eq!(
            format_duration_hms(-0.1),
            Err(MediaDurationError::InvalidSeconds)
        );
        assert_eq!(
            duration_from_samples(10, 0.0),
            Err(MediaDurationError::InvalidSampleRate)
        );
    }
}
