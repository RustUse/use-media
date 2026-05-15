use crate::attribute::{SvgAttribute, extract_attributes, opening_tag_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgElement {
    pub name: String,
    pub attributes: Vec<SvgAttribute>,
}

impl SvgElement {
    #[must_use]
    pub fn new(name: impl Into<String>, attributes: Vec<SvgAttribute>) -> Self {
        Self {
            name: name.into(),
            attributes,
        }
    }
}

#[must_use]
pub fn parse_element(element: &str) -> Option<SvgElement> {
    Some(SvgElement::new(
        opening_tag_name(element)?,
        extract_attributes(element),
    ))
}
