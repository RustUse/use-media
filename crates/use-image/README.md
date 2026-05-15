# RustUse use-image

Lightweight image utility primitives for format detection, image metadata classification, and common sizing helpers.

Warning: `use-image` is experimental and may change before 0.3.0.

## Example

```rust
use use_image::{
    ImageFormat, ImageSize, detect_image_format_from_bytes, fit_within, image_mime_type,
    orientation,
};

let format = detect_image_format_from_bytes(b"\x89PNG\r\n\x1a\nrest");
let size = ImageSize::new(1600, 900);

assert_eq!(format, ImageFormat::Png);
assert_eq!(image_mime_type(format), Some("image/png"));
assert_eq!(orientation(size).to_string(), "landscape");
assert_eq!(fit_within(size, ImageSize::new(400, 400)), ImageSize::new(400, 225));
```

## Scope

- Image format detection from extensions, MIME types, and common magic bytes
- Shared raster and vector classification helpers
- Canonical image MIME and extension lookup
- Basic image size, aspect ratio, orientation, and fit helpers
- Small metadata structs for tooling, asset pipelines, and CLIs
- Shared primitives for future crates such as `use-png`, `use-jpeg`, `use-webp`, `use-gif`, and `use-ico`

## Non-goals

- A full image decoder or encoder
- Pixel manipulation or rendering
- EXIF, ICC profile, or animation frame parsing
- Compression or decompression
- Filesystem I/O beyond what callers choose to do themselves

## Relationship To use-svg

`use-image` handles shared image primitives and lightweight detection. `use-svg` stays focused on SVG-specific document helpers such as root extraction, viewBox parsing, normalization, and path utilities. Future format-specific crates can build on `use-image` for shared classification and metadata behavior while providing deeper format parsing where that remains practical.

## License

Licensed under either the MIT license or the Apache License, Version 2.0, at your option.
