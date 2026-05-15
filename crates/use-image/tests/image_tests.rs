use use_image::{
    ImageAspectRatio, ImageFormat, ImageKind, ImageMetadata, ImageOrientation, ImageSize,
    aspect_ratio, aspect_ratio_f64, cover_size, detect_image_format_from_bytes,
    detect_image_format_from_extension, detect_image_format_from_mime, extension_from_filename,
    fit_within, image_extension, image_kind, image_mime_type, is_image_extension, is_image_mime,
    is_raster_image, is_vector_image, is_web_image_format, normalize_extension, orientation,
    scale_to_height, scale_to_width, supports_animation, supports_transparency,
};

#[test]
fn detects_formats_from_extensions() {
    assert_eq!(detect_image_format_from_extension("png"), ImageFormat::Png);
    assert_eq!(
        detect_image_format_from_extension(".JPG"),
        ImageFormat::Jpeg
    );
    assert_eq!(
        detect_image_format_from_extension(" TIFF "),
        ImageFormat::Tiff
    );
    assert_eq!(
        detect_image_format_from_extension("avif"),
        ImageFormat::Avif
    );
    assert_eq!(
        detect_image_format_from_extension("txt"),
        ImageFormat::Unknown
    );
}

#[test]
fn detects_formats_from_mime_types() {
    assert_eq!(detect_image_format_from_mime("image/png"), ImageFormat::Png);
    assert_eq!(
        detect_image_format_from_mime("IMAGE/SVG+XML; charset=utf-8"),
        ImageFormat::Svg
    );
    assert_eq!(
        detect_image_format_from_mime("image/vnd.microsoft.icon"),
        ImageFormat::Ico
    );
    assert_eq!(
        detect_image_format_from_mime("application/octet-stream"),
        ImageFormat::Unknown
    );
}

#[test]
fn detects_png_magic_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(b"\x89PNG\r\n\x1a\nrest"),
        ImageFormat::Png
    );
}

#[test]
fn detects_jpeg_magic_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(&[0xFF, 0xD8, 0xFF, 0xE0]),
        ImageFormat::Jpeg
    );
}

#[test]
fn detects_webp_magic_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(b"RIFF\x24\0\0\0WEBPVP8 "),
        ImageFormat::Webp
    );
}

#[test]
fn detects_gif_magic_bytes() {
    assert_eq!(detect_image_format_from_bytes(b"GIF89a"), ImageFormat::Gif);
}

#[test]
fn detects_ico_magic_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(&[0x00, 0x00, 0x01, 0x00, 0x01, 0x00]),
        ImageFormat::Ico
    );
}

#[test]
fn detects_bmp_magic_bytes() {
    assert_eq!(detect_image_format_from_bytes(b"BMrest"), ImageFormat::Bmp);
}

#[test]
fn detects_tiff_magic_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(&[0x49, 0x49, 0x2A, 0x00, 0x08, 0x00]),
        ImageFormat::Tiff
    );
}

#[test]
fn detects_svg_from_string_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(b"  <?xml version=\"1.0\"?><svg viewBox=\"0 0 1 1\"></svg>"),
        ImageFormat::Svg
    );
}

#[test]
fn returns_unknown_for_unknown_bytes() {
    assert_eq!(
        detect_image_format_from_bytes(b"not an image"),
        ImageFormat::Unknown
    );
}

#[test]
fn looks_up_image_mime_types() {
    assert_eq!(image_mime_type(ImageFormat::Png), Some("image/png"));
    assert_eq!(image_mime_type(ImageFormat::Unknown), None);
}

#[test]
fn looks_up_image_extensions() {
    assert_eq!(image_extension(ImageFormat::Jpeg), Some("jpg"));
    assert_eq!(image_extension(ImageFormat::Unknown), None);
}

#[test]
fn classifies_raster_images() {
    assert_eq!(image_kind(ImageFormat::Png), ImageKind::Raster);
    assert!(is_raster_image(ImageFormat::Avif));
    assert!(!is_raster_image(ImageFormat::Svg));
}

#[test]
fn classifies_vector_images() {
    assert_eq!(image_kind(ImageFormat::Svg), ImageKind::Vector);
    assert!(is_vector_image(ImageFormat::Svg));
    assert!(!is_vector_image(ImageFormat::Png));
}

#[test]
fn classifies_web_image_formats() {
    assert!(is_web_image_format(ImageFormat::Avif));
    assert!(is_web_image_format(ImageFormat::Svg));
    assert!(!is_web_image_format(ImageFormat::Bmp));
}

#[test]
fn reports_transparency_support() {
    assert!(supports_transparency(ImageFormat::Png));
    assert!(supports_transparency(ImageFormat::Svg));
    assert!(!supports_transparency(ImageFormat::Jpeg));
}

#[test]
fn reports_animation_support() {
    assert!(supports_animation(ImageFormat::Gif));
    assert!(supports_animation(ImageFormat::Webp));
    assert!(!supports_animation(ImageFormat::Png));
}

#[test]
fn computes_aspect_ratios() {
    assert_eq!(
        aspect_ratio(ImageSize::new(1920, 1080)),
        ImageAspectRatio::new(16, 9)
    );
}

#[test]
fn computes_f64_aspect_ratios() {
    let ratio = aspect_ratio_f64(ImageSize::new(1920, 1080)).unwrap();

    assert!((ratio - (16.0 / 9.0)).abs() < 1.0e-12);
}

#[test]
fn detects_landscape_orientation() {
    assert_eq!(
        orientation(ImageSize::new(1920, 1080)),
        ImageOrientation::Landscape
    );
}

#[test]
fn detects_portrait_orientation() {
    assert_eq!(
        orientation(ImageSize::new(1080, 1920)),
        ImageOrientation::Portrait
    );
}

#[test]
fn detects_square_orientation() {
    assert_eq!(
        orientation(ImageSize::new(512, 512)),
        ImageOrientation::Square
    );
}

#[test]
fn fits_sizes_within_bounds() {
    assert_eq!(
        fit_within(ImageSize::new(800, 600), ImageSize::new(300, 300)),
        ImageSize::new(300, 225)
    );
}

#[test]
fn computes_cover_sizes() {
    assert_eq!(
        cover_size(ImageSize::new(800, 600), ImageSize::new(300, 300)),
        ImageSize::new(400, 300)
    );
}

#[test]
fn scales_to_width() {
    assert_eq!(
        scale_to_width(ImageSize::new(800, 600), 320),
        Some(ImageSize::new(320, 240))
    );
}

#[test]
fn scales_to_height() {
    assert_eq!(
        scale_to_height(ImageSize::new(800, 600), 240),
        Some(ImageSize::new(320, 240))
    );
}

#[test]
fn normalizes_extensions() {
    assert_eq!(normalize_extension(".JPEG"), Some("jpg"));
    assert_eq!(normalize_extension(" tif "), Some("tiff"));
    assert_eq!(normalize_extension("txt"), None);
    assert!(is_image_extension(".webp"));
}

#[test]
fn extracts_filename_extensions() {
    assert_eq!(
        extension_from_filename("assets/icons/logo.SVG"),
        Some("SVG")
    );
    assert_eq!(extension_from_filename("photo"), None);
    assert_eq!(extension_from_filename(".gitignore"), None);
}

#[test]
fn handles_malformed_input_gracefully() {
    assert_eq!(
        detect_image_format_from_mime("image/"),
        ImageFormat::Unknown
    );
    assert_eq!(
        detect_image_format_from_bytes(b"RIFF"),
        ImageFormat::Unknown
    );
    assert_eq!(
        fit_within(ImageSize::new(0, 600), ImageSize::new(300, 300)),
        ImageSize::new(0, 0)
    );
    assert_eq!(
        cover_size(ImageSize::new(800, 0), ImageSize::new(300, 300)),
        ImageSize::new(0, 0)
    );
    assert_eq!(scale_to_width(ImageSize::new(0, 100), 50), None);
    assert_eq!(aspect_ratio_f64(ImageSize::new(16, 0)), None);
    assert_eq!(
        orientation(ImageSize::new(0, 100)),
        ImageOrientation::Unknown
    );
}

#[test]
fn handles_empty_input() {
    assert_eq!(detect_image_format_from_bytes(&[]), ImageFormat::Unknown);
    assert_eq!(detect_image_format_from_extension(""), ImageFormat::Unknown);
    assert_eq!(detect_image_format_from_mime(""), ImageFormat::Unknown);
    assert_eq!(extension_from_filename(""), None);
    assert!(!is_image_extension(""));
    assert!(!is_image_mime(""));
}

#[test]
fn builds_metadata_from_format() {
    let metadata = ImageMetadata::with_size(ImageFormat::Png, Some(ImageSize::new(64, 64)));

    assert_eq!(metadata.format, ImageFormat::Png);
    assert_eq!(metadata.kind, ImageKind::Raster);
    assert_eq!(metadata.mime_type, Some("image/png"));
    assert_eq!(metadata.extension, Some("png"));
    assert_eq!(metadata.size, Some(ImageSize::new(64, 64)));
}
