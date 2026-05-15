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
pub use use_media_duration;
pub use use_media_duration::*;
pub use use_media_timestamp;
pub use use_media_timestamp::*;
pub use use_media_type;
pub use use_media_type::*;
pub use use_resolution;
pub use use_resolution::*;

#[cfg(test)]
mod tests {
    use super::{
        AspectRatio, Dimensions, FrameRate, MediaKind, ResolutionClass, classify_resolution,
        parse_media_type,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let dimensions = Dimensions::new(1920, 1080).unwrap();
        let aspect = AspectRatio::new(1920, 1080).unwrap();
        let rate = FrameRate::new(24.0).unwrap();
        let mime = parse_media_type("video/mp4").unwrap();

        assert_eq!(
            classify_resolution(1920, 1080).unwrap(),
            ResolutionClass::FullHd
        );
        assert_eq!(aspect.label(), "16:9");
        assert_eq!(mime.kind(), MediaKind::Video);
        assert_eq!(dimensions.area(), 2_073_600);
        assert!((rate.frame_duration_seconds() - (1.0 / 24.0)).abs() < 1.0e-12);
    }
}
