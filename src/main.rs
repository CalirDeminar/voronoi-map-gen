use graph2::graph2::Graph;
pub mod graph;
pub mod graph2;
pub mod helpers;
pub mod renderer;
pub mod renderer2;
pub mod terrain;
pub mod terrain2;
pub mod voronoi;
use graph2::graph2::generate_base_graph;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use renderer2::renderer::render;
use terrain2::terrain2::full_terrain_gen;
use terrain2::terrain2::run_terrain_gen;

// use crate::graph::graph::generate_base_diagram;

pub const X_SCALE: f64 = 1600.0;
pub const Y_SCALE: f64 = 800.0;

// const I: usize = 2000;
const I: usize = 4000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    graph: Graph,
    egui: Egui,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size_pixels((X_SCALE * 1.1) as u32, (Y_SCALE * 1.1) as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window_a = app.window(window).unwrap();
    let mut base_graph = full_terrain_gen(I, X_SCALE, Y_SCALE);
    let egui = Egui::from_window(&window_a);
    // println!("Edge Cells: {}", base_graph.cells.values().filter(|cell| cell.data.ocean))
    Model {
        graph: base_graph,
        egui,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        let regenerate = ui.button("Regenerate").clicked();
        if regenerate {
            let mut base_graph = full_terrain_gen(I, X_SCALE, Y_SCALE);
            model.graph = base_graph;
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    render(app, &frame, &model.graph, false);
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
