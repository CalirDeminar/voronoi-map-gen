pub mod biome {
    use std::collections::{HashMap, HashSet};

    use uuid::Uuid;

    use crate::{
        graph2::graph2::{Biome, Cell, Corner, Graph},
        helpers::helpers::corner_distance2,
        X_SCALE,
    };

    const MOISTURE_FALLOFF: f32 = 0.05;

    fn get_fresh_water_corners(graph: &Graph) -> Vec<&Corner> {
        let mut output: HashSet<Uuid> = HashSet::new();
        for (edge_id, edge) in &graph.edges {
            // river case
            if edge.river > 0.0 {
                output.insert(edge.corners.0.clone());
                output.insert(edge.corners.1.clone());
            } else {
                // lake border case
                let cells: Vec<&Cell> = edge
                    .cells
                    .iter()
                    .map(|c_id| graph.cells.get(c_id).unwrap())
                    .collect();
                if cells.iter().any(|cell| cell.water && !cell.ocean)
                    && cells.iter().any(|cell| !cell.water)
                {
                    output.insert(edge.corners.0.clone());
                    output.insert(edge.corners.1.clone());
                }
            }
        }

        return output
            .iter()
            .map(|id| graph.corners.get(id).unwrap())
            .collect();
    }

    fn assign_moisture(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let fresh_water_corners = get_fresh_water_corners(&graph_clone);
        // pre calculate all corner moisture levels
        let mut corner_moisture_cache: HashMap<Uuid, f32> = HashMap::new();
        for (corner_id, corner) in &graph_clone.corners {
            let shortest_distance = fresh_water_corners.iter().fold(X_SCALE as f32, |acc, c| {
                let d = corner_distance2(corner, c);
                if d < acc {
                    return d;
                } else {
                    return acc;
                }
            });
            corner_moisture_cache
                .insert(corner_id.clone(), MOISTURE_FALLOFF.powf(shortest_distance));
        }
        for (c_id, c) in &graph_clone.cells {
            if !c.water {
                let cell = graph.cells.get_mut(c_id).unwrap();
                let corners = graph_clone.get_cell_corners_ids(c_id);
                let cell_moisture_total = corners.iter().fold(0.0, |acc, corner| {
                    acc + corner_moisture_cache.get(corner).unwrap()
                });
                cell.moisture = cell_moisture_total / corners.len() as f32;
                drop(cell);
            }
        }
        return graph;
    }

    pub fn assign_biomes(graph: &mut Graph) -> &mut Graph {
        assign_moisture(graph);
        let graph_clone = graph.clone();
        let cell_clone = graph.cells.clone();
        for c_id in cell_clone.keys() {
            let cell = graph.cells.get_mut(c_id).unwrap();
            if cell.ocean {
                cell.biome = Biome::Ocean;
            } else if cell.water {
                // Water
                if graph_clone.get_cell_elevation(&c_id) < 0.1 {
                    cell.biome = Biome::Marsh;
                } else if graph_clone.get_cell_elevation(&c_id) > 0.8 {
                    cell.biome = Biome::Ice;
                } else {
                    cell.biome = Biome::Lake;
                }
            } else if cell.coast {
                cell.biome = Biome::Beach;
            } else if graph_clone.get_cell_elevation(&c_id) > 0.8 {
                // High Altitude
                if cell.moisture > 0.66 {
                    cell.biome = Biome::Snow;
                } else if cell.moisture > 0.33 {
                    cell.biome = Biome::Tundra;
                } else {
                    cell.biome = Biome::Bare;
                }
            } else if graph_clone.get_cell_elevation(&c_id) > 0.6 {
                // Middle-High Altitude
                if cell.moisture > 0.66 {
                    cell.biome = Biome::Taiga;
                } else if cell.moisture > 0.33 {
                    cell.biome = Biome::Shrubland;
                } else {
                    cell.biome = Biome::TemperateDesert;
                }
            } else if graph_clone.get_cell_elevation(&c_id) > 0.3 {
                // Middle-Low Altitude
                if cell.moisture > 0.66 {
                    cell.biome = Biome::TemperateRainForest;
                } else if cell.moisture > 0.33 {
                    cell.biome = Biome::TemperateForest;
                } else {
                    cell.biome = Biome::Grassland;
                }
            } else {
                // Low Altitude
                if cell.moisture > 0.66 {
                    cell.biome = Biome::TropicalRainForest;
                } else if cell.moisture > 0.33 {
                    cell.biome = Biome::TemperateForest;
                } else if cell.moisture > 0.15 {
                    cell.biome = Biome::Grassland;
                } else {
                    cell.biome = Biome::SubtropicalDesert;
                }
            }
            drop(cell);
        }
        return graph;
    }
}
