#![forbid(unsafe_code)]
//! Thin facade for the `use-media` workspace.
//!
//! The crate reexports the focused media crates directly so consumers can opt
//! into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_media::*;
//!
//! let dimensions = Dimensions::new(1920, 1080).unwrap();
//! let aspect = AspectRatio::new(1920, 1080).unwrap();
//! let mime = parse_media_type("video/mp4").unwrap();
//!
//! assert_eq!(classify_resolution(1920, 1080).unwrap(), ResolutionClass::FullHd);
//! assert_eq!(aspect.label(), "16:9");
//! assert_eq!(mime.kind(), MediaKind::Video);
//! assert_eq!(dimensions.area(), 2_073_600);
//! ```

pub use use_aspect_ratio;
pub use use_aspect_ratio::*;
pub use use_bitrate;
pub use use_bitrate::*;
pub use use_dimensions;
pub use use_dimensions::*;
pub use use_frame_rate;
pub use use_frame_rate::*;
pub use use_image;
pub use use_image::aspect_ratio as image_aspect_ratio;
pub use use_image::fit_within as fit_image_within;
pub use use_image::is_image_mime as is_known_image_mime;
pub use use_image::{
    ImageAspectRatio, ImageFormat, ImageKind, ImageMetadata, ImageOrientation, ImageSize,
    aspect_ratio_f64, cover_size, detect_image_format_from_bytes,
    detect_image_format_from_extension, detect_image_format_from_mime, extension_from_filename,
    image_extension, image_kind, image_mime_type, is_image_extension, is_raster_image,
    is_vector_image, is_web_image_format, normalize_extension, orientation, scale_to_height,
    scale_to_width, supports_animation, supports_transparency,
};
pub use use_media_duration;
pub use use_media_duration::*;
pub use use_media_timestamp;
pub use use_media_timestamp::*;
pub use use_media_type;
pub use use_media_type::*;
pub use use_resolution;
pub use use_resolution::*;
pub use use_svg;
pub use use_svg::*;

#[cfg(test)]
mod tests {
    use super::{
        AspectRatio, Dimensions, FrameRate, ImageFormat, ImageSize, MediaKind, ResolutionClass,
        SvgPath, build_svg_icon, classify_resolution, detect_image_format_from_extension,
        extract_path_data, fit_image_within, parse_media_type, parse_view_box,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let dimensions = Dimensions::new(1920, 1080).unwrap();
        let aspect = AspectRatio::new(1920, 1080).unwrap();
        let rate = FrameRate::new(24.0).unwrap();
        let mime = parse_media_type("video/mp4").unwrap();
        let icon = build_svg_icon(
            parse_view_box("0 0 24 24").unwrap(),
            &[SvgPath::new("M0 0h24v24H0z")],
        );

        assert_eq!(
            classify_resolution(1920, 1080).unwrap(),
            ResolutionClass::FullHd
        );
        assert_eq!(aspect.label(), "16:9");
        assert_eq!(mime.kind(), MediaKind::Video);
        assert_eq!(dimensions.area(), 2_073_600);
        assert_eq!(detect_image_format_from_extension(".png"), ImageFormat::Png);
        assert_eq!(
            fit_image_within(ImageSize::new(800, 600), ImageSize::new(300, 300)),
            ImageSize::new(300, 225)
        );
        assert!((rate.frame_duration_seconds() - (1.0 / 24.0)).abs() < 1.0e-12);
        assert_eq!(extract_path_data(&icon), vec!["M0 0h24v24H0z".to_string()]);
    }
}
