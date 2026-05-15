use crate::format::ImageFormat;

/// Detects an image format from a file extension.
#[must_use]
pub fn detect_image_format_from_extension(extension: &str) -> ImageFormat {
    match normalize_extension(extension) {
        Some("png") => ImageFormat::Png,
        Some("jpg") => ImageFormat::Jpeg,
        Some("webp") => ImageFormat::Webp,
        Some("gif") => ImageFormat::Gif,
        Some("svg") => ImageFormat::Svg,
        Some("ico") => ImageFormat::Ico,
        Some("bmp") => ImageFormat::Bmp,
        Some("tiff") => ImageFormat::Tiff,
        Some("avif") => ImageFormat::Avif,
        _ => ImageFormat::Unknown,
    }
}

/// Returns the canonical extension without a leading dot.
#[must_use]
pub const fn image_extension(format: ImageFormat) -> Option<&'static str> {
    match format {
        ImageFormat::Png => Some("png"),
        ImageFormat::Jpeg => Some("jpg"),
        ImageFormat::Webp => Some("webp"),
        ImageFormat::Gif => Some("gif"),
        ImageFormat::Svg => Some("svg"),
        ImageFormat::Ico => Some("ico"),
        ImageFormat::Bmp => Some("bmp"),
        ImageFormat::Tiff => Some("tiff"),
        ImageFormat::Avif => Some("avif"),
        ImageFormat::Unknown => None,
    }
}

/// Normalizes an image extension to a canonical lowercase value without a dot.
#[must_use]
pub fn normalize_extension(extension: &str) -> Option<&'static str> {
    let trimmed = extension.trim();
    let stripped = trimmed.trim_start_matches('.');

    if stripped.is_empty() {
        return None;
    }

    match stripped.to_ascii_lowercase().as_str() {
        "png" => Some("png"),
        "jpg" | "jpeg" => Some("jpg"),
        "webp" => Some("webp"),
        "gif" => Some("gif"),
        "svg" => Some("svg"),
        "ico" => Some("ico"),
        "bmp" => Some("bmp"),
        "tif" | "tiff" => Some("tiff"),
        "avif" => Some("avif"),
        _ => None,
    }
}

/// Returns true when the input is a known image extension.
#[must_use]
pub fn is_image_extension(extension: &str) -> bool {
    normalize_extension(extension).is_some()
}

/// Extracts a filename extension without changing its original case.
#[must_use]
pub fn extension_from_filename(filename: &str) -> Option<&str> {
    let last_segment = filename.rsplit(['/', '\\']).next()?;
    let candidate = last_segment
        .split(['?', '#'])
        .next()
        .unwrap_or(last_segment);
    let (stem, extension) = candidate.rsplit_once('.')?;

    if stem.is_empty() || extension.is_empty() {
        return None;
    }

    Some(extension)
}
