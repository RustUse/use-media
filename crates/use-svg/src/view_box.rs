#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SvgViewBox {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
}

impl SvgViewBox {
    #[must_use]
    pub fn new(min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        Self {
            min_x,
            min_y,
            width,
            height,
        }
    }
}

#[must_use]
pub fn parse_view_box(input: &str) -> Option<SvgViewBox> {
    let components: Vec<_> = input
        .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
        .filter(|component| !component.is_empty())
        .collect();

    if components.len() != 4 {
        return None;
    }

    let min_x = components[0].parse::<f64>().ok()?;
    let min_y = components[1].parse::<f64>().ok()?;
    let width = components[2].parse::<f64>().ok()?;
    let height = components[3].parse::<f64>().ok()?;

    if !min_x.is_finite()
        || !min_y.is_finite()
        || !width.is_finite()
        || !height.is_finite()
        || width < 0.0
        || height < 0.0
    {
        return None;
    }

    Some(SvgViewBox::new(min_x, min_y, width, height))
}

#[must_use]
pub fn format_view_box(view_box: SvgViewBox) -> String {
    format!(
        "{} {} {} {}",
        format_number(view_box.min_x),
        format_number(view_box.min_y),
        format_number(view_box.width),
        format_number(view_box.height)
    )
}

pub(crate) fn format_number(value: f64) -> String {
    if value.abs() < f64::EPSILON {
        return "0".to_string();
    }

    let rounded = value.round();
    if (value - rounded).abs() < 1.0e-12 {
        return format!("{}", rounded as i64);
    }

    let mut formatted = value.to_string();
    if formatted.contains('.') {
        while formatted.ends_with('0') {
            formatted.pop();
        }
        if formatted.ends_with('.') {
            formatted.pop();
        }
    }

    formatted
}
