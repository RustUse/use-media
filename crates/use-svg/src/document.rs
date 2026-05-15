use crate::attribute::{find_opening_tag, find_tag_end, get_attribute, tag_name_matches};
use crate::normalize::strip_comments;
use crate::path::SvgPath;
use crate::view_box::{SvgViewBox, format_view_box, parse_view_box};

const SVG_XMLNS: &str = "http://www.w3.org/2000/svg";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SvgDocument {
    pub source: String,
}

impl SvgDocument {
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.source
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SvgMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[must_use]
pub fn is_svg(input: &str) -> bool {
    has_svg_root(input)
}

#[must_use]
pub fn has_svg_root(input: &str) -> bool {
    extract_svg_root(input).is_some()
}

pub fn extract_svg_root(input: &str) -> Option<&str> {
    let remaining = skip_leading_comments(strip_xml_declaration(input))?;

    if !tag_name_matches(remaining, 0, "svg") {
        return None;
    }

    let end = find_tag_end(remaining, 0)?;
    Some(&remaining[..end])
}

pub fn strip_xml_declaration(input: &str) -> &str {
    let trimmed = input.strip_prefix('\u{feff}').unwrap_or(input).trim_start();

    if !starts_with_ascii_case_insensitive(trimmed, "<?xml") {
        return trimmed;
    }

    match find_ascii_case_insensitive(trimmed, "?>") {
        Some(end) => trimmed[end + 2..].trim_start(),
        None => trimmed,
    }
}

#[must_use]
pub fn extract_width(input: &str) -> Option<String> {
    extract_svg_root(input).and_then(|root| get_attribute(root, "width"))
}

#[must_use]
pub fn extract_height(input: &str) -> Option<String> {
    extract_svg_root(input).and_then(|root| get_attribute(root, "height"))
}

#[must_use]
pub fn extract_view_box(input: &str) -> Option<SvgViewBox> {
    extract_svg_root(input)
        .and_then(|root| get_attribute(root, "viewBox"))
        .and_then(|value| parse_view_box(&value))
}

#[must_use]
pub fn extract_title(input: &str) -> Option<String> {
    extract_text_element(input, "title")
}

#[must_use]
pub fn extract_description(input: &str) -> Option<String> {
    extract_text_element(input, "desc")
}

#[must_use]
pub fn extract_metadata(input: &str) -> SvgMetadata {
    SvgMetadata {
        title: extract_title(input),
        description: extract_description(input),
    }
}

#[must_use]
pub fn build_svg_document(view_box: SvgViewBox, body: &str) -> String {
    let body = body.trim();

    format!(
        r#"<svg xmlns="{SVG_XMLNS}" viewBox="{}">{body}</svg>"#,
        format_view_box(view_box)
    )
}

#[must_use]
pub fn build_svg_icon(view_box: SvgViewBox, paths: &[SvgPath]) -> String {
    let body = paths
        .iter()
        .map(|path| format!(r#"<path d="{}"/>"#, escape_attribute_value(&path.data)))
        .collect::<String>();

    build_svg_document(view_box, &body)
}

fn extract_text_element(input: &str, tag_name: &str) -> Option<String> {
    let cleaned = strip_comments(strip_xml_declaration(input));
    let (_, end) = find_opening_tag(cleaned.as_str(), tag_name, 0)?;
    let remainder = &cleaned[end..];
    let close_tag = format!("</{tag_name}>");
    let close_start = find_ascii_case_insensitive(remainder, &close_tag)?;
    let value = remainder[..close_start].trim();

    (!value.is_empty()).then(|| value.to_string())
}

fn skip_leading_comments(mut input: &str) -> Option<&str> {
    loop {
        input = input.trim_start();

        if !input.starts_with("<!--") {
            return Some(input);
        }

        let end = input.find("-->")?;
        input = &input[end + 3..];
    }
}

fn starts_with_ascii_case_insensitive(input: &str, prefix: &str) -> bool {
    input
        .get(..prefix.len())
        .is_some_and(|candidate| candidate.eq_ignore_ascii_case(prefix))
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    haystack.char_indices().find_map(|(index, _)| {
        let candidate = haystack.get(index..index + needle.len())?;
        candidate.eq_ignore_ascii_case(needle).then_some(index)
    })
}

fn escape_attribute_value(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}
