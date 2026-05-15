#![forbid(unsafe_code)]
//! Primitive aspect-ratio helpers.
//!
//! These helpers validate dimensions, simplify ratios, and perform small
//! tolerance checks.
//!
//! # Examples
//!
//! ```rust
//! use use_aspect_ratio::{AspectRatio, aspect_label, fits_aspect_ratio};
//!
//! let ratio = AspectRatio::new(1920, 1080).unwrap();
//!
//! assert!((ratio.ratio() - (16.0 / 9.0)).abs() < 1.0e-12);
//! assert_eq!(ratio.simplified().label(), "16:9");
//! assert_eq!(aspect_label(1920, 1080).unwrap(), "16:9");
//! assert!(fits_aspect_ratio(1920, 1080, 1280, 720, 0.001).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AspectRatio {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatioError {
    InvalidWidth,
    InvalidHeight,
    InvalidTolerance,
}

fn gcd(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

fn validate_dimensions(width: u32, height: u32) -> Result<(u32, u32), AspectRatioError> {
    if width == 0 {
        return Err(AspectRatioError::InvalidWidth);
    }

    if height == 0 {
        return Err(AspectRatioError::InvalidHeight);
    }

    Ok((width, height))
}

fn validate_tolerance(tolerance: f64) -> Result<f64, AspectRatioError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        Err(AspectRatioError::InvalidTolerance)
    } else {
        Ok(tolerance)
    }
}

impl AspectRatio {
    pub fn new(width: u32, height: u32) -> Result<Self, AspectRatioError> {
        let (width, height) = validate_dimensions(width, height)?;
        Ok(Self { width, height })
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub fn ratio(&self) -> f64 {
        f64::from(self.width) / f64::from(self.height)
    }

    #[must_use]
    pub fn simplified(&self) -> Self {
        let divisor = gcd(self.width, self.height);

        Self {
            width: self.width / divisor,
            height: self.height / divisor,
        }
    }

    #[must_use]
    pub fn label(&self) -> String {
        let simplified = self.simplified();
        format!("{}:{}", simplified.width, simplified.height)
    }
}

pub fn aspect_ratio(width: u32, height: u32) -> Result<f64, AspectRatioError> {
    Ok(AspectRatio::new(width, height)?.ratio())
}

pub fn simplify_ratio(width: u32, height: u32) -> Result<(u32, u32), AspectRatioError> {
    let simplified = AspectRatio::new(width, height)?.simplified();
    Ok((simplified.width, simplified.height))
}

pub fn aspect_label(width: u32, height: u32) -> Result<String, AspectRatioError> {
    Ok(AspectRatio::new(width, height)?.label())
}

pub fn fits_aspect_ratio(
    width: u32,
    height: u32,
    target_width: u32,
    target_height: u32,
    tolerance: f64,
) -> Result<bool, AspectRatioError> {
    let tolerance = validate_tolerance(tolerance)?;
    let source = aspect_ratio(width, height)?;
    let target = aspect_ratio(target_width, target_height)?;

    Ok((source - target).abs() <= tolerance)
}

#[cfg(test)]
mod tests {
    use super::{AspectRatio, AspectRatioError, aspect_label, fits_aspect_ratio, simplify_ratio};

    #[test]
    fn simplifies_common_aspect_ratios() {
        let aspect = AspectRatio::new(1920, 1080).unwrap();

        assert_eq!(aspect.width(), 1920);
        assert_eq!(aspect.height(), 1080);
        assert!((aspect.ratio() - (16.0 / 9.0)).abs() < 1.0e-12);
        assert_eq!(aspect.simplified(), AspectRatio::new(16, 9).unwrap());
        assert_eq!(simplify_ratio(1920, 1080).unwrap(), (16, 9));
        assert_eq!(aspect_label(1920, 1080).unwrap(), "16:9");
    }

    #[test]
    fn checks_ratio_tolerance_matches() {
        assert!(fits_aspect_ratio(1920, 1080, 1280, 720, 0.001).unwrap());
        assert!(!fits_aspect_ratio(1920, 1080, 1024, 768, 0.001).unwrap());
    }

    #[test]
    fn rejects_invalid_ratio_inputs() {
        assert_eq!(
            AspectRatio::new(0, 1080),
            Err(AspectRatioError::InvalidWidth)
        );
        assert_eq!(
            AspectRatio::new(1920, 0),
            Err(AspectRatioError::InvalidHeight)
        );
        assert_eq!(
            fits_aspect_ratio(1920, 1080, 1280, 720, f64::NAN),
            Err(AspectRatioError::InvalidTolerance)
        );
    }
}
