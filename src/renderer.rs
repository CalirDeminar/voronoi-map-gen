pub mod renderer {
    use nannou::prelude::*;
    use nannou::App;

    use crate::graph::graph::Biome;
    use crate::graph::graph::Cell;
    use crate::graph::graph::Corner;
    use crate::graph::graph::Graph;
    use crate::X_SCALE;
    use crate::Y_SCALE;

    const FRESH_WATER: (f32, f32, f32) = (0.2, 0.33, 1.0);
    const SALT_WATER: (f32, f32, f32) = (0.2, 0.33, 1.0);
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

    pub fn render(app: &App, frame: &Frame, graph: &Graph, biome_debug: bool) {
        let draw = app.draw();
        draw.background().color(WHITE);

        for cell in graph.cells.values() {
            let points: Vec<&Corner> = cell
                .corners
                .iter()
                .map(|id| graph.corners.get(id).unwrap())
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
            let poly_points_clone = poly_points.clone();
            draw.polygon().points_colored(poly_points).z(1.0);

            if biome_debug {
                let points_len = poly_points_clone.len();
                let points_center = poly_points_clone.fold((0.0, 0.0), |acc, ((x, y), _x)| {
                    (
                        acc.0 + (x / points_len as f32),
                        acc.1 + (y / points_len as f32),
                    )
                });
                draw.text(cell_short(cell))
                    .xy(pt2(points_center.0, points_center.1))
                    .font_size(9)
                    .color(BLACK)
                    .z(5.0);
            }
        }
        for edge in graph.edges.values() {
            let is_coast = edge.data.coast;
            let is_river = edge.data.river > 0.0;

            if is_coast || is_river {
                let mut p1 = graph.corners.get(&edge.corners.0).unwrap();
                let mut p2 = graph.corners.get(&edge.corners.1).unwrap();
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

                if is_coast {
                    draw.line()
                        .start(pt_1)
                        .end(pt_2)
                        .weight(3.0)
                        .color(BLACK)
                        .caps_round()
                        .z(2.0);
                }
                if is_river {
                    draw.line()
                        .start(pt_1)
                        .end(pt_2)
                        .weight((edge.data.river as f32).sqrt() * 0.3)
                        .color(LinSrgb::from(FRESH_WATER))
                        .caps_round()
                        .z(2.0);
                }
            }
        }
        draw.to_frame(app, &frame).unwrap();
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
        };
    }
}
