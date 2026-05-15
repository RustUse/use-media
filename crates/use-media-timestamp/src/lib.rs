#![forbid(unsafe_code)]
//! Primitive media timestamp helpers.
//!
//! These helpers keep timestamps explicit and frame conversion deterministic.
//!
//! # Examples
//!
//! ```rust
//! use use_media_timestamp::{MediaTimestamp, clamp_timestamp, timestamp_from_frame};
//!
//! let timestamp = timestamp_from_frame(60, 30.0).unwrap();
//!
//! assert_eq!(timestamp, MediaTimestamp::new(2.0).unwrap());
//! assert_eq!(timestamp.frame_index(30.0).unwrap(), 60);
//! assert_eq!(clamp_timestamp(12.0, 10.0).unwrap(), 10.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MediaTimestamp {
    seconds: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTimestampError {
    InvalidSeconds,
    InvalidDuration,
    InvalidFps,
}

fn validate_seconds(seconds: f64) -> Result<f64, MediaTimestampError> {
    if !seconds.is_finite() || seconds < 0.0 {
        Err(MediaTimestampError::InvalidSeconds)
    } else {
        Ok(seconds)
    }
}

fn validate_duration(duration_seconds: f64) -> Result<f64, MediaTimestampError> {
    if !duration_seconds.is_finite() || duration_seconds < 0.0 {
        Err(MediaTimestampError::InvalidDuration)
    } else {
        Ok(duration_seconds)
    }
}

fn validate_fps(fps: f64) -> Result<f64, MediaTimestampError> {
    if !fps.is_finite() || fps <= 0.0 {
        Err(MediaTimestampError::InvalidFps)
    } else {
        Ok(fps)
    }
}

impl MediaTimestamp {
    pub fn new(seconds: f64) -> Result<Self, MediaTimestampError> {
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

    pub fn frame_index(&self, fps: f64) -> Result<u64, MediaTimestampError> {
        frame_index_at_time(self.seconds, fps)
    }
}

pub fn timestamp_from_frame(
    frame_index: u64,
    fps: f64,
) -> Result<MediaTimestamp, MediaTimestampError> {
    MediaTimestamp::new(frame_index as f64 / validate_fps(fps)?)
}

pub fn frame_index_at_time(seconds: f64, fps: f64) -> Result<u64, MediaTimestampError> {
    let seconds = validate_seconds(seconds)?;
    let fps = validate_fps(fps)?;
    Ok((seconds * fps).floor() as u64)
}

pub fn clamp_timestamp(seconds: f64, duration_seconds: f64) -> Result<f64, MediaTimestampError> {
    let seconds = validate_seconds(seconds)?;
    let duration_seconds = validate_duration(duration_seconds)?;
    Ok(seconds.min(duration_seconds))
}

pub fn format_timestamp(seconds: f64) -> Result<String, MediaTimestampError> {
    let total_millis = (validate_seconds(seconds)? * 1_000.0).round() as u64;
    let hours = total_millis / 3_600_000;
    let minutes = (total_millis % 3_600_000) / 60_000;
    let seconds = (total_millis % 60_000) / 1_000;
    let millis = total_millis % 1_000;

    Ok(format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}"))
}

#[cfg(test)]
mod tests {
    use super::{
        MediaTimestamp, MediaTimestampError, clamp_timestamp, format_timestamp,
        frame_index_at_time, timestamp_from_frame,
    };

    #[test]
    fn converts_between_frames_and_timestamps() {
        let timestamp = timestamp_from_frame(60, 30.0).unwrap();

        assert_eq!(timestamp, MediaTimestamp::new(2.0).unwrap());
        assert_eq!(timestamp.seconds(), 2.0);
        assert_eq!(timestamp.millis(), 2_000);
        assert_eq!(timestamp.frame_index(30.0).unwrap(), 60);
        assert_eq!(frame_index_at_time(2.5, 24.0).unwrap(), 60);
    }

    #[test]
    fn clamps_and_formats_timestamps() {
        assert_eq!(clamp_timestamp(12.0, 10.0).unwrap(), 10.0);
        assert_eq!(clamp_timestamp(8.0, 10.0).unwrap(), 8.0);
        assert_eq!(format_timestamp(3661.25).unwrap(), "01:01:01.250");
    }

    #[test]
    fn rejects_invalid_timestamp_inputs() {
        assert_eq!(
            MediaTimestamp::new(-1.0),
            Err(MediaTimestampError::InvalidSeconds)
        );
        assert_eq!(
            timestamp_from_frame(60, 0.0),
            Err(MediaTimestampError::InvalidFps)
        );
        assert_eq!(
            frame_index_at_time(f64::NAN, 30.0),
            Err(MediaTimestampError::InvalidSeconds)
        );
        assert_eq!(
            clamp_timestamp(1.0, -1.0),
            Err(MediaTimestampError::InvalidDuration)
        );
    }
}
