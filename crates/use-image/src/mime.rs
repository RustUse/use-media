use crate::format::ImageFormat;

pub(crate) fn normalize_mime_value(mime: &str) -> Option<String> {
    let value = mime.split(';').next()?.trim();

    if value.is_empty() {
        return None;
    }

    Some(value.to_ascii_lowercase())
}

/// Detects an image format from a MIME type string.
#[must_use]
pub fn detect_image_format_from_mime(mime: &str) -> ImageFormat {
    let Some(normalized) = normalize_mime_value(mime) else {
        return ImageFormat::Unknown;
    };

    match normalized.as_str() {
        "image/png" => ImageFormat::Png,
        "image/jpeg" | "image/jpg" | "image/pjpeg" => ImageFormat::Jpeg,
        "image/webp" => ImageFormat::Webp,
        "image/gif" => ImageFormat::Gif,
        "image/svg+xml" | "image/svg" => ImageFormat::Svg,
        "image/x-icon" | "image/vnd.microsoft.icon" => ImageFormat::Ico,
        "image/bmp" | "image/x-ms-bmp" => ImageFormat::Bmp,
        "image/tiff" | "image/tif" => ImageFormat::Tiff,
        "image/avif" => ImageFormat::Avif,
        _ => ImageFormat::Unknown,
    }
}

/// Returns the canonical MIME type for a format.
#[must_use]
pub const fn image_mime_type(format: ImageFormat) -> Option<&'static str> {
    match format {
        ImageFormat::Png => Some("image/png"),
        ImageFormat::Jpeg => Some("image/jpeg"),
        ImageFormat::Webp => Some("image/webp"),
        ImageFormat::Gif => Some("image/gif"),
        ImageFormat::Svg => Some("image/svg+xml"),
        ImageFormat::Ico => Some("image/x-icon"),
        ImageFormat::Bmp => Some("image/bmp"),
        ImageFormat::Tiff => Some("image/tiff"),
        ImageFormat::Avif => Some("image/avif"),
        ImageFormat::Unknown => None,
    }
}

/// Returns true when the input is a known image MIME type.
#[must_use]
pub fn is_image_mime(mime: &str) -> bool {
    detect_image_format_from_mime(mime) != ImageFormat::Unknown
}
