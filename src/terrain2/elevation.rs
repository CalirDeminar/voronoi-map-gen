pub mod elevation2 {
    use std::collections::{HashSet, VecDeque};

    use uuid::Uuid;

    use crate::graph2::graph2::{Cell, Corner, Edge, Graph};

    fn get_coastal_corners(graph: &Graph) -> Vec<Uuid> {
        let coastal_cells: Vec<(&uuid::Uuid, &Cell)> = graph
            .cells
            .iter()
            .filter(|(_id, cell)| cell.coast)
            .collect();
        let coastal_cell_edges_nested: Vec<Vec<Uuid>> = coastal_cells
            .iter()
            .map(|(_id, cell)| cell.edges.clone())
            .collect();

        let mut coastal_cell_edge_ids: HashSet<Uuid> = HashSet::new();
        for id in coastal_cell_edges_nested.concat() {
            coastal_cell_edge_ids.insert(id);
        }
        let mut coastal_edges = coastal_cell_edge_ids.clone();
        coastal_edges.retain(|id| {
            let edge = graph.edges.get(id).unwrap();
            let cells: Vec<&Cell> = edge
                .cells
                .iter()
                .map(|cell_id| graph.cells.get(cell_id).unwrap())
                .collect();
            return cells.iter().any(|cell| cell.ocean) && cells.iter().any(|cell| cell.coast);
        });
        let nested_corners: Vec<Vec<Uuid>> = coastal_edges
            .iter()
            .map(|edge_id| {
                let edge = graph.edges.get(edge_id).unwrap();
                return vec![edge.corners.0.clone(), edge.corners.1.clone()];
            })
            .collect();

        return nested_corners.concat();
    }

    pub fn assign_land_elevation(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let coastal_corner_ids = get_coastal_corners(&graph_clone);
        let coastal_corners: Vec<(Uuid, f32)> =
            coastal_corner_ids.iter().map(|id| (*id, 0.0)).collect();

        let mut queue: VecDeque<(Uuid, f32)> = VecDeque::from(coastal_corners);
        let mut processed: HashSet<Uuid> = HashSet::new();
        for (id, _e) in &queue {
            processed.insert(id.clone());
        }
        while let Some((id, base_elevation)) = queue.pop_front() {
            processed.insert(id.clone());
            let corner_cells = graph_clone.get_corner_cells(&id);

            let is_open_water_corner = corner_cells.iter().all(|(_id, cell)| cell.water);

            let has_adjacent_lake = corner_cells
                .iter()
                .any(|(_id, cell)| cell.water && !cell.ocean);

            let elev_change = if has_adjacent_lake {
                0.0
            } else if is_open_water_corner {
                -0.5
            } else {
                1.0
            };
            let new_elev: f32 = base_elevation + elev_change;

            let adjacent_corners = graph.get_corner_adjacent_corners(&id);
            for (c_id, _c) in adjacent_corners {
                if !c_id.eq(&id) && !processed.contains(&c_id) {
                    queue.push_back((c_id.clone(), new_elev.clone()));
                    processed.insert(c_id.clone());
                }
            }
            let corner_mut = graph.corners.get_mut(&id).unwrap();
            corner_mut.elevation = new_elev;
            drop(corner_mut);
        }
        normalise_elevation(graph);
        return graph;
    }

    fn normalise_elevation(graph: &mut Graph) -> &mut Graph {
        let graph_clone = graph.clone();
        let max_elev = graph
            .corners
            .values()
            .map(|c| c.elevation)
            .fold(0.0, |acc, elev| if elev > acc { elev } else { acc });
        for c_id in graph_clone.corners.keys() {
            let corner_mut = graph.corners.get_mut(c_id).unwrap();
            corner_mut.elevation /= max_elev;
            drop(corner_mut);
        }
        return graph;
    }
}
