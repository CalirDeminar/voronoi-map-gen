pub mod graph2 {
    use std::collections::HashMap;
    use uuid::Uuid;
    use voronator::delaunator::Point;

    use crate::voronoi::voronoi::initialise_voronoi;

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

    #[derive(Debug, Clone)]
    pub struct Corner {
        // Graph Data
        pub pos: (f32, f32),
        pub edges: Vec<Uuid>,
        // Terrain Data
        pub elevation: f32,
    }
    #[derive(Debug, Clone)]
    pub struct Edge {
        // Graph Data
        pub corners: (Uuid, Uuid),
        pub cells: Vec<Uuid>,
        // Terrain Data
        pub river: f32,
    }
    #[derive(Debug, Clone)]
    pub struct Cell {
        // Graph Data
        pub edges: Vec<Uuid>,
        // Terrain Data
        pub water: bool,
        pub ocean: bool,
        pub moisture: f32,
        pub biome: Biome,
    }
    #[derive(Debug, Clone)]
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
        pub fn get_cell_corners_in_order(&self, id: &Uuid) -> Vec<&Corner> {
            let cell = &self.cells.get(id).unwrap();
            let mut working_edges = cell.edges.clone();
            // println!("{:?}", working_edges.len());
            if working_edges.len().eq(&0) {
                return vec![];
            }
            let mut output_corners: Vec<Uuid> = Vec::new();
            let mut last_edge_id = working_edges.remove(0);
            let starting_edge = &self.edges.get(&last_edge_id).unwrap();
            output_corners.push(starting_edge.corners.0);
            output_corners.push(starting_edge.corners.1);
            while working_edges.len() > 0 {
                let mut last_corner = output_corners.last().unwrap();
                let mut next_edge_id_option = working_edges
                    .iter()
                    .find(|e_id| self.edges_share_corner(e_id, last_corner));
                if next_edge_id_option.is_none() {
                    output_corners.reverse();
                    last_corner = output_corners.last().unwrap();
                    next_edge_id_option = working_edges
                        .iter()
                        .find(|e_id| self.edges_share_corner(e_id, last_corner));
                }
                let next_edge_id = next_edge_id_option.unwrap().clone();
                let edge = self.edges.get(&next_edge_id).unwrap();
                if !output_corners.contains(&edge.corners.0) {
                    output_corners.push(edge.corners.0);
                }

                if !output_corners.contains(&edge.corners.1) {
                    output_corners.push(edge.corners.1);
                }

                working_edges.retain(|e_id| !e_id.eq(&next_edge_id));
                last_edge_id = next_edge_id;
            }
            return output_corners
                .iter()
                .map(|c_id| self.corners.get(c_id).unwrap())
                .collect();
        }
        // edges
        fn get_edge_corners(&self, id: &Uuid) -> (&Corner, &Corner) {
            let edge = &self.edges.get(id).unwrap();
            let c_1 = self.corners.get(&edge.corners.0).unwrap();
            let c_2 = self.corners.get(&edge.corners.1).unwrap();
            return (c_1, c_2);
        }
        fn edges_share_same_corners(&self, id_1: &Uuid, index: usize, id_2: &Uuid) -> bool {
            let e_1 = self.edges.get(id_1).unwrap();
            let e_2 = self.edges.get(id_2).unwrap();
            if index.eq(&0) {
                return (e_1.corners.0.eq(&e_2.corners.0)) || (e_2.corners.0.eq(&e_2.corners.1));
            }
            if index.eq(&1) {
                return (e_1.corners.1.eq(&e_2.corners.0)) || (e_2.corners.1.eq(&e_2.corners.1));
            }
            return false;
        }
        fn edges_share_corner(&self, edge_id: &Uuid, corner_id: &Uuid) -> bool {
            let edge = self.edges.get(edge_id).unwrap();
            return edge.corners.0.eq(corner_id) || edge.corners.1.eq(corner_id);
        }
        // corners
    }

    pub fn generate_base_graph(i: usize, x_scale: f64, y_scale: f64) -> Graph {
        let voronoi = initialise_voronoi(i, x_scale, y_scale, 5);
        let mut graph = Graph {
            cells: HashMap::new(),
            edges: HashMap::new(),
            corners: HashMap::new(),
        };
        for cell in voronoi.cells() {
            let cell_id = Uuid::new_v4();
            let mut graph_cell = Cell {
                edges: Vec::new(),
                water: true,
                ocean: true,
                moisture: 0.0,
                biome: Biome::Ocean,
            };

            let first_point = cell.points().first().unwrap();
            let mut previous_point: Option<Uuid> = None;
            for point in cell.points() {
                // set up corner
                let point_search = graph.corners.iter().find(|(_id, corner)| {
                    corner.pos.0.eq(&(point.x as f32)) && corner.pos.1.eq(&(point.y as f32))
                });
                let corner_id = if point_search.is_some() {
                    point_search.unwrap().0.clone()
                } else {
                    let id = Uuid::new_v4();
                    let corner = Corner {
                        pos: (point.x as f32, point.y as f32),
                        edges: Vec::new(),
                        elevation: 0.0,
                    };
                    graph.corners.insert(id, corner);
                    id.clone()
                };
                // define edge
                if point.eq(&first_point) || previous_point.is_none() {
                    previous_point = Some(corner_id);
                    continue;
                }
                let prev_corner = graph.corners.get(&previous_point.unwrap()).unwrap();
                let edge_search = graph.edges.iter().find(|(_id, edge)| {
                    let c_1 = graph.corners.get(&edge.corners.0).unwrap();
                    let c_2 = graph.corners.get(&edge.corners.1).unwrap();
                    let c_1_prev_match =
                        c_1.pos.0.eq(&prev_corner.pos.0) && c_1.pos.1.eq(&prev_corner.pos.1);
                    let c_1_curr_match =
                        c_1.pos.0.eq(&(point.x as f32)) && c_1.pos.1.eq(&(point.y as f32));
                    let c_2_prev_match =
                        c_2.pos.0.eq(&prev_corner.pos.0) && c_2.pos.1.eq(&prev_corner.pos.1);
                    let c_2_curr_match =
                        c_2.pos.0.eq(&(point.x as f32)) && c_2.pos.1.eq(&(point.y as f32));
                    return (c_1_prev_match && c_2_curr_match)
                        || (c_1_curr_match && c_2_prev_match);
                });
                let edge_id = if edge_search.is_some() {
                    edge_search.unwrap().0.clone()
                } else {
                    let edge_id = Uuid::new_v4();
                    let edge = Edge {
                        corners: (corner_id, previous_point.unwrap().clone()),
                        cells: Vec::new(),
                        river: 0.0,
                    };
                    graph.edges.insert(edge_id, edge);
                    edge_id
                };
                graph_cell.edges.push(edge_id);
                previous_point = Some(corner_id);
            }
            graph.cells.insert(cell_id, graph_cell);
        }
        // fill quick reference vectors
        let graph_clone = graph.clone();
        for (cell_id, cell) in graph_clone.cells {
            for e_id in &cell.edges {
                let edge = graph.edges.get_mut(&e_id).unwrap();
                edge.cells.push(cell_id);
                drop(edge);
            }
        }
        for (edge_id, edge) in graph_clone.edges {
            let (c_1_id, c_2_id) = edge.corners;
            let corner_1 = graph.corners.get_mut(&c_1_id).unwrap();
            corner_1.edges.push(edge_id);
            drop(corner_1);
            let corner_2 = graph.corners.get_mut(&c_2_id).unwrap();
            corner_2.edges.push(edge_id);
            drop(corner_2);
        }
        return graph;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gen_base_graph_test() {
            generate_base_graph(500, 100.0, 200.0);
        }
        #[test]
        fn test_cell_corner_ordering() {
            let cell_id = Uuid::new_v4();
            let v_1_id = Uuid::new_v4();
            let v_2_id = Uuid::new_v4();
            let v_3_id = Uuid::new_v4();
            let v_4_id = Uuid::new_v4();
            let e_1_id = Uuid::new_v4();
            let e_2_id = Uuid::new_v4();
            let e_3_id = Uuid::new_v4();
            let e_4_id = Uuid::new_v4();
            let corner_1 = Corner {
                pos: (0.0, 0.0),
                edges: vec![e_1_id, e_4_id],
                elevation: 0.0,
            };
            let corner_2 = Corner {
                pos: (0.0, 1.0),
                edges: vec![e_1_id, e_2_id],
                elevation: 0.0,
            };
            let corner_3 = Corner {
                pos: (1.0, 1.0),
                edges: vec![e_2_id, e_3_id],
                elevation: 0.0,
            };
            let corner_4 = Corner {
                pos: (1.0, 0.0),
                edges: vec![e_3_id, e_4_id],
                elevation: 0.0,
            };
            let edge_1 = Edge {
                corners: (v_1_id, v_2_id),
                cells: vec![cell_id],
                river: 0.0,
            };
            let edge_2 = Edge {
                corners: (v_2_id, v_3_id),
                cells: vec![cell_id],
                river: 0.0,
            };
            let edge_3 = Edge {
                corners: (v_3_id, v_4_id),
                cells: vec![cell_id],
                river: 0.0,
            };
            let edge_4 = Edge {
                corners: (v_4_id, v_1_id),
                cells: vec![cell_id],
                river: 0.0,
            };
            let cell = Cell {
                water: false,
                ocean: false,
                moisture: 0.0,
                biome: Biome::Bare,
                edges: vec![e_3_id, e_1_id, e_2_id, e_4_id],
            };
            let mut cells: HashMap<Uuid, Cell> = HashMap::new();
            cells.insert(cell_id, cell);
            let mut edges: HashMap<Uuid, Edge> = HashMap::new();
            edges.insert(e_1_id, edge_1);
            edges.insert(e_2_id, edge_2);
            edges.insert(e_3_id, edge_3);
            edges.insert(e_4_id, edge_4);
            let mut corners: HashMap<Uuid, Corner> = HashMap::new();
            corners.insert(v_1_id, corner_1);
            corners.insert(v_2_id, corner_2);
            corners.insert(v_3_id, corner_3);
            corners.insert(v_4_id, corner_4);
            let graph = Graph {
                cells,
                edges,
                corners,
            };
            println!("{:#?}", graph.get_cell_corners_in_order(&cell_id));
        }
    }
}
