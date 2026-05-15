use crate::format::ImageFormat;

const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
const JPEG_SIGNATURE: &[u8] = &[0xFF, 0xD8, 0xFF];

fn is_webp(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP"
}

fn is_tiff(bytes: &[u8]) -> bool {
    matches!(
        bytes,
        [0x49, 0x49, 0x2A, 0x00, ..]
            | [0x4D, 0x4D, 0x00, 0x2A, ..]
            | [0x49, 0x49, 0x2B, 0x00, ..]
            | [0x4D, 0x4D, 0x00, 0x2B, ..]
    )
}

fn is_avif(bytes: &[u8]) -> bool {
    if bytes.len() < 16 || bytes.get(4..8) != Some(b"ftyp") {
        return false;
    }

    let search_end = bytes.len().min(32);

    bytes[8..search_end]
        .chunks(4)
        .any(|chunk| chunk == b"avif" || chunk == b"avis")
}

pub(crate) fn looks_like_svg(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return false;
    }

    let sniff_len = bytes.len().min(512);
    let text = String::from_utf8_lossy(&bytes[..sniff_len]);
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let lowered = text.to_ascii_lowercase();
    let trimmed = lowered.trim_start();

    trimmed.starts_with("<svg") || (trimmed.starts_with("<?xml") && trimmed.contains("<svg"))
}

/// Detects an image format from common magic bytes without fully parsing the file.
#[must_use]
pub fn detect_image_format_from_bytes(bytes: &[u8]) -> ImageFormat {
    if bytes.starts_with(PNG_SIGNATURE) {
        return ImageFormat::Png;
    }

    if bytes.starts_with(JPEG_SIGNATURE) {
        return ImageFormat::Jpeg;
    }

    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return ImageFormat::Gif;
    }

    if is_webp(bytes) {
        return ImageFormat::Webp;
    }

    if bytes.starts_with(&[0x00, 0x00, 0x01, 0x00]) {
        return ImageFormat::Ico;
    }

    if bytes.starts_with(b"BM") {
        return ImageFormat::Bmp;
    }

    if is_tiff(bytes) {
        return ImageFormat::Tiff;
    }

    if is_avif(bytes) {
        return ImageFormat::Avif;
    }

    if looks_like_svg(bytes) {
        return ImageFormat::Svg;
    }

    ImageFormat::Unknown
}
