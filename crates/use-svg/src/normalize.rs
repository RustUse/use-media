use crate::document::strip_xml_declaration;

#[must_use]
pub fn strip_comments(input: &str) -> String {
    let mut cleaned = String::with_capacity(input.len());
    let mut index = 0;

    while let Some(start) = input[index..].find("<!--") {
        let absolute_start = index + start;
        cleaned.push_str(&input[index..absolute_start]);

        let Some(end) = input[absolute_start + 4..].find("-->") else {
            return cleaned;
        };

        index = absolute_start + 4 + end + 3;
    }

    cleaned.push_str(&input[index..]);
    cleaned
}

#[must_use]
pub fn normalize_svg(input: &str) -> String {
    let without_comments = strip_comments(strip_xml_declaration(input));
    let trimmed = without_comments.trim();

    if trimmed.is_empty() {
        return String::new();
    }

    normalize_tag_whitespace(trimmed)
}

#[must_use]
pub fn minify_svg_basic(input: &str) -> String {
    let normalized = normalize_svg(input);

    if normalized.is_empty() {
        return normalized;
    }

    remove_intertag_whitespace(&normalized)
}

fn normalize_tag_whitespace(input: &str) -> String {
    let mut normalized = String::with_capacity(input.len());
    let mut inside_tag = false;
    let mut active_quote = None;
    let mut pending_space = false;
    let mut suppress_space = false;

    for ch in input.chars() {
        if let Some(quote) = active_quote {
            normalized.push(ch);
            if ch == quote {
                active_quote = None;
            }
            continue;
        }

        if inside_tag {
            match ch {
                '<' => normalized.push(ch),
                '"' | '\'' => {
                    if pending_space
                        && !suppress_space
                        && !normalized.ends_with('<')
                        && !normalized.ends_with('=')
                    {
                        normalized.push(' ');
                    }
                    pending_space = false;
                    suppress_space = false;
                    normalized.push(ch);
                    active_quote = Some(ch);
                },
                '>' => {
                    if normalized.ends_with(' ') {
                        normalized.pop();
                    }
                    normalized.push('>');
                    inside_tag = false;
                    pending_space = false;
                    suppress_space = false;
                },
                '=' => {
                    if normalized.ends_with(' ') {
                        normalized.pop();
                    }
                    normalized.push('=');
                    pending_space = false;
                    suppress_space = true;
                },
                '/' => {
                    if normalized.ends_with(' ') {
                        normalized.pop();
                    }
                    normalized.push('/');
                    pending_space = false;
                    suppress_space = false;
                },
                _ if ch.is_whitespace() => pending_space = true,
                _ => {
                    if pending_space
                        && !suppress_space
                        && !normalized.ends_with('<')
                        && !normalized.ends_with('/')
                        && !normalized.ends_with('=')
                    {
                        normalized.push(' ');
                    }
                    normalized.push(ch);
                    pending_space = false;
                    suppress_space = false;
                },
            }
        } else if ch == '<' {
            inside_tag = true;
            pending_space = false;
            suppress_space = false;
            normalized.push(ch);
        } else {
            normalized.push(ch);
        }
    }

    normalized
}

fn remove_intertag_whitespace(input: &str) -> String {
    let chars: Vec<_> = input.chars().collect();
    let mut minified = String::with_capacity(input.len());
    let mut index = 0;

    while index < chars.len() {
        if chars[index].is_whitespace() {
            let mut next = index;
            while next < chars.len() && chars[next].is_whitespace() {
                next += 1;
            }

            let previous = minified.chars().last();
            let following = chars.get(next).copied();

            if previous == Some('>') && following == Some('<') {
                index = next;
                continue;
            }

            for ch in &chars[index..next] {
                minified.push(*ch);
            }
            index = next;
            continue;
        }

        minified.push(chars[index]);
        index += 1;
    }

    minified.trim().to_string()
}
