use svg::node::element::{Circle, Group, Line, Text};
use svg::Document;

/// Draws a tree given its list of edges, arranging nodes in a circular layout.
///
/// # Arguments
/// * `edges` - A list of edges as (u, v) pairs
/// * `radius` - Distance from center to each node (for layout)
/// * `node_radius` - Radius of each circle in the drawing
///
/// # Returns
/// * An SVG `Document` visualizing the tree
///
/// # Example
/// ```
/// let edges = vec![(0,1), (1,2), (1,3)];
/// let svg = draw_tree(&edges, 100.0, 20.0);
/// svg::save("tree.svg", &svg).unwrap();
/// ```
pub fn draw_tree(edges: &[(usize, usize)], radius: f32, node_radius: f32) -> Document {
    use std::f32::consts::PI;

    let n = edges.iter().flat_map(|&(u, v)| vec![u, v]).max().unwrap_or(0) + 1;
    let center = (radius + 2.0 * node_radius) as i32;
    let size = (2.0 * (radius + 2.0 * node_radius)) as i32;
    let mut doc = Document::new().set("viewBox", (0, 0, size, size));
    let mut group = Group::new();

    // Compute node positions
    let mut positions = vec![(0f32, 0f32); n];
    for i in 0..n {
        let angle = 2.0 * PI * (i as f32) / (n as f32);
        let x = center as f32 + radius * angle.cos();
        let y = center as f32 + radius * angle.sin();
        positions[i] = (x, y);
    }

    // Draw edges
    for &(u, v) in edges {
        let (x1, y1) = positions[u];
        let (x2, y2) = positions[v];
        let line = Line::new()
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2)
            .set("stroke", "black");
        group = group.add(line);
    }

    // Draw nodes
    for (i, &(x, y)) in positions.iter().enumerate() {
        let circle = Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", node_radius)
            .set("fill", "lightblue")
            .set("stroke", "black");

        let label = Text::new()
            .set("x", x)
            .set("y", y + 4.0)
            .set("fill", "black")
            .set("text-anchor", "middle")
            .set("font-size", node_radius)
            .add(svg::node::Text::new(i.to_string()));

        group = group.add(circle).add(label);
    }

    doc.add(group)
}
