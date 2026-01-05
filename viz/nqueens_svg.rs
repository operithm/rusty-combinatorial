use svg::node::element::{Group, Rectangle, Text};
use svg::node::element::SVG;
use svg::Document;

/// Generates an SVG document visualizing an n-Queens solution.
pub fn draw_nqueens(solution: &[usize], square_size: usize) -> Document {
    let n = solution.len();
    let board_size = square_size * n;
    let mut doc = Document::new().set("viewBox", (0, 0, board_size, board_size));

    let mut group = Group::new();

    // Draw board squares
    for i in 0..n {
        for j in 0..n {
            let fill = if (i + j) % 2 == 0 { "#EEE" } else { "#666" };
            let rect = Rectangle::new()
                .set("x", j * square_size)
                .set("y", i * square_size)
                .set("width", square_size)
                .set("height", square_size)
                .set("fill", fill);
            group = group.add(rect);
        }
    }

    // Draw queens
    for (i, &col) in solution.iter().enumerate() {
        let x = col * square_size + square_size / 2;
        let y = i * square_size + square_size / 2;
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("fill", "red")
            .set("text-anchor", "middle")
            .set("dominant-baseline", "central")
            .set("font-size", square_size / 2)
            .add(svg::node::Text::new("♛"));
        group = group.add(text);
    }

    doc.add(group)
}
