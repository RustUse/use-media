use use_svg::{
    SvgPath, SvgViewBox, build_svg_document, build_svg_icon, extract_attributes,
    extract_description, extract_fill_values, extract_height, extract_metadata, extract_path_data,
    extract_paths, extract_size, extract_stroke_values, extract_svg_root, extract_title,
    extract_transform_values, extract_view_box, extract_width, format_view_box, has_attribute,
    has_svg_root, is_svg, minify_svg_basic, normalize_svg, normalize_transform, parse_element,
    parse_view_box, strip_comments, strip_xml_declaration,
};

#[test]
fn detects_svg_roots() {
    let svg = r#"<?xml version="1.0"?>
<!-- comment -->
<svg width="24" height="24px" viewBox="0 0 24 24"></svg>"#;

    assert!(is_svg(svg));
    assert!(has_svg_root(svg));
    assert_eq!(
        extract_svg_root(svg),
        Some(r#"<svg width="24" height="24px" viewBox="0 0 24 24">"#)
    );
}

#[test]
fn rejects_non_svg_input() {
    assert!(!is_svg("<div></div>"));
    assert!(!has_svg_root("plain text"));
    assert_eq!(extract_svg_root("<html></html>"), None);
}

#[test]
fn extracts_width_height_and_size() {
    let svg = r#"<svg width="24px" height="100%"></svg>"#;
    let size = extract_size(svg);

    assert_eq!(extract_width(svg), Some("24px".to_string()));
    assert_eq!(extract_height(svg), Some("100%".to_string()));
    assert_eq!(size.width, Some(24.0));
    assert_eq!(size.height, Some(100.0));
    assert_eq!(size.width_unit.as_deref(), Some("px"));
    assert_eq!(size.height_unit.as_deref(), Some("%"));
}

#[test]
fn parses_and_formats_view_boxes() {
    let view_box = parse_view_box("0 0 24 24").unwrap();
    let negative = parse_view_box("-10 -10 20 20").unwrap();

    assert_eq!(view_box, SvgViewBox::new(0.0, 0.0, 24.0, 24.0));
    assert_eq!(format_view_box(view_box), "0 0 24 24");
    assert_eq!(negative.min_x, -10.0);
    assert_eq!(negative.min_y, -10.0);
    assert_eq!(format_view_box(negative), "-10 -10 20 20");
}

#[test]
fn extracts_view_box_and_paths() {
    let svg =
        r#"<svg viewBox="0 0 24 24"><path d="M0 0h24v24H0z"/><path d="M5 5h14v14H5z"/></svg>"#;

    assert_eq!(
        extract_view_box(svg),
        Some(SvgViewBox::new(0.0, 0.0, 24.0, 24.0))
    );
    assert_eq!(
        extract_paths(svg),
        vec![SvgPath::new("M0 0h24v24H0z"), SvgPath::new("M5 5h14v14H5z"),]
    );
    assert_eq!(
        extract_path_data(svg),
        vec!["M0 0h24v24H0z".to_string(), "M5 5h14v14H5z".to_string(),]
    );
}

#[test]
fn extracts_attributes_and_elements() {
    let element =
        r##"<path d="M0 0h10v10" fill="#fff" stroke="none" transform="translate(1, 2)"/>"##;
    let attributes = extract_attributes(element);
    let parsed = parse_element(element).unwrap();

    assert_eq!(parsed.name, "path");
    assert_eq!(attributes.len(), 4);
    assert!(has_attribute(element, "fill"));
    assert_eq!(attributes[0].name, "d");
    assert_eq!(attributes[1].value, "#fff");
}

#[test]
fn extracts_fill_stroke_and_transform_values() {
    let svg = r##"
        <svg>
            <path fill="#fff" stroke="#111" d="M0 0"/>
            <circle fill="none" stroke="red" transform=" translate(10, 20)   rotate(45) "/>
        </svg>
    "##;

    assert_eq!(
        extract_fill_values(svg),
        vec!["#fff".to_string(), "none".to_string()]
    );
    assert_eq!(
        extract_stroke_values(svg),
        vec!["#111".to_string(), "red".to_string()]
    );
    assert_eq!(
        extract_transform_values(svg)[0].value,
        " translate(10, 20)   rotate(45) "
    );
    assert_eq!(
        normalize_transform(" translate(10, 20)   rotate(45) "),
        "translate(10,20) rotate(45)"
    );
}

#[test]
fn extracts_metadata() {
    let svg = r#"<svg><title>Logo</title><desc>Primary brand mark</desc></svg>"#;
    let metadata = extract_metadata(svg);

    assert_eq!(extract_title(svg), Some("Logo".to_string()));
    assert_eq!(
        extract_description(svg),
        Some("Primary brand mark".to_string())
    );
    assert_eq!(metadata.title.as_deref(), Some("Logo"));
    assert_eq!(metadata.description.as_deref(), Some("Primary brand mark"));
}

#[test]
fn strips_xml_declarations_and_comments() {
    let svg = "<?xml version=\"1.0\"?>\n<svg></svg>";
    let commented = "<svg><!-- hidden --><path d=\"M0 0\"/></svg>";

    assert_eq!(strip_xml_declaration(svg), "<svg></svg>");
    assert_eq!(strip_comments(commented), "<svg><path d=\"M0 0\"/></svg>");
}

#[test]
fn normalizes_and_minifies_svg() {
    let svg = r#"
        <?xml version="1.0"?>
        <!-- comment -->
        <svg   viewBox = "0 0 24 24"   >
            <g>
                <path d = "M0 0" />
            </g>
        </svg>
    "#;

    assert_eq!(
        normalize_svg(svg),
        "<svg viewBox=\"0 0 24 24\">\n            <g>\n                <path d=\"M0 0\"/>\n            </g>\n        </svg>"
    );
    assert_eq!(
        minify_svg_basic(svg),
        "<svg viewBox=\"0 0 24 24\"><g><path d=\"M0 0\"/></g></svg>"
    );
}

#[test]
fn builds_svg_documents_and_icons() {
    let view_box = SvgViewBox::new(0.0, 0.0, 24.0, 24.0);
    let body = r#"<path d="M0 0h24v24H0z"/>"#;

    assert_eq!(
        build_svg_document(view_box, body),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M0 0h24v24H0z"/></svg>"#
    );
    assert_eq!(
        build_svg_icon(view_box, &[SvgPath::new("M0 0h24v24H0z")]),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M0 0h24v24H0z"/></svg>"#
    );
}

#[test]
fn handles_empty_and_malformed_input() {
    assert!(!is_svg(""));
    assert_eq!(extract_width(""), None);
    assert_eq!(extract_height(""), None);
    assert_eq!(extract_view_box("<svg viewBox=\"bad\"></svg>"), None);
    assert_eq!(parse_view_box("nope"), None);
    assert!(extract_path_data("<svg><path></svg>").is_empty());
    assert_eq!(strip_comments("<!-- broken"), "");
    assert!(minify_svg_basic("").is_empty());
}
