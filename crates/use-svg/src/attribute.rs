#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgAttribute {
    pub name: String,
    pub value: String,
}

impl SvgAttribute {
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[must_use]
pub fn extract_attributes(element: &str) -> Vec<SvgAttribute> {
    let tag = match opening_tag_slice(element) {
        Some(tag) => tag,
        None => return Vec::new(),
    };
    let bytes = tag.as_bytes();
    let mut index = 1;

    if bytes.get(index) == Some(&b'/') {
        return Vec::new();
    }

    while index < bytes.len()
        && !bytes[index].is_ascii_whitespace()
        && bytes[index] != b'>'
        && bytes[index] != b'/'
    {
        index += 1;
    }

    let mut attributes = Vec::new();

    while index < bytes.len() {
        while index < bytes.len() && (bytes[index].is_ascii_whitespace() || bytes[index] == b'/') {
            index += 1;
        }

        if index >= bytes.len() || bytes[index] == b'>' {
            break;
        }

        let name_start = index;
        while index < bytes.len()
            && !bytes[index].is_ascii_whitespace()
            && bytes[index] != b'='
            && bytes[index] != b'>'
            && bytes[index] != b'/'
        {
            index += 1;
        }

        let name = tag[name_start..index].trim();
        if name.is_empty() {
            if index < bytes.len() {
                index += 1;
            }
            continue;
        }

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }

        let value = if index < bytes.len() && bytes[index] == b'=' {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            parse_attribute_value(tag, &mut index)
        } else {
            String::new()
        };

        attributes.push(SvgAttribute::new(name, value));
    }

    attributes
}

#[must_use]
pub fn get_attribute(element: &str, name: &str) -> Option<String> {
    extract_attributes(element)
        .into_iter()
        .find(|attribute| attribute.name == name)
        .map(|attribute| attribute.value)
}

#[must_use]
pub fn has_attribute(element: &str, name: &str) -> bool {
    get_attribute(element, name).is_some()
}

pub(crate) fn extract_attribute_values(input: &str, name: &str) -> Vec<String> {
    let cleaned = crate::normalize::strip_comments(input);
    let mut values = Vec::new();
    let mut index = 0;

    while let Some(relative) = cleaned[index..].find('<') {
        let start = index + relative;
        let Some(end) = find_tag_end(cleaned.as_str(), start) else {
            break;
        };
        let next = cleaned[start + 1..].chars().next().unwrap_or('\0');

        if !matches!(next, '/' | '!' | '?') {
            let tag = &cleaned[start..end];
            if let Some(value) = get_attribute(tag, name) {
                values.push(value);
            }
        }

        index = end;
    }

    values
}

pub(crate) fn find_opening_tag(input: &str, tag_name: &str, from: usize) -> Option<(usize, usize)> {
    let mut index = from;

    while let Some(relative) = input[index..].find('<') {
        let start = index + relative;
        let next = input[start + 1..].chars().next().unwrap_or('\0');

        if matches!(next, '/' | '!' | '?') {
            index = start + 1;
            continue;
        }

        if tag_name_matches(input, start, tag_name) {
            let end = find_tag_end(input, start)?;
            return Some((start, end));
        }

        index = start + 1;
    }

    None
}

pub(crate) fn find_tag_end(input: &str, start: usize) -> Option<usize> {
    if !input[start..].starts_with('<') {
        return None;
    }

    let mut active_quote = None;

    for (offset, ch) in input[start + 1..].char_indices() {
        if let Some(quote) = active_quote {
            if ch == quote {
                active_quote = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' => active_quote = Some(ch),
            '>' => return Some(start + offset + 2),
            _ => {}
        }
    }

    None
}

pub(crate) fn opening_tag_name(element: &str) -> Option<String> {
    let tag = opening_tag_slice(element)?;
    let bytes = tag.as_bytes();
    let mut index = 1;

    if bytes.get(index) == Some(&b'/') {
        return None;
    }

    let start = index;
    while index < bytes.len()
        && !bytes[index].is_ascii_whitespace()
        && bytes[index] != b'>'
        && bytes[index] != b'/'
    {
        index += 1;
    }

    (start < index).then(|| tag[start..index].to_string())
}

pub(crate) fn tag_name_matches(input: &str, start: usize, tag_name: &str) -> bool {
    if !input[start..].starts_with('<') {
        return false;
    }

    let name_start = start + 1;
    let Some(candidate) = input[name_start..].get(..tag_name.len()) else {
        return false;
    };

    if !candidate.eq_ignore_ascii_case(tag_name) {
        return false;
    }

    match input[name_start + tag_name.len()..].chars().next() {
        Some(ch) => ch.is_ascii_whitespace() || matches!(ch, '>' | '/'),
        None => true,
    }
}

fn opening_tag_slice(element: &str) -> Option<&str> {
    let start = element.find('<')?;
    let end = find_tag_end(element, start)?;
    Some(&element[start..end])
}

fn parse_attribute_value(tag: &str, index: &mut usize) -> String {
    let bytes = tag.as_bytes();

    if *index >= bytes.len() {
        return String::new();
    }

    match bytes[*index] {
        b'"' | b'\'' => {
            let quote = bytes[*index];
            *index += 1;
            let start = *index;

            while *index < bytes.len() && bytes[*index] != quote {
                *index += 1;
            }

            let value = tag[start..*index].to_string();
            if *index < bytes.len() {
                *index += 1;
            }
            value
        }
        _ => {
            let start = *index;
            while *index < bytes.len()
                && !bytes[*index].is_ascii_whitespace()
                && bytes[*index] != b'>'
                && bytes[*index] != b'/'
            {
                *index += 1;
            }

            tag[start..*index].to_string()
        }
    }
}
