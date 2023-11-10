pub mod graph2 {
    use rand::Rng;
    use std::collections::HashMap;
    use uuid::Uuid;
    use voronator::{delaunator, VoronoiDiagram};

    #[derive(Debug, Clone)]
    pub enum Biome {
        Ocean,
        Lake,
        Beach,
        Marsh,
        Ice,
        Snow,
        Tundra,
        Taiga,
        Bare,
        Shrubland,
        TemperateDesert,
        TemperateRainForest,
        TemperateForest,
        Grassland,
        TropicalRainForest,
        TropicalForest,
        SubtropicalDesert,
    }

    pub struct Corner {
        pub pos: (f32, f32),
        pub elevation: f32,
    }
    pub struct Edge {
        pub corners: (Uuid, Uuid),
        pub river: f32,
    }
    pub struct Cell {
        pub edges: Vec<Uuid>,
        pub water: bool,
        pub ocean: bool,
        pub moisture: f32,
        pub biome: Biome,
    }
    pub struct Graph {
        pub corners: HashMap<Uuid, Corner>,
        pub edges: HashMap<Uuid, Edge>,
        pub cells: HashMap<Uuid, Cell>,
    }

    impl Graph {
        // cells
        fn get_cell_edges(&self, id: &Uuid) -> Vec<&Edge> {
            let cell = &self.cells.get(id).unwrap();
            return cell
                .edges
                .iter()
                .map(|e_id| self.edges.get(e_id).unwrap())
                .collect();
        }
        fn get_cell_corners(&self, id: &Uuid) -> Vec<&Corner> {
            let cell = &self.cells.get(id).unwrap();
            let mut output: Vec<&Corner> = Vec::new();
            for edge in &cell.edges {
                let (c_1, c_2) = self.get_edge_corners(edge);
                output.push(c_1);
                output.push(c_2);
            }
            return output;
        }
        // edges
        fn get_edge_corners(&self, id: &Uuid) -> (&Corner, &Corner) {
            let edge = &self.edges.get(id).unwrap();
            let c_1 = self.corners.get(&edge.corners.0).unwrap();
            let c_2 = self.corners.get(&edge.corners.1).unwrap();
            return (c_1, c_2);
        }
        // corners
    }

    fn initialise(i: usize, x_scale: f64, y_scale: f64) -> Vec<(f64, f64)> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<(f64, f64)> = Vec::new();
        for _i in 0..i {
            output.push((rng.gen::<f64>() * x_scale, rng.gen::<f64>() * y_scale));
        }
        return output;
    }
}
