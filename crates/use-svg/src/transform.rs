#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgTransform {
    pub value: String,
}

impl SvgTransform {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[must_use]
pub fn extract_transform_values(input: &str) -> Vec<SvgTransform> {
    crate::attribute::extract_attribute_values(input, "transform")
        .into_iter()
        .map(SvgTransform::new)
        .collect()
}

#[must_use]
pub fn normalize_transform(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut pending_space = false;

    for ch in value.trim().chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }

        match ch {
            '(' => {
                if normalized.ends_with(' ') {
                    normalized.pop();
                }
                normalized.push('(');
                pending_space = false;
            },
            ')' => {
                if normalized.ends_with(' ') {
                    normalized.pop();
                }
                normalized.push(')');
                pending_space = true;
            },
            ',' => {
                if normalized.ends_with(' ') {
                    normalized.pop();
                }
                normalized.push(',');
                pending_space = false;
            },
            _ => {
                if pending_space
                    && !normalized.is_empty()
                    && !normalized.ends_with('(')
                    && !normalized.ends_with(',')
                {
                    normalized.push(' ');
                }
                normalized.push(ch);
                pending_space = false;
            },
        }
    }

    normalized.trim().to_string()
}
