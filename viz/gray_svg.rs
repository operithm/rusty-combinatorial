use svg::node::element::{Group, Line, Text};
use svg::Document;

/// Draws a vertical walk of Gray code binary strings as an SVG document.
///
/// # Arguments
/// * `codes` - A list of Gray code strings (e.g. "000", "001", ...)\n/// * `font_size` - Font size in pixels\n/// * `line_spacing` - Vertical spacing between rows
///
/// # Returns
/// * An SVG document displaying the Gray code sequence
///
/// # Example
/// ```
/// let codes = vec![
///     "000".to_string(), "001".to_string(), "011".to_string(), "010".to_string(),
///     "110".to_string(), "111".to_string(), "101".to_string(), "100".to_string()
/// ];
/// let svg = draw_gray_code(&codes, 16, 24);
/// svg::save("gray.svg", &svg).unwrap();
/// ```
pub fn draw_gray_code(codes: &[String], font_size: usize, line_spacing: usize) -> Document {
    let width = codes.iter().map(|s| s.len()).max().unwrap_or(0) * font_size;
    let height = codes.len() * line_spacing;

    let mut doc = Document::new().set("viewBox", (0, 0, width, height));
    let mut group = Group::new();

    for (i, code) in codes.iter().enumerate() {
        let x = 0;
        let y = i * line_spacing + font_size;
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("font-size", font_size)
            .set("font-family", "monospace")
            .set("fill", "black")
            .add(svg::node::Text::new(code));
        group = group.add(text);
    }

    doc.add(group)
}
