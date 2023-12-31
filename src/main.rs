use graph2::graph2::Graph;
pub mod graph;
pub mod graph2;
pub mod helpers;
pub mod renderer;
pub mod renderer2;
pub mod terrain2;
pub mod voronoi;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use renderer2::renderer::render;
use terrain2::terrain2::full_terrain_gen;

// use crate::graph::graph::generate_base_diagram;

pub const X_SCALE: f64 = 1600.0;
pub const Y_SCALE: f64 = 800.0;

// const I: usize = 2000;
const I: usize = 2000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    graph: Graph,
    egui: Egui,
    has_logged_render: bool,
    log_render: bool,
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
    let base_graph = full_terrain_gen(I, X_SCALE, Y_SCALE);
    let egui = Egui::from_window(&window_a);
    // println!("Edge Cells: {}", base_graph.cells.values().filter(|cell| cell.data.ocean))
    Model {
        graph: base_graph,
        egui,
        has_logged_render: false,
        log_render: true,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let ctx = egui.begin_frame();
    if model.has_logged_render && model.log_render {
        model.log_render = false;
    } else if !model.has_logged_render {
        model.has_logged_render = true;
    }

    egui::Window::new("Settings").show(&ctx, |ui| {
        let regenerate = ui.button("Regenerate").clicked();
        if regenerate {
            let base_graph = full_terrain_gen(I, X_SCALE, Y_SCALE);
            model.graph = base_graph;
            model.log_render = true;
            model.has_logged_render = false;
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    render(app, &frame, &model.graph, true, model.log_render);
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
