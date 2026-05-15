#![forbid(unsafe_code)]
//! Primitive media dimension helpers.
//!
//! These helpers keep width and height calculations explicit and validated.
//!
//! # Examples
//!
//! ```rust
//! use use_dimensions::{Dimensions, is_landscape, pixel_area};
//!
//! let size = Dimensions::new(1920, 1080).unwrap();
//!
//! assert_eq!(size.area(), 2_073_600);
//! assert!(size.is_landscape());
//! assert_eq!(pixel_area(1920, 1080).unwrap(), 2_073_600);
//! assert!(is_landscape(1920, 1080).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionsError {
    InvalidWidth,
    InvalidHeight,
}

fn validate_dimensions(width: u32, height: u32) -> Result<(u32, u32), DimensionsError> {
    if width == 0 {
        return Err(DimensionsError::InvalidWidth);
    }

    if height == 0 {
        return Err(DimensionsError::InvalidHeight);
    }

    Ok((width, height))
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Result<Self, DimensionsError> {
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
    pub fn area(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
    }

    #[must_use]
    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    #[must_use]
    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

pub fn pixel_area(width: u32, height: u32) -> Result<u64, DimensionsError> {
    Ok(Dimensions::new(width, height)?.area())
}

pub fn is_landscape(width: u32, height: u32) -> Result<bool, DimensionsError> {
    Ok(Dimensions::new(width, height)?.is_landscape())
}

pub fn is_portrait(width: u32, height: u32) -> Result<bool, DimensionsError> {
    Ok(Dimensions::new(width, height)?.is_portrait())
}

pub fn is_square(width: u32, height: u32) -> Result<bool, DimensionsError> {
    Ok(Dimensions::new(width, height)?.is_square())
}

#[cfg(test)]
mod tests {
    use super::{Dimensions, DimensionsError, is_landscape, is_portrait, is_square, pixel_area};

    #[test]
    fn validates_dimensions_and_area() {
        let dimensions = Dimensions::new(1920, 1080).unwrap();

        assert_eq!(dimensions.width(), 1920);
        assert_eq!(dimensions.height(), 1080);
        assert_eq!(dimensions.area(), 2_073_600);
        assert!(dimensions.is_landscape());
        assert!(!dimensions.is_portrait());
        assert!(!dimensions.is_square());
        assert_eq!(pixel_area(1920, 1080).unwrap(), 2_073_600);
        assert!(is_landscape(1920, 1080).unwrap());
    }

    #[test]
    fn detects_portrait_and_square_sizes() {
        assert!(is_portrait(1080, 1920).unwrap());
        assert!(is_square(512, 512).unwrap());
        assert!(!is_square(640, 480).unwrap());
    }

    #[test]
    fn rejects_invalid_dimensions() {
        assert_eq!(Dimensions::new(0, 1080), Err(DimensionsError::InvalidWidth));
        assert_eq!(
            Dimensions::new(1920, 0),
            Err(DimensionsError::InvalidHeight)
        );
        assert_eq!(pixel_area(0, 1), Err(DimensionsError::InvalidWidth));
    }
}
