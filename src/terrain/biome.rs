pub mod biome {
    use crate::{
        graph::graph::{Biome, Corner, Graph},
        helpers::helpers::corner_distance,
        X_SCALE,
    };

    const MOISTURE_FALLOFF: f32 = 0.05;

    fn assign_moisture<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        let graph_clone = graph.clone();
        let fresh_water_corners: Vec<&Corner> = graph_clone
            .corners
            .values()
            .filter(|c| c.data.water && !c.data.ocean)
            .collect();
        for c_id in graph_clone.corners.keys() {
            let corner = graph.corners.get_mut(&c_id).unwrap();
            let shortest_distance = fresh_water_corners.iter().fold(X_SCALE as f32, |acc, c| {
                let d = corner_distance(corner, c);
                if d < acc {
                    return d;
                } else {
                    return acc;
                }
            });
            corner.data.moisture = MOISTURE_FALLOFF.powf(shortest_distance);
            drop(corner);
        }
        for c_id in graph_clone.cells.keys() {
            let cell = graph.cells.get_mut(c_id).unwrap();
            let corners = cell.corners.iter().map(|c| graph.corners.get(c).unwrap());
            let len = corners.len();
            let mean_moisture = corners.fold(0.0, |acc, c| acc + (c.data.moisture / len as f32));
            cell.data.moisture = mean_moisture;
            drop(cell);
        }
        return graph;
    }

    pub fn assign_biomes<'a>(graph: &'a mut Graph) -> &'a mut Graph {
        assign_moisture(graph);
        let cell_clone = graph.cells.clone();
        for c_id in cell_clone.keys() {
            let cell = graph.cells.get_mut(c_id).unwrap();
            if cell.data.ocean {
                cell.data.biome = Biome::Ocean;
            } else if cell.data.water {
                // Water
                if cell.data.elevation < 0.1 {
                    cell.data.biome = Biome::Marsh;
                } else if cell.data.elevation > 0.8 {
                    cell.data.biome = Biome::Ice;
                } else {
                    cell.data.biome = Biome::Lake;
                }
            } else if cell.data.coast {
                cell.data.biome = Biome::Beach;
            } else if cell.data.elevation > 0.8 {
                // High Altitude
                if cell.data.moisture > 0.66 {
                    cell.data.biome = Biome::Snow;
                } else if cell.data.moisture > 0.33 {
                    cell.data.biome = Biome::Tundra;
                } else {
                    cell.data.biome = Biome::Bare;
                }
            } else if cell.data.elevation > 0.6 {
                // Middle-High Altitude
                if cell.data.moisture > 0.66 {
                    cell.data.biome = Biome::Taiga;
                } else if cell.data.moisture > 0.33 {
                    cell.data.biome = Biome::Shrubland;
                } else {
                    cell.data.biome = Biome::TemperateDesert;
                }
            } else if cell.data.elevation > 0.3 {
                // Middle-Low Altitude
                if cell.data.moisture > 0.66 {
                    cell.data.biome = Biome::TemperateRainForest;
                } else if cell.data.moisture > 0.33 {
                    cell.data.biome = Biome::TemperateForest;
                } else {
                    cell.data.biome = Biome::Grassland;
                }
            } else {
                // Low Altitude
                if cell.data.moisture > 0.66 {
                    cell.data.biome = Biome::TropicalRainForest;
                } else if cell.data.moisture > 0.33 {
                    cell.data.biome = Biome::TemperateForest;
                } else {
                    cell.data.biome = Biome::Grassland;
                }
            }
            drop(cell);
        }
        return graph;
    }
}
