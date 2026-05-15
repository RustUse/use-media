#[must_use]
pub fn extract_fill_values(input: &str) -> Vec<String> {
    crate::attribute::extract_attribute_values(input, "fill")
}

#[must_use]
pub fn extract_stroke_values(input: &str) -> Vec<String> {
    crate::attribute::extract_attribute_values(input, "stroke")
}
