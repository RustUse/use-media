# use-svg

Practical SVG utility primitives for inspection, extraction, normalization, minification, and lightweight document construction.

Warning: `use-svg` is experimental and may change before 0.3.0.

## Example

```rust
use use_svg::{SvgPath, build_svg_icon, extract_path_data, extract_view_box};

let source = r#"<svg viewBox="0 0 24 24"><path d="M3 3h18v18H3z"/></svg>"#;
let view_box = extract_view_box(source).unwrap();
let paths = extract_path_data(source);

assert_eq!(paths, vec!["M3 3h18v18H3z".to_string()]);

let icon = build_svg_icon(view_box, &[SvgPath::new("M3 3h18v18H3z")]);
assert!(icon.starts_with("<svg "));
```

## Scope

- SVG document detection and root extraction
- Width, height, and SVG size extraction
- viewBox parsing and formatting
- SVG element and attribute helpers
- Path data extraction helpers
- Fill, stroke, and transform attribute helpers
- Metadata extraction for common title and description tags
- Basic normalization and minification
- Simple SVG document and icon construction

## Non-goals

- A full SVG rendering engine
- A full XML parser or DOM implementation
- A complete SVG specification parser
- CSS layout or browser compatibility behavior
- Heavy dependency-driven parsing pipelines

## License

Licensed under either the MIT license or the Apache License, Version 2.0, at your option.
