pub mod graph {
    use std::collections::HashMap;
    use uuid::Uuid;

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
    pub struct WorldData {
        pub water: bool,
        pub ocean: bool,
        pub coast: bool,
        pub elevation: f32,
        pub river: f32,
        pub moisture: f32,
        pub biome: Biome,
    }

    #[derive(Debug, Clone)]
    pub struct Cell {
        pub id: Uuid,
        pub edges: Vec<(Uuid, Uuid)>,
        pub corners: Vec<Uuid>,
        pub center: (f32, f32),
        pub data: WorldData,
    }

    #[derive(Debug, Clone)]
    pub struct Edge {
        pub id: Uuid,
        pub corners: (Uuid, Uuid),
        pub cells: Vec<Uuid>,
        pub data: WorldData,
        pub down_corner: Uuid,
        pub noisey_midpoints: Vec<(f32, f32)>,
        pub river: f32,
    }

    #[derive(Debug, Clone)]
    pub struct Corner {
        pub id: Uuid,
        pub pos: (f32, f32),
        pub edges: Vec<(Uuid, Uuid)>,
        pub cells: Vec<Uuid>,
        pub data: WorldData,
        pub elevation: f32,
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        pub cells: HashMap<Uuid, Cell>,
        pub edges: HashMap<(Uuid, Uuid), Edge>,
        pub corners: HashMap<Uuid, Corner>,
    }

    impl Graph {
        // cell getters
        pub fn get_cell_edges(&self, cell_id: &Uuid) -> Vec<&Edge> {
            let cell = self.cells.get(cell_id).unwrap();
            return cell
                .edges
                .iter()
                .map(|id| self.edges.get(id).unwrap())
                .collect();
        }
        pub fn get_cell_corners(&self, cell_id: &Uuid) -> Vec<&Corner> {
            let edges = self.get_cell_edges(cell_id);
            return edges
                .iter()
                .fold(Vec::new(), |acc: Vec<&Corner>, edge: &&Edge| {
                    vec![acc, self.get_edge_corners(&edge.corners)].concat()
                });
        }
        pub fn get_cell_adjacent_cells(&self, cell_id: &Uuid) -> Vec<&Cell> {
            let t_cell = self.cells.get(cell_id).unwrap();
            let edges = self.get_cell_edges(cell_id);
            let all_cells: Vec<Vec<&Cell>> = edges
                .iter()
                .map(|edge| self.get_edge_cells(&edge.corners))
                .collect();
            let mut flattened_cells = all_cells.concat();
            flattened_cells.sort_by_key(|cell| cell.id);
            flattened_cells.dedup_by_key(|cell| cell.id);
            flattened_cells.retain(|cell| !t_cell.id.eq(&cell.id));
            return flattened_cells;
        }
        pub fn get_cell_center(&self, cell_id: &Uuid) -> (f32, f32) {
            let corners = self.get_cell_corners(cell_id);
            let l = corners.len() as f32;
            return corners.iter().fold((0.0, 0.0), |(x, y), corner| {
                ((x + (corner.pos.0 / l)), y + (corner.pos.1 / l))
            });
        }
        pub fn get_cell_elevation(&self, cell_id: &Uuid) -> f32 {
            let corners = self.get_cell_corners(cell_id);
            let l = corners.len() as f32;
            return corners
                .iter()
                .fold(0.0, |acc, corner| 
                    // TODO - move to navtive corner elevation
                    acc + (corner.data.elevation / l));
        }
        // edge getters
        pub fn get_edge_corners(&self, edge_id: &(Uuid, Uuid)) -> Vec<&Corner> {
            let edge = self.edges.get(edge_id).unwrap();
            let corner_1 = self.corners.get(&edge.corners.0).unwrap();
            let corner_2 = self.corners.get(&edge.corners.1).unwrap();
            return vec![corner_1, corner_2];
        }
        pub fn get_edge_cells(&self, edge_id: &(Uuid, Uuid)) -> Vec<&Cell> {
            let edge = self.edges.get(edge_id).unwrap();
            return edge
                .cells
                .iter()
                .map(|id| self.cells.get(id).unwrap())
                .collect();
        }
        pub fn get_edge_downhill_corner(&self, edge_id: &(Uuid, Uuid)) -> &Corner {
            let edge = self.edges.get(edge_id).unwrap();
            let corner_1 = self.corners.get(&edge.corners.0).unwrap();
            let corner_2 = self.corners.get(&edge.corners.1).unwrap();
            if corner_1.elevation < corner_2.elevation {
                return corner_1;
            } else {
                return corner_2;
            }
        }
        // corner getters
        pub fn get_corner_edges(&self, corner_id: &Uuid) -> Vec<&Edge> {
            let corner = self.corners.get(corner_id).unwrap();
            return corner
                .edges
                .iter()
                .map(|id| self.edges.get(id).unwrap())
                .collect();
        }
    }

    pub fn generate_base_diagram(i: usize, x_scale: f64, y_scale: f64) -> Graph {
        let rtn = initialise_voronoi(i, x_scale, y_scale, 5);
        let mut graph = Graph {
            cells: HashMap::new(),
            edges: HashMap::new(),
            corners: HashMap::new(),
        };
        for cell in rtn.cells() {
            let cell_id = Uuid::new_v4();
            let mut graph_cell = Cell {
                id: cell_id,
                edges: Vec::new(),
                corners: Vec::new(),
                center: (0.0, 0.0),
                data: WorldData {
                    ocean: false,
                    water: false,
                    coast: false,
                    elevation: 0.0,
                    river: 0.0,
                    moisture: 0.0,
                    biome: Biome::Bare,
                },
            };
            // Corner Handling
            let points = cell.points();
            let mut cell_corner_ids: Vec<Uuid> = Vec::new();
            for point in points {
                let pos = (point.x as f32, point.y as f32);
                let existing_corner = graph.corners.values().find(|corner| corner.pos.eq(&pos));
                if existing_corner.is_some() {
                    let corner_id = existing_corner.unwrap().id.clone();
                    cell_corner_ids.push(corner_id.clone());
                    let corner_mut = graph.corners.get_mut(&corner_id).unwrap();
                    corner_mut.cells.push(cell_id.clone());
                    drop(corner_mut);
                    graph_cell.corners.push(corner_id.clone());
                } else {
                    let corner = Corner {
                        id: Uuid::new_v4(),
                        cells: vec![cell_id.clone()],
                        edges: Vec::new(),
                        pos: pos,
                        data: WorldData {
                            ocean: false,
                            water: false,
                            coast: false,
                            elevation: 0.0,
                            river: 0.0,
                            moisture: 0.0,
                            biome: Biome::Bare,
                        },
                        elevation: 0.0,
                    };
                    cell_corner_ids.push(corner.id.clone());
                    graph_cell.corners.push(corner.id);
                    graph.corners.insert(corner.id, corner);
                }
            }
            // Edge Handling
            let final_point: Option<&Uuid> = cell_corner_ids.last();
            let mut prev_point: Option<&Uuid> = None;
            for id in &cell_corner_ids {
                let new_edge = if prev_point.is_none() {
                    // first case
                    (final_point.unwrap(), id)
                } else {
                    // all other cases
                    (id, prev_point.unwrap())
                };
                let new_edge_corners = (
                    graph.corners.get(&new_edge.0).unwrap(),
                    graph.corners.get(&new_edge.1).unwrap(),
                );
                let existing_edge = graph
                    .edges
                    .get(&(new_edge_corners.0.id, new_edge_corners.1.id))
                    .or(graph
                        .edges
                        .get(&(new_edge_corners.1.id, new_edge_corners.0.id)));
                if existing_edge.is_some() {
                    let key = existing_edge.unwrap().corners.clone();
                    graph_cell.edges.push(key.clone());
                    drop(existing_edge);
                    let existing_edge_mut = graph.edges.get_mut(&key).unwrap();
                    existing_edge_mut.cells.push(cell_id.clone());
                    drop(existing_edge_mut);
                } else {
                    drop(existing_edge);
                    let key = (new_edge_corners.0.id.clone(), new_edge_corners.1.id.clone());
                    drop(new_edge_corners);
                    let edge = Edge {
                        id: Uuid::new_v4(),
                        corners: key.clone(),
                        cells: vec![cell_id.clone()],
                        data: WorldData {
                            ocean: false,
                            water: false,
                            coast: false,
                            elevation: 0.0,
                            river: 0.0,
                            moisture: 0.0,
                            biome: Biome::Bare,
                        },
                        down_corner: key.0.clone(),
                        noisey_midpoints: Vec::new(),
                        river: 0.0,
                    };
                    graph_cell.edges.push(edge.corners.clone());
                    graph.edges.insert(
                        (new_edge_corners.0.id.clone(), new_edge_corners.1.id.clone()).clone(),
                        edge.clone(),
                    );

                    let corner_1 = graph.corners.get_mut(&key.0).unwrap();
                    corner_1.edges.push(key.clone());
                    drop(corner_1);

                    let corner_2 = graph.corners.get_mut(&key.1).unwrap();
                    corner_2.edges.push(key.clone());
                    drop(corner_2);
                }
                prev_point = Some(id);
            }

            graph.cells.insert(cell_id, graph_cell);
        }
        let cells_clone = graph.cells.clone();
        let cell_ids = cells_clone.keys();
        // Cell centers
        for id in cell_ids {
            let cell = graph.cells.get_mut(&id).unwrap();
            let center = cell.corners.iter().fold((0.0, 0.0), |acc, corner| {
                let pos = graph.corners.get(corner).unwrap();
                return (
                    acc.0 + (pos.pos.0 / cell.corners.len() as f32),
                    acc.1 + (pos.pos.1 / cell.corners.len() as f32),
                );
            });
            cell.center = center;

            drop(cell);
        }
        return graph;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gen_base_graph_test() {
            let graph = generate_base_diagram(500, 100.0, 200.0);
            for edge_id in graph.edges.keys() {
                let edge_cells = graph.get_edge_cells(&edge_id);
                assert!(edge_cells.len().eq(&2)||edge_cells.len().eq(&1));
            }
            for (cell_id, cell) in graph.cells.iter() {
                let adjacent_cells = graph.get_cell_adjacent_cells(&cell_id);
                assert!(adjacent_cells.len() <= cell.edges.len());
            }
        }
    }
}
