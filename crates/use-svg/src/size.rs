use crate::document::{extract_height, extract_width};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SvgSize {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub width_unit: Option<String>,
    pub height_unit: Option<String>,
}

#[must_use]
pub fn extract_size(input: &str) -> SvgSize {
    let width = extract_width(input);
    let height = extract_height(input);

    let (width, width_unit) = width
        .as_deref()
        .and_then(parse_dimension)
        .map_or((None, None), |(value, unit)| (Some(value), unit));
    let (height, height_unit) = height
        .as_deref()
        .and_then(parse_dimension)
        .map_or((None, None), |(value, unit)| (Some(value), unit));

    SvgSize {
        width,
        height,
        width_unit,
        height_unit,
    }
}

pub(crate) fn parse_dimension(value: &str) -> Option<(f64, Option<String>)> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return None;
    }

    for split in (1..=trimmed.len()).rev() {
        if !trimmed.is_char_boundary(split) {
            continue;
        }

        let number = trimmed[..split].trim();
        let unit = trimmed[split..].trim();
        let Ok(parsed) = number.parse::<f64>() else {
            continue;
        };

        if !parsed.is_finite() {
            return None;
        }

        return Some((parsed, (!unit.is_empty()).then(|| unit.to_string())));
    }

    None
}
