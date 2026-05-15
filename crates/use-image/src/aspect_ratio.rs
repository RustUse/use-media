use crate::size::ImageSize;

/// Integer image aspect ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ImageAspectRatio {
    pub width: u32,
    pub height: u32,
}

impl ImageAspectRatio {
    /// Builds a ratio value directly.
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn gcd(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

/// Simplifies a size into its smallest whole-number aspect ratio.
#[must_use]
pub fn aspect_ratio(size: ImageSize) -> ImageAspectRatio {
    let divisor = gcd(size.width, size.height);

    if divisor == 0 {
        return ImageAspectRatio::new(size.width, size.height);
    }

    ImageAspectRatio::new(size.width / divisor, size.height / divisor)
}

/// Returns the floating-point aspect ratio when the height is non-zero.
#[must_use]
pub fn aspect_ratio_f64(size: ImageSize) -> Option<f64> {
    if size.height == 0 {
        return None;
    }

    Some(f64::from(size.width) / f64::from(size.height))
}
