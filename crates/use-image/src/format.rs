use crate::{extension::image_extension, mime::image_mime_type, size::ImageSize};

/// Common image formats handled by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
    Gif,
    Svg,
    Ico,
    Bmp,
    Tiff,
    Avif,
    #[default]
    Unknown,
}

/// Broad classification for image container types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageKind {
    Raster,
    Vector,
    #[default]
    Unknown,
}

/// Small image metadata bundle for callers that already know or discovered a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageMetadata {
    pub format: ImageFormat,
    pub kind: ImageKind,
    pub mime_type: Option<&'static str>,
    pub extension: Option<&'static str>,
    pub size: Option<ImageSize>,
}

impl ImageMetadata {
    /// Builds metadata from a detected format without embedded size information.
    #[must_use]
    pub fn new(format: ImageFormat) -> Self {
        Self::with_size(format, None)
    }

    /// Builds metadata from a detected format and optional size information.
    #[must_use]
    pub fn with_size(format: ImageFormat, size: Option<ImageSize>) -> Self {
        Self {
            format,
            kind: image_kind(format),
            mime_type: image_mime_type(format),
            extension: image_extension(format),
            size,
        }
    }
}

/// Returns the broad image kind for a format.
#[must_use]
pub const fn image_kind(format: ImageFormat) -> ImageKind {
    match format {
        ImageFormat::Svg => ImageKind::Vector,
        ImageFormat::Png
        | ImageFormat::Jpeg
        | ImageFormat::Webp
        | ImageFormat::Gif
        | ImageFormat::Ico
        | ImageFormat::Bmp
        | ImageFormat::Tiff
        | ImageFormat::Avif => ImageKind::Raster,
        ImageFormat::Unknown => ImageKind::Unknown,
    }
}

/// Returns true when the format is raster-based.
#[must_use]
pub const fn is_raster_image(format: ImageFormat) -> bool {
    matches!(image_kind(format), ImageKind::Raster)
}

/// Returns true when the format is vector-based.
#[must_use]
pub const fn is_vector_image(format: ImageFormat) -> bool {
    matches!(image_kind(format), ImageKind::Vector)
}

/// Returns true for formats commonly used in browsers and web asset pipelines.
#[must_use]
pub const fn is_web_image_format(format: ImageFormat) -> bool {
    matches!(
        format,
        ImageFormat::Png
            | ImageFormat::Jpeg
            | ImageFormat::Webp
            | ImageFormat::Gif
            | ImageFormat::Svg
            | ImageFormat::Ico
            | ImageFormat::Avif
    )
}

/// Returns true when the format can commonly carry transparency.
#[must_use]
pub const fn supports_transparency(format: ImageFormat) -> bool {
    matches!(
        format,
        ImageFormat::Png
            | ImageFormat::Webp
            | ImageFormat::Gif
            | ImageFormat::Svg
            | ImageFormat::Ico
            | ImageFormat::Tiff
            | ImageFormat::Avif
    )
}

/// Returns true when the format commonly supports animation.
#[must_use]
pub const fn supports_animation(format: ImageFormat) -> bool {
    matches!(
        format,
        ImageFormat::Gif | ImageFormat::Webp | ImageFormat::Svg | ImageFormat::Avif
    )
}
