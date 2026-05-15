/// Basic image dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy)]
enum Rounding {
    Floor,
    Nearest,
    Ceil,
}

impl ImageSize {
    /// Builds a size value directly.
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns true when either dimension is zero.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns the pixel area using a wide integer.
    #[must_use]
    pub const fn area(self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }
}

fn scale_value(numerator: u64, denominator: u32, rounding: Rounding) -> u32 {
    let denominator = u64::from(denominator);
    let value = match rounding {
        Rounding::Floor => numerator / denominator,
        Rounding::Nearest => (numerator + (denominator / 2)) / denominator,
        Rounding::Ceil => numerator.div_ceil(denominator),
    };

    value as u32
}

fn scale_by_width(size: ImageSize, width: u32, rounding: Rounding) -> Option<ImageSize> {
    if size.is_empty() {
        return None;
    }

    if width == 0 {
        return Some(ImageSize::default());
    }

    let height = scale_value(
        u64::from(size.height) * u64::from(width),
        size.width,
        rounding,
    )
    .max(1);

    Some(ImageSize::new(width, height))
}

fn scale_by_height(size: ImageSize, height: u32, rounding: Rounding) -> Option<ImageSize> {
    if size.is_empty() {
        return None;
    }

    if height == 0 {
        return Some(ImageSize::default());
    }

    let width = scale_value(
        u64::from(size.width) * u64::from(height),
        size.height,
        rounding,
    )
    .max(1);

    Some(ImageSize::new(width, height))
}

/// Scales a size to fit completely within bounds, preserving aspect ratio.
#[must_use]
pub fn fit_within(size: ImageSize, bounds: ImageSize) -> ImageSize {
    if size.is_empty() || bounds.is_empty() {
        return ImageSize::default();
    }

    let use_width = u64::from(bounds.width) * u64::from(size.height)
        <= u64::from(bounds.height) * u64::from(size.width);

    if use_width {
        scale_by_width(size, bounds.width, Rounding::Floor).unwrap_or_default()
    } else {
        scale_by_height(size, bounds.height, Rounding::Floor).unwrap_or_default()
    }
}

/// Scales a size so it fully covers bounds, preserving aspect ratio.
#[must_use]
pub fn cover_size(size: ImageSize, bounds: ImageSize) -> ImageSize {
    if size.is_empty() || bounds.is_empty() {
        return ImageSize::default();
    }

    let use_width = u64::from(bounds.width) * u64::from(size.height)
        >= u64::from(bounds.height) * u64::from(size.width);

    if use_width {
        scale_by_width(size, bounds.width, Rounding::Ceil).unwrap_or_default()
    } else {
        scale_by_height(size, bounds.height, Rounding::Ceil).unwrap_or_default()
    }
}

/// Scales a size to an exact width and rounded matching height.
#[must_use]
pub fn scale_to_width(size: ImageSize, width: u32) -> Option<ImageSize> {
    scale_by_width(size, width, Rounding::Nearest)
}

/// Scales a size to an exact height and rounded matching width.
#[must_use]
pub fn scale_to_height(size: ImageSize, height: u32) -> Option<ImageSize> {
    scale_by_height(size, height, Rounding::Nearest)
}
