#![forbid(unsafe_code)]
//! Primitive frame-rate helpers.
//!
//! These helpers keep frame duration and frame-count calculations explicit.
//!
//! # Examples
//!
//! ```rust
//! use use_frame_rate::{FrameRate, frame_count, is_standard_frame_rate};
//!
//! let rate = FrameRate::new(24.0).unwrap();
//!
//! assert!((rate.frame_duration_seconds() - (1.0 / 24.0)).abs() < 1.0e-12);
//! assert_eq!(frame_count(2.5, 24.0).unwrap(), 60);
//! assert!(is_standard_frame_rate(23.976));
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameRate {
    fps: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRateError {
    InvalidFps,
    InvalidDuration,
}

fn validate_fps(fps: f64) -> Result<f64, FrameRateError> {
    if !fps.is_finite() || fps <= 0.0 {
        Err(FrameRateError::InvalidFps)
    } else {
        Ok(fps)
    }
}

fn validate_duration(duration_seconds: f64) -> Result<f64, FrameRateError> {
    if !duration_seconds.is_finite() || duration_seconds < 0.0 {
        Err(FrameRateError::InvalidDuration)
    } else {
        Ok(duration_seconds)
    }
}

impl FrameRate {
    pub fn new(fps: f64) -> Result<Self, FrameRateError> {
        Ok(Self {
            fps: validate_fps(fps)?,
        })
    }

    #[must_use]
    pub fn fps(&self) -> f64 {
        self.fps
    }

    #[must_use]
    pub fn frame_duration_seconds(&self) -> f64 {
        1.0 / self.fps
    }

    #[must_use]
    pub fn frame_duration_millis(&self) -> f64 {
        self.frame_duration_seconds() * 1_000.0
    }
}

pub fn frame_duration_seconds(fps: f64) -> Result<f64, FrameRateError> {
    Ok(1.0 / validate_fps(fps)?)
}

pub fn frame_count(duration_seconds: f64, fps: f64) -> Result<u64, FrameRateError> {
    let duration_seconds = validate_duration(duration_seconds)?;
    let fps = validate_fps(fps)?;
    Ok((duration_seconds * fps).round() as u64)
}

pub fn duration_from_frames(frame_count: u64, fps: f64) -> Result<f64, FrameRateError> {
    Ok(frame_count as f64 / validate_fps(fps)?)
}

#[must_use]
pub fn is_standard_frame_rate(fps: f64) -> bool {
    const STANDARD_FRAME_RATES: [f64; 9] =
        [23.976, 24.0, 25.0, 29.97, 30.0, 50.0, 59.94, 60.0, 120.0];

    fps.is_finite()
        && STANDARD_FRAME_RATES
            .into_iter()
            .any(|standard| (fps - standard).abs() <= 0.01)
}

#[cfg(test)]
mod tests {
    use super::{
        FrameRate, FrameRateError, duration_from_frames, frame_count, frame_duration_seconds,
        is_standard_frame_rate,
    };

    #[test]
    fn computes_frame_durations_and_counts() {
        let rate = FrameRate::new(24.0).unwrap();

        assert_eq!(rate.fps(), 24.0);
        assert!((rate.frame_duration_seconds() - (1.0 / 24.0)).abs() < 1.0e-12);
        assert!((rate.frame_duration_millis() - 41.666_666_666_666_664).abs() < 1.0e-12);
        assert_eq!(frame_count(2.5, 24.0).unwrap(), 60);
        assert!((duration_from_frames(60, 24.0).unwrap() - 2.5).abs() < 1.0e-12);
        assert!((frame_duration_seconds(25.0).unwrap() - 0.04).abs() < 1.0e-12);
    }

    #[test]
    fn detects_standard_frame_rates() {
        assert!(is_standard_frame_rate(23.976));
        assert!(is_standard_frame_rate(23.98));
        assert!(is_standard_frame_rate(60.0));
        assert!(!is_standard_frame_rate(27.0));
    }

    #[test]
    fn rejects_invalid_frame_rate_inputs() {
        assert_eq!(FrameRate::new(0.0), Err(FrameRateError::InvalidFps));
        assert_eq!(
            frame_count(-1.0, 24.0),
            Err(FrameRateError::InvalidDuration)
        );
        assert_eq!(
            duration_from_frames(10, f64::NAN),
            Err(FrameRateError::InvalidFps)
        );
    }
}
