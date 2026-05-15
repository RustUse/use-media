#![forbid(unsafe_code)]
//! Primitive resolution helpers.
//!
//! These helpers classify common resolutions and perform small scaling
//! calculations without decoding media.
//!
//! # Examples
//!
//! ```rust
//! use use_resolution::{ResolutionClass, classify_resolution, fit_within, megapixels};
//!
//! assert_eq!(classify_resolution(1920, 1080).unwrap(), ResolutionClass::FullHd);
//! assert!((megapixels(1920, 1080).unwrap() - 2.0736).abs() < 1.0e-12);
//! assert_eq!(fit_within(3840, 2160, 1920, 1080).unwrap(), (1920, 1080));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionClass {
    Sd,
    Hd,
    FullHd,
    QuadHd,
    UltraHd4k,
    UltraHd8k,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionError {
    InvalidWidth,
    InvalidHeight,
    InvalidMaxWidth,
    InvalidMaxHeight,
    InvalidScale,
    ScaledOutOfRange,
}

fn validate_dimensions(width: u32, height: u32) -> Result<(u32, u32), ResolutionError> {
    if width == 0 {
        return Err(ResolutionError::InvalidWidth);
    }

    if height == 0 {
        return Err(ResolutionError::InvalidHeight);
    }

    Ok((width, height))
}

fn validate_max_dimensions(max_width: u32, max_height: u32) -> Result<(u32, u32), ResolutionError> {
    if max_width == 0 {
        return Err(ResolutionError::InvalidMaxWidth);
    }

    if max_height == 0 {
        return Err(ResolutionError::InvalidMaxHeight);
    }

    Ok((max_width, max_height))
}

fn validate_scale(scale: f64) -> Result<f64, ResolutionError> {
    if !scale.is_finite() || scale <= 0.0 {
        Err(ResolutionError::InvalidScale)
    } else {
        Ok(scale)
    }
}

pub fn classify_resolution(width: u32, height: u32) -> Result<ResolutionClass, ResolutionError> {
    let (width, height) = validate_dimensions(width, height)?;

    Ok(if width >= 7_680 && height >= 4_320 {
        ResolutionClass::UltraHd8k
    } else if width >= 3_840 && height >= 2_160 {
        ResolutionClass::UltraHd4k
    } else if width >= 2_560 && height >= 1_440 {
        ResolutionClass::QuadHd
    } else if width >= 1_920 && height >= 1_080 {
        ResolutionClass::FullHd
    } else if width >= 1_280 && height >= 720 {
        ResolutionClass::Hd
    } else if width >= 640 && height >= 480 {
        ResolutionClass::Sd
    } else {
        ResolutionClass::Custom
    })
}

pub fn megapixels(width: u32, height: u32) -> Result<f64, ResolutionError> {
    Ok(pixels(width, height)? as f64 / 1_000_000.0)
}

pub fn pixels(width: u32, height: u32) -> Result<u64, ResolutionError> {
    let (width, height) = validate_dimensions(width, height)?;
    Ok(u64::from(width) * u64::from(height))
}

pub fn scale_dimensions(
    width: u32,
    height: u32,
    scale: f64,
) -> Result<(u32, u32), ResolutionError> {
    let (width, height) = validate_dimensions(width, height)?;
    let scale = validate_scale(scale)?;
    let scaled_width = (f64::from(width) * scale).round();
    let scaled_height = (f64::from(height) * scale).round();

    if !(1.0..=f64::from(u32::MAX)).contains(&scaled_width)
        || !(1.0..=f64::from(u32::MAX)).contains(&scaled_height)
    {
        return Err(ResolutionError::ScaledOutOfRange);
    }

    Ok((scaled_width as u32, scaled_height as u32))
}

pub fn fit_within(
    width: u32,
    height: u32,
    max_width: u32,
    max_height: u32,
) -> Result<(u32, u32), ResolutionError> {
    let (width, height) = validate_dimensions(width, height)?;
    let (max_width, max_height) = validate_max_dimensions(max_width, max_height)?;

    if width <= max_width && height <= max_height {
        return Ok((width, height));
    }

    let scale =
        (f64::from(max_width) / f64::from(width)).min(f64::from(max_height) / f64::from(height));
    scale_dimensions(width, height, scale)
}

#[cfg(test)]
mod tests {
    use super::{
        ResolutionClass, ResolutionError, classify_resolution, fit_within, megapixels, pixels,
        scale_dimensions,
    };

    #[test]
    fn classifies_common_resolution_bands() {
        assert_eq!(classify_resolution(640, 480).unwrap(), ResolutionClass::Sd);
        assert_eq!(classify_resolution(1280, 720).unwrap(), ResolutionClass::Hd);
        assert_eq!(
            classify_resolution(1920, 1080).unwrap(),
            ResolutionClass::FullHd
        );
        assert_eq!(
            classify_resolution(3840, 2160).unwrap(),
            ResolutionClass::UltraHd4k
        );
        assert_eq!(
            classify_resolution(300, 200).unwrap(),
            ResolutionClass::Custom
        );
    }

    #[test]
    fn computes_pixel_and_scaling_helpers() {
        assert_eq!(pixels(1920, 1080).unwrap(), 2_073_600);
        assert!((megapixels(1920, 1080).unwrap() - 2.0736).abs() < 1.0e-12);
        assert_eq!(scale_dimensions(1920, 1080, 0.5).unwrap(), (960, 540));
        assert_eq!(fit_within(3840, 2160, 1920, 1080).unwrap(), (1920, 1080));
        assert_eq!(fit_within(1920, 1080, 1000, 1000).unwrap(), (1000, 563));
    }

    #[test]
    fn rejects_invalid_resolution_inputs() {
        assert_eq!(
            classify_resolution(0, 1080),
            Err(ResolutionError::InvalidWidth)
        );
        assert_eq!(
            scale_dimensions(1920, 1080, 0.0),
            Err(ResolutionError::InvalidScale)
        );
        assert_eq!(
            fit_within(1920, 1080, 0, 1080),
            Err(ResolutionError::InvalidMaxWidth)
        );
    }
}
