use graph::graph::{Corner, Graph};
pub mod graph;
pub mod terrain;
use nannou::{draw::background, prelude::*, text::font};
use terrain::terrain::run_terrain_gen;

use crate::graph::graph::generate_base_diagram;

pub const X_SCALE: f64 = 1600.0;
pub const Y_SCALE: f64 = 800.0;

const I: usize = 1000;

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
    for cell in model.graph.cells.values() {
        let points: Vec<&Corner> = cell
            .corners
            .iter()
            .map(|id| model.graph.corners.get(id).unwrap())
            .collect();
        let p = points.clone();
        let mut prev: Option<&&Corner> = p.last();
        let poly_points = points.iter().map(|c| {
            let mut colour = if cell.data.ocean {
                DARKBLUE
            } else if cell.data.water {
                BLUE
            } else if cell.data.coast {
                WHEAT
            } else {
                WHITE
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
        // draw edges
        for point in &points {
            if prev.is_some() {
                draw.line()
                    .start(pt2(
                        prev.unwrap().pos.0 - (X_SCALE as f32 / 2.0),
                        prev.unwrap().pos.1 - (Y_SCALE as f32 / 2.0),
                    ))
                    .end(pt2(
                        point.pos.0 - (X_SCALE as f32 / 2.0),
                        point.pos.1 - (Y_SCALE as f32 / 2.0),
                    ))
                    .weight(if point.data.coast && prev.unwrap().data.coast {
                        3.0
                    } else {
                        1.0
                    })
                    .color(BLACK)
                    .z(2.0);
            }
            draw.text(&format!("{}", point.data.elevation))
                .glyph_colors([BLACK])
                .font_size(12)
                .x(point.pos.0 - (X_SCALE as f32 / 2.0))
                .y(point.pos.1 - (Y_SCALE as f32 / 2.0))
                .z(3.0);
            prev = Some(&point);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
