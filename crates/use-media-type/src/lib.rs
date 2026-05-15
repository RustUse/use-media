#![forbid(unsafe_code)]
//! Primitive media type helpers.
//!
//! These helpers keep MIME-like parsing intentionally small and deterministic.
//!
//! # Examples
//!
//! ```rust
//! use use_media_type::{MediaKind, is_video_mime, parse_media_type};
//!
//! let media_type = parse_media_type("video/mp4").unwrap();
//!
//! assert_eq!(media_type.kind(), MediaKind::Video);
//! assert_eq!(media_type.subtype(), "mp4");
//! assert_eq!(media_type.as_mime(), "video/mp4");
//! assert!(is_video_mime("video/mp4"));
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Image,
    Audio,
    Video,
    Text,
    Application,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaType {
    kind: MediaKind,
    subtype: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaTypeError {
    InvalidSubtype,
    InvalidMime,
}

fn kind_label(kind: MediaKind) -> &'static str {
    match kind {
        MediaKind::Image => "image",
        MediaKind::Audio => "audio",
        MediaKind::Video => "video",
        MediaKind::Text => "text",
        MediaKind::Application => "application",
        MediaKind::Unknown => "unknown",
    }
}

impl MediaType {
    pub fn new(kind: MediaKind, subtype: impl Into<String>) -> Result<Self, MediaTypeError> {
        let subtype = subtype.into().trim().to_ascii_lowercase();

        if subtype.is_empty() || subtype.contains('/') {
            return Err(MediaTypeError::InvalidSubtype);
        }

        Ok(Self { kind, subtype })
    }

    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.kind
    }

    #[must_use]
    pub fn subtype(&self) -> &str {
        &self.subtype
    }

    #[must_use]
    pub fn as_mime(&self) -> String {
        format!("{}/{}", kind_label(self.kind), self.subtype)
    }
}

#[must_use]
pub fn media_kind_from_mime(mime: &str) -> MediaKind {
    let base = mime.trim().split(';').next().unwrap_or("").trim();
    let kind = base
        .split('/')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();

    match kind.as_str() {
        "image" => MediaKind::Image,
        "audio" => MediaKind::Audio,
        "video" => MediaKind::Video,
        "text" => MediaKind::Text,
        "application" => MediaKind::Application,
        _ => MediaKind::Unknown,
    }
}

pub fn parse_media_type(mime: &str) -> Result<MediaType, MediaTypeError> {
    let base = mime.trim().split(';').next().unwrap_or("").trim();
    let mut parts = base.splitn(2, '/');
    let kind_part = parts.next().unwrap_or("").trim();
    let subtype = parts.next().unwrap_or("").trim();

    if kind_part.is_empty() || subtype.is_empty() {
        return Err(MediaTypeError::InvalidMime);
    }

    MediaType::new(media_kind_from_mime(base), subtype)
}

#[must_use]
pub fn is_image_mime(mime: &str) -> bool {
    media_kind_from_mime(mime) == MediaKind::Image
}

#[must_use]
pub fn is_audio_mime(mime: &str) -> bool {
    media_kind_from_mime(mime) == MediaKind::Audio
}

#[must_use]
pub fn is_video_mime(mime: &str) -> bool {
    media_kind_from_mime(mime) == MediaKind::Video
}

#[cfg(test)]
mod tests {
    use super::{
        MediaKind, MediaType, MediaTypeError, is_audio_mime, is_image_mime, is_video_mime,
        media_kind_from_mime, parse_media_type,
    };

    #[test]
    fn parses_simple_media_types() {
        let image = parse_media_type("image/png").unwrap();
        let audio = parse_media_type("audio/mpeg").unwrap();
        let video = parse_media_type("video/mp4; codecs=avc1").unwrap();

        assert_eq!(image.kind(), MediaKind::Image);
        assert_eq!(audio.kind(), MediaKind::Audio);
        assert_eq!(video.kind(), MediaKind::Video);
        assert_eq!(video.subtype(), "mp4");
        assert_eq!(video.as_mime(), "video/mp4");
        assert_eq!(
            MediaType::new(MediaKind::Text, "plain").unwrap().as_mime(),
            "text/plain"
        );
    }

    #[test]
    fn checks_media_kind_predicates() {
        assert_eq!(media_kind_from_mime("image/jpeg"), MediaKind::Image);
        assert_eq!(
            media_kind_from_mime("application/json"),
            MediaKind::Application
        );
        assert_eq!(media_kind_from_mime("font/woff2"), MediaKind::Unknown);
        assert!(is_image_mime("image/png"));
        assert!(is_audio_mime("audio/ogg"));
        assert!(is_video_mime("video/mp4"));
        assert!(!is_video_mime("image/png"));
    }

    #[test]
    fn rejects_invalid_media_type_inputs() {
        assert_eq!(parse_media_type("image"), Err(MediaTypeError::InvalidMime));
        assert_eq!(parse_media_type("/png"), Err(MediaTypeError::InvalidMime));
        assert_eq!(
            MediaType::new(MediaKind::Image, ""),
            Err(MediaTypeError::InvalidSubtype)
        );
        assert_eq!(
            MediaType::new(MediaKind::Image, "image/png"),
            Err(MediaTypeError::InvalidSubtype)
        );
    }
}
