use crate::attribute::{find_opening_tag, get_attribute};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgPath {
    pub data: String,
}

impl SvgPath {
    #[must_use]
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }
}

#[must_use]
pub fn extract_paths(input: &str) -> Vec<SvgPath> {
    extract_path_data(input)
        .into_iter()
        .map(SvgPath::new)
        .collect()
}

#[must_use]
pub fn extract_path_data(input: &str) -> Vec<String> {
    let cleaned = crate::normalize::strip_comments(input);
    let mut values = Vec::new();
    let mut index = 0;

    while let Some((start, end)) = find_opening_tag(cleaned.as_str(), "path", index) {
        let tag = &cleaned[start..end];
        if let Some(value) = get_attribute(tag, "d") {
            values.push(value);
        }
        index = end;
    }

    values
}
