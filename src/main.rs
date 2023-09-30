use voronator::{delaunator::Point, VoronoiDiagram};
pub mod graph;
use nannou::prelude::*;

use crate::graph::graph::generate_base_diagram;

const X_SCALE: f64 = 800.0;
const Y_SCALE: f64 = 500.0;

const I: usize = 500;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    diagram: VoronoiDiagram<Point>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size_pixels((X_SCALE * 1.1) as u32, (Y_SCALE * 1.1) as u32)
        .view(view)
        .build()
        .unwrap();
    Model {
        diagram: generate_base_diagram(I, X_SCALE, Y_SCALE),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for cell in model.diagram.cells() {
        let mut prev: Option<&Point> = cell.points().last();
        for point in cell.points() {
            if prev.is_some() {
                draw.line()
                    .start(pt2(
                        (prev.unwrap().x - (X_SCALE / 2.0)) as f32,
                        (prev.unwrap().y - (Y_SCALE / 2.0)) as f32,
                    ))
                    .end(pt2(
                        (point.x - (X_SCALE / 2.0)) as f32,
                        (point.y - (Y_SCALE / 2.0)) as f32,
                    ))
                    .weight(1.0)
                    .color(BLACK);
            }
            prev = Some(&point);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
