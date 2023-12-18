pub mod renderer {
    use nannou::prelude::*;
    use nannou::App;
    use uuid::Uuid;

    use crate::graph2::graph2::Biome;
    use crate::graph2::graph2::Cell;
    use crate::graph2::graph2::Corner;
    use crate::graph2::graph2::Edge;
    use crate::graph2::graph2::Graph;
    use crate::helpers::helpers::create_benchmarker;
    use crate::X_SCALE;
    use crate::Y_SCALE;

    const FRESH_WATER: (f32, f32, f32) = (0.2, 0.33, 1.0);
    const SALT_WATER: (f32, f32, f32) = (0.15, 0.25, 0.75);
    // const EDGE: (f32, f32, f32) = (0.0, 0.0, 0.0);

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

    fn get_cell_positions_with_midpoints(graph: &Graph, cell_id: &Uuid) -> Vec<(f32, f32)> {
        let cell = graph.cells.get(cell_id).unwrap();
        let mut output: Vec<(f32, f32)> = Vec::new();

        let mut working_edges = cell.edges.clone();
        let last_edge_id = working_edges.remove(0);
        let starting_edge = graph.edges.get(&last_edge_id).unwrap();
        for pos in &starting_edge.corner_midpoints {
            output.push(pos.clone());
        }
        while working_edges.len() > 0 {
            let mut last_position = output.last().unwrap();
            let mut next_edge_id_option = working_edges
                .iter()
                .find(|e_id| graph.edge_shares_pos_at(e_id, last_position));
            if next_edge_id_option.is_none() {
                output.reverse();
                last_position = output.last().unwrap();
                next_edge_id_option = working_edges
                    .iter()
                    .find(|e_id| graph.edge_shares_pos_at(e_id, last_position));
            }
            let next_edge_id = next_edge_id_option.unwrap().clone();
            let edge = graph.edges.get(&next_edge_id).unwrap();
            if !output.contains(edge.corner_midpoints.last().unwrap()) {
                for pos in &edge.corner_midpoints {
                    output.push(pos.clone());
                }
            }

            if !output.contains(edge.corner_midpoints.first().unwrap()) {
                let mut reversed_points = edge.corner_midpoints.clone();
                reversed_points.reverse();
                for pos in &reversed_points {
                    output.push(pos.clone());
                }
            }
            working_edges.retain(|e_id| !e_id.eq(&next_edge_id));
        }
        return output;
    }

    pub fn render(
        app: &App,
        frame: &Frame,
        graph: &Graph,
        biome_debug: bool,
        log_render_time: bool,
    ) {
        let draw = app.draw();
        draw.background().color(WHITE);
        let render_time = create_benchmarker(String::from("Render"));
        for (_i, cell_id) in graph.cells.keys().enumerate() {
            let cell = graph.cells.get(cell_id).unwrap();

            let points = get_cell_positions_with_midpoints(graph, cell_id);

            let poly_points_2 = points.iter().map(|c| {
                let colour: LinSrgb<f32> = LinSrgb::from(cell_colour(cell));

                return (
                    (c.0 - (X_SCALE as f32 / 2.0), c.1 - (Y_SCALE as f32 / 2.0)),
                    colour,
                );
            });

            let poly_points_clone = poly_points_2.clone();

            draw.polygon().points_colored(poly_points_2).z(1.0);

            if biome_debug {
                let points_len = poly_points_clone.len();
                let points_center = poly_points_clone.fold((0.0, 0.0), |acc, ((x, y), _x)| {
                    (
                        acc.0 + (x / points_len as f32),
                        acc.1 + (y / points_len as f32),
                    )
                });
                // draw.text(cell_short(cell))
                //     .xy(pt2(points_center.0, points_center.1))
                //     .font_size(9)
                //     .color(BLACK)
                //     .z(5.0);
                draw.text(&format!(
                    "{}",
                    cell_short(cell),
                    // graph.get_cell_elevation(cell_id),
                    // if cell.water { "w" } else { "" },
                    // if cell.ocean { "o" } else { "" }
                ))
                .xy(pt2(points_center.0, points_center.1))
                .font_size(7)
                .color(BLACK)
                .z(5.0);
            }
        }
        for (edge_id, edge) in &graph.edges {
            let is_coast = graph.edge_is_coastal(edge_id);
            let is_river = edge.river > 0.0;

            if is_coast || is_river {
                let mut p1 = graph.corners.get(&edge.corners.0).unwrap();
                let mut p2 = graph.corners.get(&edge.corners.1).unwrap();
                if p2.elevation < p1.elevation {
                    let t = p1;
                    p1 = p2;
                    p2 = t;
                }

                let mut midpoints = edge.corner_midpoints.clone();
                let mut last_point = midpoints.remove(0);
                for point in midpoints {
                    let pt_1 = pt2(
                        point.0 - (X_SCALE as f32 / 2.0),
                        point.1 - (Y_SCALE as f32 / 2.0),
                    );
                    let pt_2 = pt2(
                        last_point.0 - (X_SCALE as f32 / 2.0),
                        last_point.1 - (Y_SCALE as f32 / 2.0),
                    );
                    if is_river {
                        draw.line()
                            .start(pt_1)
                            .end(pt_2)
                            .weight((edge.river as f32).sqrt() * 0.3)
                            .color(LinSrgb::from(FRESH_WATER))
                            .caps_round()
                            .z(2.0);
                    } else if is_coast {
                        draw.line()
                            .start(pt_1)
                            .end(pt_2)
                            .weight(3.0)
                            .color(BLACK)
                            .caps_round()
                            .z(2.0);
                    }
                    last_point = point;
                }
            }
        }
        draw.to_frame(app, &frame).unwrap();
        if log_render_time {
            render_time();
        }
    }

    fn cell_colour(cell: &Cell) -> (f32, f32, f32) {
        return match cell.biome {
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
        };
    }
    fn cell_short(cell: &Cell) -> &str {
        return match cell.biome {
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
        };
    }
}
