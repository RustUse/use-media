#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod attribute;
pub mod color;
pub mod document;
pub mod element;
pub mod normalize;
pub mod path;
pub mod size;
pub mod transform;
pub mod view_box;

pub use attribute::{SvgAttribute, extract_attributes, get_attribute, has_attribute};
pub use color::{extract_fill_values, extract_stroke_values};
pub use document::{
    SvgDocument, SvgMetadata, build_svg_document, build_svg_icon, extract_description,
    extract_height, extract_metadata, extract_svg_root, extract_title, extract_view_box,
    extract_width, has_svg_root, is_svg, strip_xml_declaration,
};
pub use element::{SvgElement, parse_element};
pub use normalize::{minify_svg_basic, normalize_svg, strip_comments};
pub use path::{SvgPath, extract_path_data, extract_paths};
pub use size::{SvgSize, extract_size};
pub use transform::{SvgTransform, extract_transform_values, normalize_transform};
pub use view_box::{SvgViewBox, format_view_box, parse_view_box};
