use graph::graph::{Biome, Cell, Corner, Edge, Graph};
pub mod graph;
pub mod helpers;
pub mod terrain;
use nannou::prelude::*;
use terrain::terrain::run_terrain_gen;

use crate::graph::graph::generate_base_diagram;

pub const X_SCALE: f64 = 1600.0;
pub const Y_SCALE: f64 = 800.0;

const FRESH_WATER: (f32, f32, f32) = (0.2, 0.33, 1.0);
const SALT_WATER: (f32, f32, f32) = (0.2, 0.33, 1.0);
const EDGE: (f32, f32, f32) = (0.0, 0.0, 0.0);

const BEACH: (f32, f32, f32) = (0.62, 0.56, 0.46);
const ICE: (f32, f32, f32) = (0.6, 1.0, 1.0);
const MARSH: (f32, f32, f32) = (0.18, 0.4, 0.4);
const SNOW: (f32, f32, f32) = (1.0, 1.0, 1.0);
const TUNDRA: (f32, f32, f32) = (0.73, 0.73, 0.7);
const BARE: (f32, f32, f32) = (0.53, 0.53, 0.53);
const TAIGA: (f32, f32, f32) = (0.6, 0.66, 0.47);
const SHRUBLAND: (f32, f32, f32) = (0.53, 0.6, 0.47);
const TEMPERATEDESERT: (f32, f32, f32) = (0.79, 0.82, 0.61);
const TEMPERATERAINFOREST: (f32, f32, f32) = (0.26, 0.53, 0.33);
const TEMPERATEFOREST: (f32, f32, f32) = (0.4, 0.59, 0.34);
const GRASSLAND: (f32, f32, f32) = (0.53, 0.66, 0.34);
const SUBTROPICALDESERT: (f32, f32, f32) = (0.79, 0.73, 0.54);
const TROPICALRAINFOREST: (f32, f32, f32) = (0.2, 0.47, 0.33);
const TROPICALFOREST: (f32, f32, f32) = (0.33, 0.59, 0.26);

const I: usize = 5000;

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
        // let mut prev: Option<&&Corner> = p.last();
        let poly_points = points.iter().map(|c| {
            let colour: LinSrgb<f32> = LinSrgb::from(cell_colour(cell));

            return (
                (
                    c.pos.0 - (X_SCALE as f32 / 2.0),
                    c.pos.1 - (Y_SCALE as f32 / 2.0),
                ),
                colour,
            );
        });
        let l = poly_points.len();
        let points_center = poly_points.clone().fold((0.0, 0.0), |acc, ((x, y), _x)| {
            (acc.0 + (x / l as f32), acc.1 + (y / l as f32))
        });
        draw.polygon().points_colored(poly_points).z(1.0);

        draw.text(cell_short(cell))
            .xy(pt2(points_center.0, points_center.1))
            .font_size(9)
            .color(BLACK)
            .z(5.0);

        let edges: Vec<&Edge> = cell
            .edges
            .iter()
            .map(|id| model.graph.edges.get(&id).unwrap())
            .collect();

        for edge in &edges {
            let mut p1 = model.graph.corners.get(&edge.corners.0).unwrap();
            let mut p2 = model.graph.corners.get(&edge.corners.1).unwrap();
            if !edge.corners.1.eq(&edge.down_corner) {
                let t = p1;
                p1 = p2;
                p2 = t;
            }

            let pt_1 = pt2(
                p1.pos.0 - (X_SCALE as f32 / 2.0),
                p1.pos.1 - (Y_SCALE as f32 / 2.0),
            );
            let pt_2 = pt2(
                p2.pos.0 - (X_SCALE as f32 / 2.0),
                p2.pos.1 - (Y_SCALE as f32 / 2.0),
            );

            let weight = if edge.data.coast {
                3.0
            } else if edge.data.river > 0 {
                (edge.data.river as f32).sqrt() * 2.0
            } else {
                1.0
            };

            let has_ocean_cell = edge
                .cells
                .iter()
                .any(|c_id| model.graph.cells.get(c_id).unwrap().data.ocean);

            draw.line()
                .start(pt_1)
                .end(pt_2)
                .weight(weight)
                .color(LinSrgb::from(edge_colour(edge.data.water, has_ocean_cell)))
                .z(2.0);

            // if !edge.data.ocean {
            //     draw.arrow()
            //         .start(pt_1)
            //         .end(pt_2)
            //         .weight(weight)
            //         .head_length(6.0)
            //         .head_width(3.0)
            //         .color(LinSrgb::from(edge_colour(edge.data.water, has_ocean_cell)))
            //         .z(2.0);
            // }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn edge_colour(has_water: bool, has_ocean: bool) -> (f32, f32, f32) {
    if has_water && !has_ocean {
        return FRESH_WATER;
    } else {
        return EDGE;
    }
}

fn cell_colour(cell: &Cell) -> (f32, f32, f32) {
    return match cell.data.biome {
        Biome::Ocean => SALT_WATER,
        Biome::Lake => FRESH_WATER,
        Biome::Beach => BEACH,
        Biome::Ice => ICE,
        Biome::Marsh => MARSH,
        Biome::Snow => SNOW,
        Biome::Tundra => TUNDRA,
        Biome::Bare => BARE,
        Biome::Taiga => TAIGA,
        Biome::Shrubland => SHRUBLAND,
        Biome::TemperateDesert => TEMPERATEDESERT,
        Biome::TemperateForest => TEMPERATEFOREST,
        Biome::TemperateRainForest => TEMPERATERAINFOREST,
        Biome::Grassland => GRASSLAND,
        Biome::SubtropicalDesert => SUBTROPICALDESERT,
        Biome::TropicalRainForest => TROPICALRAINFOREST,
        Biome::TropicalForest => TROPICALFOREST,
        _ => (1.0, 1.0, 1.0),
    };
}

fn cell_short(cell: &Cell) -> &str {
    return match cell.data.biome {
        Biome::Ocean => "",
        Biome::Lake => "LKE",
        Biome::Beach => "BCH",
        Biome::Ice => "ICE",
        Biome::Marsh => "MSH",
        Biome::Snow => "SNW",
        Biome::Tundra => "TUND",
        Biome::Bare => "BARE",
        Biome::Taiga => "TAIG",
        Biome::Shrubland => "SHRUB",
        Biome::TemperateDesert => "TDST",
        Biome::TemperateForest => "TFST",
        Biome::TemperateRainForest => "TRFST",
        Biome::Grassland => "GSLD",
        Biome::SubtropicalDesert => "STDST",
        Biome::TropicalRainForest => "TRFST",
        Biome::TropicalForest => "TRFST",
        _ => "???",
    };
}
