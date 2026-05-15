use core::fmt;

use crate::size::ImageSize;

/// Common image orientation buckets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageOrientation {
    Landscape,
    Portrait,
    Square,
    #[default]
    Unknown,
}

impl fmt::Display for ImageOrientation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Landscape => "landscape",
            Self::Portrait => "portrait",
            Self::Square => "square",
            Self::Unknown => "unknown",
        })
    }
}

/// Detects the orientation for a size.
#[must_use]
pub const fn orientation(size: ImageSize) -> ImageOrientation {
    if size.width == 0 || size.height == 0 {
        ImageOrientation::Unknown
    } else if size.width > size.height {
        ImageOrientation::Landscape
    } else if size.height > size.width {
        ImageOrientation::Portrait
    } else {
        ImageOrientation::Square
    }
}
