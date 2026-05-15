#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod aspect_ratio;
pub mod extension;
pub mod format;
pub mod magic;
pub mod mime;
pub mod orientation;
pub mod size;

pub use aspect_ratio::{ImageAspectRatio, aspect_ratio, aspect_ratio_f64};
pub use extension::{
    detect_image_format_from_extension, extension_from_filename, image_extension,
    is_image_extension, normalize_extension,
};
pub use format::{
    ImageFormat, ImageKind, ImageMetadata, image_kind, is_raster_image, is_vector_image,
    is_web_image_format, supports_animation, supports_transparency,
};
pub use magic::detect_image_format_from_bytes;
pub use mime::{detect_image_format_from_mime, image_mime_type, is_image_mime};
pub use orientation::{ImageOrientation, orientation};
pub use size::{ImageSize, cover_size, fit_within, scale_to_height, scale_to_width};
