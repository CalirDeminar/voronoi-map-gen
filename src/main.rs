use graph::graph::{Corner, Edge, Graph};
pub mod graph;
pub mod terrain;
use nannou::color::encoding::Linear;
use nannou::color::Blend;
use nannou::{draw::mesh::vertex::Color, prelude::*};
use terrain::terrain::run_terrain_gen;

use crate::graph::graph::generate_base_diagram;

pub const X_SCALE: f64 = 1600.0;
pub const Y_SCALE: f64 = 800.0;

const I: usize = 2000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    graph: Graph,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size_pixels((X_SCALE * 1.1) as u32, (Y_SCALE * 1.1) as u32)
        .view(view)
        .build()
        .unwrap();
    let mut base_graph = generate_base_diagram(I, X_SCALE, Y_SCALE);
    run_terrain_gen(&mut base_graph);
    // println!("Edge Cells: {}", base_graph.cells.values().filter(|cell| cell.data.ocean))
    Model { graph: base_graph }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let max_elevation = model
        .graph
        .corners
        .values()
        .map(|c| c.data.elevation)
        .fold(0.0, |acc, i| if i > acc { i } else { acc });

    for cell in model.graph.cells.values() {
        let points: Vec<&Corner> = cell
            .corners
            .iter()
            .map(|id| model.graph.corners.get(id).unwrap())
            .collect();
        let mean_elevation = points
            .iter()
            .fold(0.0, |acc, e| acc + (e.data.elevation / points.len() as f32));
        // let mut prev: Option<&&Corner> = p.last();
        let poly_points = points.iter().map(|c| {
            let colour: LinSrgb<f32> = if cell.data.ocean {
                LinSrgb::new(0.0, 0.0, 1.0)
            } else if cell.data.water {
                LinSrgb::new(0.2, 0.33, 1.0)
            } else if cell.data.coast {
                LinSrgb::new(0.965, 0.843, 0.69)
            } else {
                LinSrgb::new(
                    mean_elevation / max_elevation,
                    0.2 + ((mean_elevation / max_elevation) * 0.8),
                    mean_elevation / max_elevation,
                )
            };
            return (
                (
                    c.pos.0 - (X_SCALE as f32 / 2.0),
                    c.pos.1 - (Y_SCALE as f32 / 2.0),
                ),
                colour,
            );
        });
        draw.polygon().points_colored(poly_points).z(1.0);

        let edges: Vec<&Edge> = cell
            .edges
            .iter()
            .map(|id| model.graph.edges.get(&id).unwrap())
            .collect();

        for edge in &edges {
            let p1 = model.graph.corners.get(&edge.corners.0).unwrap();
            let p2 = model.graph.corners.get(&edge.corners.1).unwrap();
            draw.line()
                .start(pt2(
                    p1.pos.0 - (X_SCALE as f32 / 2.0),
                    p1.pos.1 - (Y_SCALE as f32 / 2.0),
                ))
                .end(pt2(
                    p2.pos.0 - (X_SCALE as f32 / 2.0),
                    p2.pos.1 - (Y_SCALE as f32 / 2.0),
                ))
                .weight(if edge.data.coast { 3.0 } else { 1.0 })
                .color(BLACK)
                .z(2.0);
            // draw.text(&format!("{}", p1.data.elevation))
            //     .glyph_colors([BLACK])
            //     .font_size(12)
            //     .x(p1.pos.0 - (X_SCALE as f32 / 2.0))
            //     .y(p1.pos.1 - (Y_SCALE as f32 / 2.0))
            //     .z(3.0);
            // draw.text(&format!("{}", p2.data.elevation))
            //     .glyph_colors([BLACK])
            //     .font_size(12)
            //     .x(p2.pos.0 - (X_SCALE as f32 / 2.0))
            //     .y(p2.pos.1 - (Y_SCALE as f32 / 2.0))
            //     .z(3.0);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
