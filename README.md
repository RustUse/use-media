# use-media

Composable primitive media metadata utilities for Rust.

`use-media` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-signal`, `use-wave`, `use-acoustics`,
`use-color`, `use-text`, `use-time`, `use-units`, `use-accessibility`,
and `use-typography`. It groups small, focused crates for dimensions,
aspect ratios, resolutions, frame rates, bitrates, durations, timestamps,
and lightweight media type helpers.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `u32`, `u64`, `f64`, `usize`, `String`, `&str`, and small enums or structs
- dependencies stay minimal so each crate is easy to audit and adopt

These crates provide media metadata and calculation helpers, not full
audio, video, or image processing. They do not implement codecs,
transcoding, decoding, encoding, or file parsing in the first pass.

## Workspace crates

- `use-media`: thin facade crate that reexports the full media workspace
- `use-aspect-ratio`: aspect-ratio simplification and tolerance helpers
- `use-dimensions`: width, height, and pixel-area helpers
- `use-resolution`: resolution class and scaling helpers
- `use-frame-rate`: frame duration and frame-count helpers
- `use-bitrate`: bitrate and size-for-duration helpers
- `use-media-duration`: duration formatting and sample-count helpers
- `use-media-timestamp`: timestamp, frame-index, and clamp helpers
- `use-media-type`: lightweight media kind and MIME-style helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-media`.
It reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_media::*;

let dimensions = Dimensions::new(1920, 1080).unwrap();
let aspect = AspectRatio::new(1920, 1080).unwrap();
let mime = parse_media_type("video/mp4").unwrap();

assert_eq!(classify_resolution(1920, 1080).unwrap(), ResolutionClass::FullHd);
assert_eq!(aspect.label(), "16:9");
assert_eq!(mime.kind(), MediaKind::Video);
assert_eq!(dimensions.area(), 2_073_600);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
media surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
