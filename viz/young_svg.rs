use svg::node::element::{Group, Rectangle, Text};
use svg::Document;

/// Draws a standard Young tableau as an SVG document.
///
/// # Arguments
/// * `tableau` - A 2D matrix representing the tableau (strictly increasing rows/columns)
/// * `cell_size` - Size in pixels of each square
///
/// # Returns
/// * An SVG document rendering the tableau as a grid with numbered cells
///
/// # Example
/// ```
/// let t = vec![vec![1, 2], vec![3]];
/// let svg = draw_young_tableau(&t, 50);
/// svg::save("young.svg", &svg).unwrap();
/// ```
pub fn draw_young_tableau(tableau: &[Vec<usize>], cell_size: usize) -> Document {
    let rows = tableau.len();
    let cols = tableau.iter().map(|r| r.len()).max().unwrap_or(0);
    let width = cols * cell_size;
    let height = rows * cell_size;

    let mut doc = Document::new().set("viewBox", (0, 0, width, height));
    let mut group = Group::new();

    for (i, row) in tableau.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            let x = j * cell_size;
            let y = i * cell_size;
            let rect = Rectangle::new()
                .set("x", x)
                .set("y", y)
                .set("width", cell_size)
                .set("height", cell_size)
                .set("fill", "white")
                .set("stroke", "black");
            let text = Text::new()
                .set("x", x + cell_size / 2)
                .set("y", y + cell_size / 2)
                .set("fill", "black")
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("font-size", cell_size / 2)
                .add(svg::node::Text::new(val.to_string()));

            group = group.add(rect).add(text);
        }
    }

    doc.add(group)
}
